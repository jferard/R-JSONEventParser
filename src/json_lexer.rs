/*
 * R-JSON Event Parser - a Rust JSON event based parser.
 *
 *    Copyright (C) 2021 J. Férard <https://github.com/jferard>
 *
 * This file is part of JSON Event Parser.
 *
 * R-JSON Event Parser is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * R-JSON Event Parser is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
#![allow(unused_variables)]

use std::io::Read;
use std::str;

use crate::byte_source::ByteSource;
use crate::json_lexer::LexerToken::{BeginFile, EndFile};

#[derive(Debug, PartialEq)]
pub enum LexerToken {
    BeginObject,
    EndObject,
    BeginArray,
    EndArray,
    NameSeparator,
    ValueSeparator,
    BooleanValue(bool),
    NullValue,
    String(String),
    IntValue(String),
    FloatValue(String),
    BeginFile,
    EndFile,
}

#[derive(Debug, PartialEq)]
pub struct JSONLexError {
    pub msg: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq)]
pub struct ConsumeError {
    pub msg: String,
    pub line: usize,
    pub column: usize,
}

pub trait JSONLexConsumer {
    fn consume(&mut self, token: Result<LexerToken, JSONLexError>, line: usize, column: usize) -> Result<(), ConsumeError>;
}

enum LexerState {
    None,
    Expect(LexerToken),
    Number,
    String,
}

enum LexerNumberSubState {
    None,
    NegNumberStart,
    ZeroNumberStart,
    OtherNumber,
    NumberFracStart,
    NumberFrac,
    NumberFracExpStart,
    NumberFracExp,
    NumberFracExpMinusStart,
    NumberFracExpMinus,
}

#[derive(Debug)]
enum LexerStringSubState {
    None,
    Escape,
    Unicode,
}

const REPLACEMENT_CHARACTER: char = '\u{fffd}';

pub struct JSONLexer<R: Read> {
    byte_source: ByteSource<R>,
    line: usize,
    column: usize,
    ignore_unicode_errs: bool,
}

impl<R: Read> JSONLexer<R> {
    pub fn new(byte_source: ByteSource<R>, ignore_unicode_errs: bool) -> Self {
        JSONLexer {
            byte_source,
            line: 0,
            column: 0,
            ignore_unicode_errs,
        }
    }

    pub fn lex<C: JSONLexConsumer>(&mut self, consumer: &mut C) -> Result<(), ConsumeError> {
        macro_rules! lex_error {
            ($($arg:tt)*) => {{
                Err(JSONLexError {
                    msg: format!($($arg)*),
                    line: self.line,
                    column: self.column,
                })
            }}
        }

        macro_rules! consume_lex_error {
            ($($arg:tt)*) => {{
                consumer.consume(lex_error!($($arg)*), self.line, self.column)?;
            }}
        }

        macro_rules! end_of_number {
            ($buf:ident, $number_sub_state: ident, $state: ident) => {{
                $buf = vec!();
                self.byte_source.unget();
                $number_sub_state = LexerNumberSubState::None;
                $state = LexerState::None;
            }};
        }

        macro_rules! end_of_string {
            ($buf:ident, $string_sub_state: ident, $state: ident) => {{
                $buf = vec!();
                $string_sub_state = LexerStringSubState::None;
                $state = LexerState::None;
            }};
        }

        macro_rules! end_of_unicode {
            ($code_point:ident, $unicode_index: ident, $string_sub_state: ident) => {{
                $code_point = 0u32;
                $unicode_index = 0;
                $string_sub_state = LexerStringSubState::None;
            }};
        }

        macro_rules! consume_buf {
            ($buf:ident, $token_variant: ident) => {{
                match String::from_utf8($buf) {
                    Ok(s) => {
                        consumer.consume(Ok(LexerToken::$token_variant(s)), self.line, self.column)?;
                    }
                    Err(e) => {
                        consume_lex_error!("Can't decode string `{}`", e);
                    }
                }
            }};
        }

        let mut bytes = [0u8; 4];

        macro_rules! try_to_append_code_point {
            ($buf:ident, $code_point: ident) => {{
                match char::from_u32($code_point) {
                    Some(c) => {
                        let utf8_bytes = c.encode_utf8(&mut bytes);
                        $buf.append(&mut utf8_bytes.as_bytes().to_vec());
                    }
                    None => {
                        if self.ignore_unicode_errs {
                            let utf8_bytes = REPLACEMENT_CHARACTER.encode_utf8(&mut bytes);
                            $buf.append(&mut utf8_bytes.as_bytes().to_vec());
                        } else {
                            consume_lex_error!("This is not a code point `{}`", $code_point);
                        }
                    }
                }
            }};
        }

        consumer.consume(Ok(BeginFile), self.line, self.column)?;

        let mut state: LexerState = LexerState::None;
        let mut expect: &[u8; 4] = &[1u8, 2u8, 3u8, 4u8];
        let mut expected_index: usize = 0;
        let mut number_sub_state: LexerNumberSubState = LexerNumberSubState::None;
        let mut string_sub_state: LexerStringSubState = LexerStringSubState::None;
        let mut buf: Vec<u8> = vec!();
        let mut code_point: u32 = 0;
        let mut unicode_index: usize = 0;
        let mut high: u32 = 0;

        while let Some(byte) = self.byte_source.get() {
            self.column += 1;
            if byte == b'\n' {
                self.line += 1;
            } else {
                match state {
                    LexerState::None => {
                        match byte {
                            b' ' | b'\t' | b'\r' => {} // pass
                            b'f' => {
                                expect = &[b'a', b'l', b's', b'e'];
                                state = LexerState::Expect(LexerToken::BooleanValue(false));
                                expected_index = 0;
                            }
                            b't' => {
                                expect = &[0u8, b'r', b'u', b'e'];
                                state = LexerState::Expect(LexerToken::BooleanValue(true));
                                expected_index = 1;
                            }
                            b'n' => {
                                expect = &[0u8, b'u', b'l', b'l'];
                                state = LexerState::Expect(LexerToken::NullValue);
                                expected_index = 1;
                            }
                            b'{' => {
                                consumer.consume(Ok(LexerToken::BeginObject), self.line, self.column)?;
                            }
                            b'}' => {
                                consumer.consume(Ok(LexerToken::EndObject), self.line, self.column)?;
                            }
                            b'[' => {
                                consumer.consume(Ok(LexerToken::BeginArray), self.line, self.column)?;
                            }
                            b']' => {
                                consumer.consume(Ok(LexerToken::EndArray), self.line, self.column)?;
                            }
                            b':' => {
                                consumer.consume(Ok(LexerToken::NameSeparator), self.line, self.column)?;
                            }
                            b',' => {
                                consumer.consume(Ok(LexerToken::ValueSeparator), self.line, self.column)?;
                            }
                            b'-' => {
                                state = LexerState::Number;
                                number_sub_state = LexerNumberSubState::NegNumberStart;
                                buf = vec!(b'-');
                            }
                            b'0' => {
                                state = LexerState::Number;
                                number_sub_state = LexerNumberSubState::ZeroNumberStart;
                                buf = vec!(b'0');
                            }
                            b'"' => {
                                state = LexerState::String;
                                string_sub_state = LexerStringSubState::None;
                                buf = vec!();
                            }
                            _ if b'1' <= byte && byte <= b'9' => {
                                state = LexerState::Number;
                                number_sub_state = LexerNumberSubState::OtherNumber;
                                buf = vec!(byte);
                            }
                            _ => {
                                consume_lex_error!("Unexpected char `{}`", byte as char);
                            }
                        }
                    }
                    LexerState::Expect(ref token) if expected_index < expect.len() => {
                        if expect[expected_index] == byte {
                            expected_index += 1;
                        } else {
                            consume_lex_error!("Expected word `{}`", str::from_utf8(expect).unwrap());
                            state = LexerState::None
                        }
                    }
                    LexerState::Expect(token) if expected_index == expect.len() => {
                        self.byte_source.unget();
                        expected_index = 0;
                        consumer.consume(Ok(token), self.line, self.column)?;
                        state = LexerState::None;
                    }
                    LexerState::Number => {  // 6. Numbers
                        match number_sub_state {
                            LexerNumberSubState::NegNumberStart => { // -...
                                match byte {
                                    b'0' => {
                                        buf.push(b'0');
                                        number_sub_state = LexerNumberSubState::ZeroNumberStart;
                                    }
                                    _ if b'1' <= byte && byte <= b'9' => {
                                        buf.push(byte);
                                        number_sub_state = LexerNumberSubState::OtherNumber;
                                    }
                                    _ => {
                                        consume_lex_error!("Expected a digit `{}`", byte as char);
                                        end_of_number!(buf, number_sub_state, state);
                                    }
                                }
                            }
                            LexerNumberSubState::ZeroNumberStart => { // -?0
                                match byte {
                                    b'.' => {
                                        buf.push(b'.');
                                        number_sub_state = LexerNumberSubState::NumberFracStart;
                                    }
                                    b'e' | b'E' => {
                                        buf.push(b'e');
                                        number_sub_state = LexerNumberSubState::NumberFracExpStart;
                                    }
                                    _ => {
                                        consumer.consume(Ok(LexerToken::IntValue("0".into())), self.line, self.column)?;
                                        end_of_number!(buf, number_sub_state, state);
                                    }
                                }
                            }
                            LexerNumberSubState::OtherNumber => { // -?[1-9]
                                match byte {
                                    b'.' => {
                                        buf.push(b'.');
                                        number_sub_state = LexerNumberSubState::NumberFracStart;
                                    }
                                    b'e' | b'E' => {
                                        buf.push(b'e');
                                        number_sub_state = LexerNumberSubState::NumberFracExpStart;
                                    }
                                    _ if b'0' <= byte && byte <= b'9' => {
                                        buf.push(byte);
                                    }
                                    _ => {
                                        consume_buf!(buf, IntValue);
                                        end_of_number!(buf, number_sub_state, state);
                                    }
                                }
                            }
                            LexerNumberSubState::NumberFracStart => { // -?[0-9][1-9]*\.
                                match byte {
                                    _ if b'0' <= byte && byte <= b'9' => {
                                        buf.push(byte);
                                        number_sub_state = LexerNumberSubState::NumberFrac;
                                    }
                                    _ => {
                                        consume_lex_error!("Missing decimals `{}`", String::from_utf8(buf).unwrap());
                                        end_of_number!(buf, number_sub_state, state);
                                    }
                                }
                            }
                            LexerNumberSubState::NumberFrac => { // -?[0-9][1-9]*\.[0-9]+
                                match byte {
                                    b'e' | b'E' => {
                                        buf.push(b'e');
                                        number_sub_state = LexerNumberSubState::NumberFracExpStart;
                                    }
                                    _ if b'0' <= byte && byte <= b'9' => {
                                        buf.push(byte);
                                    }
                                    _ => {
                                        consume_buf!(buf, FloatValue);
                                        end_of_number!(buf, number_sub_state, state);
                                    }
                                }
                            }
                            LexerNumberSubState::NumberFracExpStart => { // -?[0-9][1-9](*\.[0-9]+)?e
                                match byte {
                                    b'-' => {
                                        buf.push(b'-');
                                        number_sub_state = LexerNumberSubState::NumberFracExpMinusStart;
                                    }
                                    _ if b'0' <= byte && byte <= b'9' => {
                                        buf.push(byte);
                                        number_sub_state = LexerNumberSubState::NumberFracExp;
                                    }
                                    _ => {
                                        consume_lex_error!("Missing exp `{}`", String::from_utf8(buf).unwrap());
                                        end_of_number!(buf, number_sub_state, state);
                                    }
                                }
                            }
                            LexerNumberSubState::NumberFracExp => { // -?[0-9][1-9](*\.[0-9]+)?e[0-9]+
                                match byte {
                                    _ if b'0' <= byte && byte <= b'9' => {
                                        buf.push(byte);
                                        number_sub_state = LexerNumberSubState::NumberFracExp;
                                    }
                                    _ => {
                                        consume_buf!(buf, FloatValue);
                                        end_of_number!(buf, number_sub_state, state);
                                    }
                                }
                            }
                            LexerNumberSubState::NumberFracExpMinusStart => { // -?[0-9][1-9](*\.[0-9]+)?e-
                                match byte {
                                    _ if b'0' <= byte && byte <= b'9' => {
                                        buf.push(byte);
                                        number_sub_state = LexerNumberSubState::NumberFracExpMinus;
                                    }
                                    _ => {
                                        consume_lex_error!("Missing exp `{}`", String::from_utf8(buf).unwrap());
                                        end_of_number!(buf, number_sub_state, state);
                                    }
                                }
                            }
                            LexerNumberSubState::NumberFracExpMinus => { // -?[0-9][1-9](*\.[0-9]+)?e-[0-9]+
                                match byte {
                                    _ if b'0' <= byte && byte <= b'9' => {
                                        buf.push(byte);
                                    }
                                    _ => {
                                        consume_buf!(buf, FloatValue);
                                        end_of_number!(buf, number_sub_state, state);
                                    }
                                }
                            }
                            LexerNumberSubState::None => { panic!() }
                        }
                    }
                    LexerState::String => { //  7. Strings
                        if high == 0 {
                            match string_sub_state {
                                LexerStringSubState::Escape => {
                                    match byte {
                                        b'"' | b'\\' => {
                                            buf.push(byte);
                                            string_sub_state = LexerStringSubState::None;
                                        }
                                        b'b' => {
                                            buf.push(0x08);
                                            string_sub_state = LexerStringSubState::None;
                                        }
                                        b'f' => {
                                            buf.push(0x0C);
                                            string_sub_state = LexerStringSubState::None;
                                        }
                                        b'n' => {
                                            buf.push(b'\n');
                                            string_sub_state = LexerStringSubState::None;
                                        }
                                        b'r' => {
                                            buf.push(b'\r');
                                            string_sub_state = LexerStringSubState::None;
                                        }
                                        b't' => {
                                            buf.push(b'\t');
                                            string_sub_state = LexerStringSubState::None;
                                        }
                                        b'u' => {
                                            string_sub_state = LexerStringSubState::Unicode;
                                            code_point = 0u32;
                                            unicode_index = 0;
                                        }
                                        _ => {
                                            consume_lex_error!("Unknown escaped char `{}`", byte as char);
                                        }
                                    }
                                }
                                LexerStringSubState::Unicode => { // \u was seen
                                    if unicode_index <= 3 {
                                        let n = self.parse_hex(byte);
                                        match n {
                                            Ok(i) => {
                                                code_point = code_point * 16 + i;
                                                unicode_index += 1;
                                            }
                                            Err(e) => {
                                                end_of_unicode!(code_point, unicode_index, string_sub_state);
                                                consumer.consume(Err(e), self.line, self.column)?;
                                            }
                                        }
                                    }
                                    if unicode_index == 4 {
                                        if 0xd800 <= code_point && code_point <= 0xdbff {
                                            high = code_point;
                                        } else {
                                            try_to_append_code_point!(buf, code_point);
                                        }
                                        end_of_unicode!(code_point, unicode_index, string_sub_state);
                                    }
                                }
                                LexerStringSubState::None => {
                                    match byte {
                                        b'\\' => { string_sub_state = LexerStringSubState::Escape }
                                        b'"' => {
                                            consume_buf!(buf, String);
                                            end_of_string!(buf, string_sub_state, state);
                                        }
                                        _ => {
                                            buf.push(byte);
                                        }
                                    }
                                }
                            }
                        } else {
                            match string_sub_state {
                                LexerStringSubState::Escape => {
                                    match byte {
                                        b'u' => {
                                            string_sub_state = LexerStringSubState::Unicode;
                                            code_point = 0u32;
                                            unicode_index = 0;
                                        }
                                        _ => {
                                            consume_lex_error!("Waiting for low surrogate: needs \\u, got `\\{}`", byte as char);
                                            self.byte_source.unget();
                                            high = 0;
                                        }
                                    }
                                }
                                LexerStringSubState::Unicode => { // \u was seen
                                    if unicode_index <= 3 {
                                        let n = self.parse_hex(byte);
                                        match n {
                                            Ok(i) => {
                                                code_point = code_point * 16 + i;
                                                unicode_index += 1;
                                            }
                                            Err(e) => {
                                                end_of_unicode!(code_point, unicode_index, string_sub_state);
                                                consumer.consume(Err(e), self.line, self.column)?;
                                            }
                                        }
                                    }
                                    if unicode_index == 4 {
                                        if 0xdc00 <= code_point && code_point <= 0xdfff {
                                            code_point = 0x10000 + (high - 0xd800) * 0x400 + code_point - 0xdc00;
                                            try_to_append_code_point!(buf, code_point);
                                        } else {
                                            consume_lex_error!("Waiting for low surrogate, got `{}`", code_point);
                                            let utf8_bytes = REPLACEMENT_CHARACTER.encode_utf8(&mut bytes);
                                            buf.append(&mut utf8_bytes.as_bytes().to_vec());
                                        }
                                        high = 0;
                                        end_of_unicode!(code_point, unicode_index, string_sub_state);
                                    }
                                }
                                LexerStringSubState::None => {
                                    match byte {
                                        b'\\' => { string_sub_state = LexerStringSubState::Escape }
                                        _ => {
                                            consume_lex_error!("Waiting for low surrogate: needs backslash, got `{}`", byte as char);
                                            self.byte_source.unget();
                                            high = 0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        // end of loop
        match state {
            LexerState::Number => {  // finish our number if possible
                match number_sub_state {
                    LexerNumberSubState::ZeroNumberStart => { // 0
                        consumer.consume(Ok(LexerToken::IntValue("0".into())), self.line, self.column)?;
                    }
                    LexerNumberSubState::NegNumberStart => {
                        // -
                        consume_lex_error!("Missing digits `{}`", String::from_utf8(buf).unwrap());
                    }
                    LexerNumberSubState::OtherNumber => {
                        // [1-9]
                        consume_buf!(buf, IntValue);
                    }
                    LexerNumberSubState::NumberFracStart => {
                        //  [0-9]\.
                        consume_lex_error!("Missing decimals `{}`", String::from_utf8(buf).unwrap());
                    }
                    LexerNumberSubState::NumberFrac => {
                        // [0-9]\.[0-9]
                        consume_buf!(buf, FloatValue);
                    }
                    LexerNumberSubState::NumberFracExpStart => {
                        consume_lex_error!("Missing exp `{}`", String::from_utf8(buf).unwrap());
                    }
                    LexerNumberSubState::NumberFracExp => {
                        consume_buf!(buf, FloatValue);
                    }
                    LexerNumberSubState::NumberFracExpMinusStart => {
                        consume_lex_error!("Missing exp `{}`", String::from_utf8(buf).unwrap());
                    }
                    LexerNumberSubState::NumberFracExpMinus => {
                        consume_buf!(buf, FloatValue);
                    }
                    _ => {
                        consume_lex_error!("Unexpected sub_state");
                    }
                }
            }
            LexerState::String => {
                match String::from_utf8(buf) {
                    Ok(s) => { consume_lex_error!("Unfinished string `{}`", s); }
                    Err(e) => { consume_lex_error!("Can't decode string `{}`", e); }
                }
            }
            LexerState::None => {
                // pass
            }
            _ => { consume_lex_error!("Unexpected sub_state"); }
        }
        consumer.consume(Ok(EndFile), self.line, self.column)?;
        Ok(())
    }

    #[inline]
    fn parse_hex(&self, byte: u8) -> Result<u32, JSONLexError> {
        macro_rules! lex_error {
            ($($arg:tt)*) => {{
                Err(JSONLexError {
                    msg: format!($($arg)*),
                    line: self.line,
                    column: self.column,
                })
            }}
        }

        match byte {
            _ if b'0' <= byte && byte <= b'9' => { Ok((byte - b'0') as u32) }
            _ if b'a' <= byte && byte <= b'f' => { Ok((byte - b'a') as u32 + 10) }
            _ if b'A' <= byte && byte <= b'F' => { Ok((byte - b'A') as u32 + 10) }
            _ => {
                lex_error!("Unknown hex digit `{}`", byte as char)
            }
        }
    }
}

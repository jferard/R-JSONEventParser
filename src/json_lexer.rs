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
}

#[derive(Debug, PartialEq)]
pub struct JSONLexError {
    pub msg: String,
    pub line: usize,
    pub column: usize,
}

pub trait JSONLexConsumer {
    fn consume(&mut self, token: Result<LexerToken, JSONLexError>);
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

enum LexerStringSubState {
    None,
    Escape,
    Unicode,
}


pub struct JSONLexer<R: Read> {
    byte_source: ByteSource<R>,
    line: usize,
    column: usize,
}

impl<R: Read> JSONLexer<R> {
    pub fn new(byte_source: ByteSource<R>) -> Self {
        JSONLexer {
            byte_source,
            line: 0,
            column: 0,
        }
    }

    pub fn lex<C: JSONLexConsumer>(&mut self, consumer: &mut C) {
        macro_rules! lex_error {
            ($($arg:tt)*) => {{
                consumer.consume(Err(JSONLexError {
                    msg: format!($($arg)*),
                    line: self.line,
                    column: self.column,
                }));
            }};
        }

        macro_rules! end_of_number {
            ($buf:ident, $number_sub_state: ident, $state: ident) => {{
                $buf = vec!();
                self.byte_source.unget();
                $number_sub_state = LexerNumberSubState::None;
                $state = LexerState::None;
            }};
        }

        let mut state: LexerState = LexerState::None;
        let mut expect: &[u8; 4] = &[1u8, 2u8, 3u8, 4u8];
        let mut expected_index: usize = 0;
        let mut number_sub_state: LexerNumberSubState = LexerNumberSubState::None;
        let mut string_sub_state: LexerStringSubState = LexerStringSubState::None;
        let mut buf: Vec<u8> = vec!();
        let mut sub_buf = [0u8; 4];

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
                                consumer.consume(Ok(LexerToken::BeginObject));
                            }
                            b'}' => {
                                consumer.consume(Ok(LexerToken::EndObject));
                            }
                            b'[' => {
                                consumer.consume(Ok(LexerToken::BeginArray));
                            }
                            b']' => {
                                consumer.consume(Ok(LexerToken::EndArray));
                            }
                            b':' => {
                                consumer.consume(Ok(LexerToken::NameSeparator));
                            }
                            b',' => {
                                consumer.consume(Ok(LexerToken::ValueSeparator));
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
                                lex_error!("Unexpected char `{}`", byte as char);
                            }
                        }
                    }
                    LexerState::Expect(ref token) if expected_index < expect.len() => {
                        if expect[expected_index] == byte {
                            expected_index += 1;
                        } else {
                            lex_error!("Expected word `{}`", str::from_utf8(expect).unwrap());
                            state = LexerState::None
                        }
                    }
                    LexerState::Expect(token) if expected_index == expect.len() => {
                        self.byte_source.unget();
                        expected_index = 0;
                        consumer.consume(Ok(token));
                        state = LexerState::None;
                    }
                    LexerState::Number => {  // 6. Numbers
                        match number_sub_state {
                            LexerNumberSubState::NegNumberStart => { // -...
                                match byte {
                                    b'0' => {
                                        buf.push(0);
                                        number_sub_state = LexerNumberSubState::ZeroNumberStart;
                                    }
                                    _ if b'1' <= byte && byte <= b'9' => {
                                        buf.push(byte);
                                        number_sub_state = LexerNumberSubState::OtherNumber;
                                    }
                                    _ => {
                                        lex_error!("Expected a digit `{}`", byte as char);
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
                                        consumer.consume(Ok(LexerToken::IntValue("0".into())));
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
                                        match String::from_utf8(buf) {
                                            Ok(s) => {
                                                consumer.consume(Ok(LexerToken::IntValue(s)));
                                            }
                                            Err(e) => {
                                                lex_error!("Can't decode string `{}`", e);
                                            }
                                        }
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
                                        lex_error!("Missing decimals `{}`", String::from_utf8(buf).unwrap());
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
                                        match String::from_utf8(buf) {
                                            Ok(s) => {
                                                consumer.consume(Ok(LexerToken::FloatValue(s)));
                                            }
                                            Err(e) => {
                                                lex_error!("Can't decode string `{}`", e);
                                            }
                                        }
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
                                        lex_error!("Missing exp `{}`", String::from_utf8(buf).unwrap());
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
                                        match String::from_utf8(buf) {
                                            Ok(s) => {
                                                consumer.consume(Ok(LexerToken::FloatValue(s)));
                                            }
                                            Err(e) => {
                                                lex_error!("Can't decode string `{}`", e);
                                            }
                                        }
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
                                        lex_error!("Missing exp `{}`", String::from_utf8(buf).unwrap());
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
                                        match String::from_utf8(buf) {
                                            Ok(s) => {
                                                consumer.consume(
                                                    Ok(LexerToken::FloatValue(s)));
                                            }
                                            Err(e) => {
                                                lex_error!("Can't decode string `{}`", e);
                                            }
                                        }
                                        end_of_number!(buf, number_sub_state, state);
                                    }
                                }
                            }
                            LexerNumberSubState::None => { panic!() }
                        }
                    }
                    LexerState::String => { //  7. Strings
                        match string_sub_state {
                            LexerStringSubState::Escape => {
                                match byte {
                                    b'"' | b'\\' => {
                                        buf.push(byte);
                                        number_sub_state = LexerNumberSubState::None
                                    }
                                    b'b' => {
                                        buf.push(0x08);
                                        number_sub_state = LexerNumberSubState::None
                                    }
                                    b'f' => {
                                        buf.push(0x0C);
                                        number_sub_state = LexerNumberSubState::None
                                    }
                                    b'n' => {
                                        buf.push(b'\n');
                                        number_sub_state = LexerNumberSubState::None
                                    }
                                    b'r' => {
                                        buf.push(b'\r');
                                        string_sub_state = LexerStringSubState::None
                                    }
                                    b't' => {
                                        buf.push(b'\t');
                                        string_sub_state = LexerStringSubState::None
                                    }
                                    b'u' => {
                                        string_sub_state = LexerStringSubState::Unicode
                                    }
                                    _ => {
                                        lex_error!("Unknown escaped char `{}`", byte as char);
                                    }
                                }
                            }
                            LexerStringSubState::Unicode => {
                                sub_buf[0] = 1u8;
                                // TODO
                            }
                            LexerStringSubState::None => {
                                match byte {
                                    b'\\' => { string_sub_state = LexerStringSubState::Escape }
                                    b'"' => {
                                        match String::from_utf8(buf) {
                                            Ok(s) => {
                                                consumer.consume(Ok(LexerToken::String(s)));
                                            }
                                            Err(e) => {
                                                lex_error!("Can't decode string `{}`", e);
                                            }
                                        }
                                        buf = vec!();
                                        state = LexerState::None;
                                        number_sub_state = LexerNumberSubState::None;
                                    }
                                    _ => {
                                        buf.push(byte);
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
                        consumer.consume(Ok(LexerToken::IntValue("0".into())));
                    }
                    LexerNumberSubState::NegNumberStart => {
                        // -
                        lex_error!("Missing digits `{}`", String::from_utf8(buf).unwrap());
                    }
                    LexerNumberSubState::OtherNumber => { // [1-9]
                        match String::from_utf8(buf) {
                            Ok(s) => {
                                consumer.consume(Ok(LexerToken::IntValue(s)));
                            }
                            Err(e) => {
                                lex_error!("Can't decode string `{}`", e);
                            }
                        }
                    }
                    LexerNumberSubState::NumberFracStart => {
                        //  [0-9]\.
                        lex_error!("Missing decimals `{}`", String::from_utf8(buf).unwrap());
                    }
                    LexerNumberSubState::NumberFrac => { // [0-9]\.[0-9]
                        match String::from_utf8(buf) {
                            Ok(s) => {
                                consumer.consume(Ok(LexerToken::FloatValue(s)));
                            }
                            Err(e) => {
                                lex_error!("Can't decode string `{}`", e);
                            }
                        }
                    }
                    LexerNumberSubState::NumberFracExpStart => {
                        lex_error!("Missing exp `{}`", String::from_utf8(buf).unwrap());
                    }
                    LexerNumberSubState::NumberFracExp => {
                        match String::from_utf8(buf) {
                            Ok(s) => {
                                consumer.consume(Ok(LexerToken::FloatValue(s)));
                            }
                            Err(e) => {
                                lex_error!("Can't decode string `{}`", e);
                            }
                        }
                    }
                    LexerNumberSubState::NumberFracExpMinusStart => {
                        lex_error!("Missing exp `{}`", String::from_utf8(buf).unwrap());
                    }
                    LexerNumberSubState::NumberFracExpMinus => {
                        match String::from_utf8(buf) {
                            Ok(s) => {
                                consumer.consume(Ok(LexerToken::FloatValue(s)));
                            }
                            Err(e) => {
                                lex_error!("Can't decode string `{}`", e);
                            }
                        }
                    }
                    _ => {
                        lex_error!("Unexpected sub_state");
                    }
                }
            }
            LexerState::String => {
                match String::from_utf8(buf) {
                    Ok(s) => { lex_error!("Unfinished string `{}`", s); }
                    Err(e) => { lex_error!("Can't decode string `{}`", e); }
                }
            }
            LexerState::None => {
                // pass
            }
            _ => { lex_error!("Unexpected sub_state"); }
        }
    }
}

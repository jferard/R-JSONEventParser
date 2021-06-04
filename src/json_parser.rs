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

use std::io::Read;

use crate::byte_source::ByteSource;
use crate::json_lexer::{ConsumeError, JSONLexConsumer, JSONLexer, JSONLexError, LexerToken};
use crate::json_lexer::LexerToken::BeginFile;

#[derive(Debug, PartialEq)]
pub enum ParserToken {
    BeginFile,
    EndFile,
    BeginObject,
    EndObject,
    BeginArray,
    EndArray,
    Key(String),
    BooleanValue(bool),
    NullValue,
    StringValue(String),
    IntValue(String),
    FloatValue(String),
}

#[derive(Debug, PartialEq)]
pub struct JSONParseError {
    pub msg: String,
    pub line: usize,
    pub column: usize,
}


pub trait JSONParseConsumer {
    fn consume(&mut self, token: Result<ParserToken, JSONParseError>) -> Result<(), ConsumeError>;
}

#[derive(Debug, PartialEq)]
enum ParserState {
    Undefined,
    None,
    InObject,
    InObjectMember,
    InObjectMemberValue,
    InObjectSep,
    InArray,
    InArraySep,
}

pub struct JSONParser<R: Read> {
    json_lexer: JSONLexer<R>,
}

pub struct JSONLexerToParser<'a, C: JSONParseConsumer> {
    consumer: &'a mut C,
    state: ParserState,
    states: Vec<ParserState>,
}

impl<'a, C: JSONParseConsumer> JSONLexConsumer for JSONLexerToParser<'a, C> {
    fn consume(&mut self, token: Result<LexerToken, JSONLexError>, line: usize, column: usize) -> Result<(), ConsumeError> {
        macro_rules! parse_error {
            ($($arg:tt)*) => {{
                Err(JSONParseError {
                    msg: format!($($arg)*),
                    line,
                    column,
                })
            }};
        }

        macro_rules! consume_parse_error {
            ($($arg:tt)*) => {{
                self.consumer.consume(parse_error!($($arg)*))?;
            }};
        }

        if let Err(e) = token {
            self.consumer.consume(Err(JSONParseError {
                msg: e.msg,
                line: e.line,
                column: e.column,
            }))?;
            return Err(ConsumeError);
        }
        match self.state {
            ParserState::Undefined => {
                self.consumer.consume(match token {
                    Ok(BeginFile) => {
                        self.state = ParserState::None;
                        Ok(ParserToken::BeginFile)
                    }
                    _ => parse_error!("Unexpected state")
                })?
            }
            ParserState::None => {
                let token = match token {
                    Ok(LexerToken::EndFile) => {
                        match self.states.last() {
                            Some(t) => parse_error!("Should be closed: {:?}", t),
                            _ => Ok(ParserToken::EndFile)
                        }
                    }
                    Ok(LexerToken::BeginObject) => {
                        self.states.push(ParserState::None);
                        self.state = ParserState::InObject;
                        Ok(ParserToken::BeginObject)
                    }
                    Ok(LexerToken::BeginArray) => {
                        self.states.push(ParserState::None);
                        self.state = ParserState::InArray;
                        Ok(ParserToken::BeginArray)
                    }
                    Ok(LexerToken::BooleanValue(b)) => {
                        Ok(ParserToken::BooleanValue(b))
                    }
                    Ok(LexerToken::NullValue) => {
                        Ok(ParserToken::NullValue)
                    }
                    Ok(LexerToken::IntValue(s)) => {
                        Ok(ParserToken::IntValue(s))
                    }
                    Ok(LexerToken::FloatValue(s)) => {
                        Ok(ParserToken::FloatValue(s))
                    }
                    Ok(LexerToken::String(s)) => {
                        Ok(ParserToken::StringValue(s))
                    }
                    t => {
                        parse_error!("Unexpected token `{:?}`", t)
                    }
                };
                self.consumer.consume(token)?;
            }
            ParserState::InObject => {
                let token = match token {
                    Ok(LexerToken::EndObject) => {
                        self.state = self.states.pop().unwrap();
                        Ok(ParserToken::EndObject)
                    }
                    Ok(LexerToken::String(s)) => {
                        self.state = ParserState::InObjectMember;
                        Ok(ParserToken::Key(s))
                    }
                    t => {
                        parse_error!("Unexpected token `{:?}`", t)
                    }
                };
                self.consumer.consume(token)?;
            }
            ParserState::InObjectMember => {
                match token {
                    Ok(LexerToken::NameSeparator) => {
                        self.state = ParserState::InObjectMemberValue
                    }
                    t => {
                        consume_parse_error!("Unexpected token `{:?}`", t);
                    }
                }
            }
            ParserState::InObjectMemberValue => {
                let token = match token {
                    Ok(LexerToken::BooleanValue(b)) => {
                        self.state = ParserState::InObjectSep;
                        Ok(ParserToken::BooleanValue(b))
                    }
                    Ok(LexerToken::NullValue) => {
                        self.state = ParserState::InObjectSep;
                        Ok(ParserToken::NullValue)
                    }
                    Ok(LexerToken::IntValue(s)) => {
                        self.state = ParserState::InObjectSep;
                        Ok(ParserToken::IntValue(s))
                    }
                    Ok(LexerToken::FloatValue(s)) => {
                        self.state = ParserState::InObjectSep;
                        Ok(ParserToken::FloatValue(s))
                    }
                    Ok(LexerToken::String(s)) => {
                        self.state = ParserState::InObjectSep;
                        Ok(ParserToken::StringValue(s))
                    }
                    Ok(LexerToken::BeginObject) => {
                        self.states.push(ParserState::InObjectSep);
                        self.state = ParserState::InObject;
                        Ok(ParserToken::BeginObject)
                    }
                    Ok(LexerToken::BeginArray) => {
                        self.states.push(ParserState::InObjectSep);
                        self.state = ParserState::InArray;
                        Ok(ParserToken::BeginArray)
                    }
                    t => {
                        parse_error!("Unexpected token `{:?}`", t)
                    }
                };
                self.consumer.consume(token)?;
            }
            ParserState::InObjectSep => {
                match token {
                    Ok(LexerToken::ValueSeparator) => {
                        self.state = ParserState::InObject
                    }
                    Ok(LexerToken::EndObject) => {
                        self.state = self.states.pop().unwrap();
                        self.consumer.consume(Ok(ParserToken::EndObject))?;
                    }
                    t => {
                        consume_parse_error!("Unexpected token `{:?}`", t);
                    }
                }
            }
            ParserState::InArray => {
                let token = match token {
                    Ok(LexerToken::EndArray) => {
                        self.state = self.states.pop().unwrap();
                        Ok(ParserToken::EndArray)
                    }
                    Ok(LexerToken::BooleanValue(b)) => {
                        self.state = ParserState::InArraySep;
                        Ok(ParserToken::BooleanValue(b))
                    }
                    Ok(LexerToken::NullValue) => {
                        self.state = ParserState::InArraySep;
                        Ok(ParserToken::NullValue)
                    }
                    Ok(LexerToken::IntValue(s)) => {
                        self.state = ParserState::InArraySep;
                        Ok(ParserToken::IntValue(s))
                    }
                    Ok(LexerToken::FloatValue(s)) => {
                        self.state = ParserState::InArraySep;
                        Ok(ParserToken::FloatValue(s))
                    }
                    Ok(LexerToken::String(s)) => {
                        self.state = ParserState::InArraySep;
                        Ok(ParserToken::StringValue(s))
                    }
                    Ok(LexerToken::BeginObject) => {
                        self.states.push(ParserState::InArraySep);
                        self.state = ParserState::InObject;
                        Ok(ParserToken::BeginObject)
                    }
                    Ok(LexerToken::BeginArray) => {
                        self.states.push(ParserState::InArraySep);
                        self.state = ParserState::InArray;
                        Ok(ParserToken::BeginArray)
                    }
                    t => {
                        parse_error!("Unexpected token `{:?}`", t)
                    }
                };
                self.consumer.consume(token)?;
            }
            ParserState::InArraySep => {
                match token {
                    Ok(LexerToken::ValueSeparator) => {
                        self.state = ParserState::InArray
                    }
                    Ok(LexerToken::EndArray) => {
                        self.state = self.states.pop().unwrap();
                        self.consumer.consume(Ok(ParserToken::EndArray))?;
                    }
                    t => {
                        consume_parse_error!("Unexpected token `{:?}`", t);
                    }
                }
            }
        }
        Ok(())
    }
}

impl<'a, C: JSONParseConsumer> JSONLexerToParser<'a, C> {
    pub fn new(consumer: &'a mut C) -> Self {
        JSONLexerToParser {
            consumer,
            state: ParserState::Undefined,
            states: vec!(),
        }
    }
}

impl<R: Read> JSONParser<R> {
    pub fn new(byte_source: ByteSource<R>) -> Self {
        JSONParser {
            json_lexer: JSONLexer::new(byte_source),
        }
    }

    pub fn parse<C: JSONParseConsumer>(&mut self, consumer: &mut C) -> Result<(), ConsumeError> {
        let mut parser = JSONLexerToParser::new(consumer);
        self.json_lexer.lex(&mut parser)
    }
}
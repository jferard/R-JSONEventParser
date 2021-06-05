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

use std::io::Write;

use crate::json_lexer::ConsumeError;
use crate::json_parser::{JSONParseConsumer, JSONParseError, ParserToken};
use crate::json_parser::ParserToken::{BeginArray, BeginFile, EndFile, BeginObject, BooleanValue, EndArray, EndObject, FloatValue, IntValue, Key, NullValue, StringValue};
use std::io;

pub struct JSON2XMLConsumer<W: Write> {
    pub destination: W,
    formatted: bool,
    typed: bool,
    pub states_stack: Vec<ParserToken>,
    pub keys_stack: Vec<String>,
}

impl<W: Write> JSON2XMLConsumer<W> {
    pub fn new(destination: W, formatted: bool, typed: bool) -> Self {
        return JSON2XMLConsumer {
            destination,
            formatted,
            typed,
            states_stack: vec!(),
            keys_stack: vec!(),
        };
    }
}

impl<W: Write> JSONParseConsumer for JSON2XMLConsumer<W> {
    fn consume(&mut self, token: Result<ParserToken, JSONParseError>) -> Result<(), ConsumeError> {
        let result = match token {
            Ok(BeginFile) => {
                self.destination.write_fmt(format_args!("{}\n<{}>\n", "<?xml version=\"1.0\" encoding=\"utf-8\"?>", "root"))
            }
            Ok(EndFile) => {
                self.destination.write_fmt(format_args!("</{}>\n", "root"))
            }
            Ok(BeginObject) | Ok(BeginArray) => {
                let r = match self.states_stack.last() {
                    Some(BeginArray) => {
                        let cur_key = "li";
                        self.keys_stack.push(cur_key.into());
                        if self.formatted {
                            self.destination.write_fmt(format_args!("{0: >1$}<{2}>\n", "", self.states_stack.len() * 4, cur_key))
                        } else {
                            self.destination.write_fmt(format_args!("<{}>", cur_key)) // spaces
                        }
                    }
                    Some(_) => {
                        let cur_key = self.keys_stack.last().unwrap();
                        if self.formatted {
                            self.destination.write_fmt(format_args!("{0: >1$}<{2}>\n", "", self.states_stack.len() * 4, cur_key))
                        } else {
                            self.destination.write_fmt(format_args!("<{}>", cur_key)) // spaces
                        }
                    }
                    None => { Ok(()) }
                };
                match r {
                    Err(_) => { return Err(ConsumeError); }
                    _ => {}
                }
                self.states_stack.push(token.unwrap());
                Ok(())
            }
            Ok(EndObject) | Ok(EndArray) => {
                self.states_stack.pop().unwrap();
                match self.states_stack.last() {
                    Some(_) => {
                        let previous_key = self.keys_stack.pop().unwrap();
                        if self.formatted {
                            self.destination.write_fmt(format_args!("{0: >1$}</{2}>\n", "", self.states_stack.len() * 4, previous_key))
                        } else {
                            self.destination.write_fmt(format_args!("</{}>", previous_key))
                        }
                    }
                    None => {
                        Ok(())
                    }
                }
            }
            Ok(Key(s)) => {
                self.keys_stack.push(s);
                Ok(())
            }
            Ok(BooleanValue(b)) => {
                let cur_key = self.get_cur_key()?;
                let value = if b { "true".into() } else { "false".into() };
                self.write_value(cur_key, "boolean", value)
            }
            Ok(NullValue) => {
                let cur_key = self.get_cur_key()?;
                self.write_value(cur_key, "null", String::from("null"))
            }
            Ok(StringValue(s)) => {
                let cur_key = self.get_cur_key()?;
                if s.is_empty() {
                    self.write_empty_string_value(cur_key)
                } else {
                    self.write_string_value(cur_key, s)
                }
            }
            Ok(IntValue(s)) => {
                let cur_key = self.get_cur_key()?;
                self.write_value(cur_key, "int", s)
            }
            Ok(FloatValue(s)) => {
                let cur_key = self.get_cur_key()?;
                self.write_value(cur_key, "float", s)
            }
            Err(_) => { return Err(ConsumeError); }
        };
        match result {
            Ok(_) => { Ok(()) }
            Err(_) => { Err(ConsumeError) }
        }
    }
}

impl<W: Write> JSON2XMLConsumer<W> {
    fn get_cur_key(&mut self) -> Result<String, ConsumeError> {
        match self.states_stack.last() {
            Some(BeginArray) => { Ok("li".into()) }
            Some(_) => { Ok(self.keys_stack.pop().unwrap()) }
            None => { return Err(ConsumeError); }
        }
    }

    fn write_value(&mut self, cur_key: String, value_type: &str, value: String) -> io::Result<()> {
        if self.formatted {
            if self.typed {
                self.destination.write_fmt(format_args!("{0: >1$}<{2} type=\"{3}\">{4}</{2}>\n", "", self.states_stack.len() * 4, cur_key, value_type, value))
            } else {
                self.destination.write_fmt(format_args!("{0: >1$}<{2}>{3}</{2}>\n", "", self.states_stack.len() * 4, cur_key, value))
            }
        } else {
            if self.typed {
                self.destination.write_fmt(format_args!("<{0} type=\"{1}\">{2}</{0}>", cur_key, value_type, value))
            } else {
                self.destination.write_fmt(format_args!("<{0}>{1}</{0}>", cur_key, value))
            }
        }
    }

    fn write_string_value(&mut self, cur_key: String, value: String) -> io::Result<()> {
        let e_value = JSON2XMLConsumer::<W>::escape_value(value);
        if self.formatted {
            if self.typed {
                self.destination.write_fmt(format_args!("{0: >1$}<{2} type=\"string\">{3}</{2}>\n", "", self.states_stack.len() * 4, cur_key, e_value))
            } else {
                self.destination.write_fmt(format_args!("{0: >1$}<{2}>{3}</{2}>\n", "", self.states_stack.len() * 4, cur_key, e_value))
            }
        } else {
            if self.typed {
                self.destination.write_fmt(format_args!("<{0} type=\"string\">{1}</{0}>", cur_key, e_value))
            } else {
                self.destination.write_fmt(format_args!("<{0}>{1}</{0}>", cur_key, e_value))
            }
        }
    }

    fn write_empty_string_value(&mut self, cur_key: String) -> io::Result<()> {
        if self.formatted {
            if self.typed {
                self.destination.write_fmt(format_args!("{0: >1$}<{2} type=\"string\"/>\n", "", self.states_stack.len() * 4, cur_key))
            } else {
                self.destination.write_fmt(format_args!("{0: >1$}<{2}/>\n", "", self.states_stack.len() * 4, cur_key))
            }
        } else {
            if self.typed {
                self.destination.write_fmt(format_args!("<{0} type=\"string\"/>", cur_key))
            } else {
                self.destination.write_fmt(format_args!("<{0}/>", cur_key))
            }
        }
    }

    fn escape_value(s: String) -> String {
        if s.find(&['<', '>', '&', '"', '\''][..]).is_some() {
            if s.find("]]>").is_some() {
                format!("{}{}{}", "<![CDATA[", s.replace("]]>", "]]]]><![CDATA[>"), "]]>")
            } else {
                format!("{}{}{}", "<![CDATA[", s, "]]>")
            }
        } else {
            s
        }
    }
}


fn _main() {}


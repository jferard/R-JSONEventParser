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

use std::io;
use std::io::Write;
use std::marker::PhantomData;

use crate::json_lexer::ConsumeError;
use crate::json_parser::{JSONParseConsumer, JSONParseError, ParserToken};
use crate::json_parser::ParserToken::{BeginArray, BeginFile, BeginObject, BooleanValue, EndArray, EndFile, EndObject, FloatValue, IntValue, Key, NullValue, StringValue};

pub trait XMLWrite<W: Write> {
    fn write_value(&mut self, size: usize, cur_key: String, value_type: &str, value: String) -> io::Result<()>;

    fn write_string_value(&mut self, size: usize, cur_key: String, value: String) -> io::Result<()>;

    fn write_open(&mut self) -> io::Result<()>;

    fn write_close(&mut self) -> io::Result<()>;

    fn write_begin(&mut self, size: usize, cur_key: &str) -> io::Result<()>;

    fn write_end(&mut self, size: usize, cur_key: &str) -> io::Result<()>;

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

pub struct FormattedTypedXMLWrite<W: Write> {
    destination: W,
}

impl<W: Write> XMLWrite<W> for FormattedTypedXMLWrite<W> {
    fn write_value(&mut self, size: usize, cur_key: String, value_type: &str, value: String) -> io::Result<()> {
        self.destination.write_fmt(format_args!("{0: >1$}<{2} type=\"{3}\">{4}</{2}>\n", "", size, cur_key, value_type, value))
    }

    fn write_string_value(&mut self, size: usize, cur_key: String, value: String) -> io::Result<()> {
        if value.is_empty() {
            self.destination.write_fmt(format_args!("{0: >1$}<{2} type=\"string\"/>\n", "", size, cur_key))
        } else {
            let e_value = FormattedTypedXMLWrite::<W>::escape_value(value);
            self.destination.write_fmt(format_args!("{0: >1$}<{2} type=\"string\">{3}</{2}>\n", "", size, cur_key, e_value))
        }
    }

    fn write_open(&mut self) -> io::Result<()> {
        self.destination.write_fmt(format_args!("{}\n<{}>\n", "<?xml version=\"1.0\" encoding=\"utf-8\"?>", "root"))
    }

    fn write_close(&mut self) -> io::Result<()> {
        self.destination.write_fmt(format_args!("</{}>\n", "root"))
    }

    fn write_begin(&mut self, size: usize, cur_key: &str) -> io::Result<()> {
        self.destination.write_fmt(format_args!("{0: >1$}<{2}>\n", "", size, cur_key))
    }

    fn write_end(&mut self, size: usize, cur_key: &str) -> io::Result<()> {
        self.destination.write_fmt(format_args!("{0: >1$}</{2}>\n", "", size, cur_key))
    }
}

pub struct FormattedXMLWrite<W: Write> {
    destination: W,
}

impl<W: Write> XMLWrite<W> for FormattedXMLWrite<W> {
    fn write_value(&mut self, size: usize, cur_key: String, _value_type: &str, value: String) -> io::Result<()> {
        self.destination.write_fmt(format_args!("{0: >1$}<{2}>{3}</{2}>\n", "", size, cur_key, value))
    }

    fn write_string_value(&mut self, size: usize, cur_key: String, value: String) -> io::Result<()> {
        if value.is_empty() {
            self.destination.write_fmt(format_args!("{0: >1$}<{2}/>\n", "", size, cur_key))
        } else {
            let e_value = FormattedXMLWrite::<W>::escape_value(value);
            self.destination.write_fmt(format_args!("{0: >1$}<{2}>{3}</{2}>\n", "", size, cur_key, e_value))
        }
    }

    fn write_open(&mut self) -> io::Result<()> {
        self.destination.write_fmt(format_args!("{}\n<{}>\n", "<?xml version=\"1.0\" encoding=\"utf-8\"?>", "root"))
    }

    fn write_close(&mut self) -> io::Result<()> {
        self.destination.write_fmt(format_args!("</{}>\n", "root"))
    }

    fn write_begin(&mut self, size: usize, cur_key: &str) -> io::Result<()> {
        self.destination.write_fmt(format_args!("{0: >1$}<{2}>\n", "", size, cur_key))
    }

    fn write_end(&mut self, size: usize, cur_key: &str) -> io::Result<()> {
        self.destination.write_fmt(format_args!("{0: >1$}</{2}>\n", "", size, cur_key))
    }
}

pub struct TypedXMLWrite<W: Write> {
    destination: W,
}

impl<W: Write> XMLWrite<W> for TypedXMLWrite<W> {
    fn write_value(&mut self, _size: usize, cur_key: String, value_type: &str, value: String) -> io::Result<()> {
        self.destination.write_fmt(format_args!("<{0} type=\"{1}\">{2}</{0}>\n", cur_key, value_type, value))
    }

    fn write_string_value(&mut self, _size: usize, cur_key: String, value: String) -> io::Result<()> {
        if value.is_empty() {
            self.destination.write_fmt(format_args!("<{0} type=\"string\"/>", cur_key))
        } else {
            let e_value = TypedXMLWrite::<W>::escape_value(value);
            self.destination.write_fmt(format_args!("<{0} type=\"string\">{1}</{0}>", cur_key, e_value))
        }
    }

    fn write_open(&mut self) -> io::Result<()> {
        self.destination.write_fmt(format_args!("{}\n<{}>", "<?xml version=\"1.0\" encoding=\"utf-8\"?>", "root"))
    }

    fn write_close(&mut self) -> io::Result<()> {
        self.destination.write_fmt(format_args!("</{}>", "root"))
    }

    fn write_begin(&mut self, _size: usize, cur_key: &str) -> io::Result<()> {
        self.destination.write_fmt(format_args!("<{}>", cur_key))
    }

    fn write_end(&mut self, _size: usize, cur_key: &str) -> io::Result<()> {
        self.destination.write_fmt(format_args!("</{}>", cur_key))
    }
}


pub struct RawXMLWrite<W: Write> {
    destination: W,
}

impl<W: Write> XMLWrite<W> for RawXMLWrite<W> {
    fn write_value(&mut self, _size: usize, cur_key: String, _value_type: &str, value: String) -> io::Result<()> {
        self.destination.write_fmt(format_args!("<{0}>{1}</{0}>\n", cur_key, value))
    }

    fn write_string_value(&mut self, _size: usize, cur_key: String, value: String) -> io::Result<()> {
        if value.is_empty() {
            self.destination.write_fmt(format_args!("<{0}/>", cur_key))
        } else {
            let e_value = RawXMLWrite::<W>::escape_value(value);
            self.destination.write_fmt(format_args!("<{0}>{1}</{0}>", cur_key, e_value))
        }
    }

    fn write_open(&mut self) -> io::Result<()> {
        self.destination.write_fmt(format_args!("{}\n<{}>", "<?xml version=\"1.0\" encoding=\"utf-8\"?>", "root"))
    }

    fn write_close(&mut self) -> io::Result<()> {
        self.destination.write_fmt(format_args!("</{}>", "root"))
    }

    fn write_begin(&mut self, _size: usize, cur_key: &str) -> io::Result<()> {
        self.destination.write_fmt(format_args!("<{}>", cur_key))
    }

    fn write_end(&mut self, _size: usize, cur_key: &str) -> io::Result<()> {
        self.destination.write_fmt(format_args!("</{}>", cur_key))
    }
}

pub struct JSON2XMLConsumer<W: Write, T: XMLWrite<W>> {
    pub states_stack: Vec<ParserToken>,
    pub keys_stack: Vec<String>,
    pub xml_write: T,
    phantom: PhantomData<W>,
}

impl<W: Write> JSON2XMLConsumer<W, FormattedTypedXMLWrite<W>> {
    pub fn new_formatted_and_typed(destination: W) -> JSON2XMLConsumer<W, FormattedTypedXMLWrite<W>> {
        JSON2XMLConsumer {
            xml_write: FormattedTypedXMLWrite { destination },
            states_stack: vec!(),
            keys_stack: vec!(),
            phantom: PhantomData,
        }
    }
}

impl<W: Write> JSON2XMLConsumer<W, FormattedXMLWrite<W>> {
    pub fn new_formatted(destination: W) -> JSON2XMLConsumer<W, FormattedXMLWrite<W>> {
        JSON2XMLConsumer {
            xml_write: FormattedXMLWrite { destination },
            states_stack: vec!(),
            keys_stack: vec!(),
            phantom: PhantomData,
        }
    }
}

impl<W: Write> JSON2XMLConsumer<W, TypedXMLWrite<W>> {
    pub fn new_typed(destination: W) -> JSON2XMLConsumer<W, TypedXMLWrite<W>> {
        JSON2XMLConsumer {
            xml_write: TypedXMLWrite { destination },
            states_stack: vec!(),
            keys_stack: vec!(),
            phantom: PhantomData,
        }
    }
}

impl<W: Write> JSON2XMLConsumer<W, RawXMLWrite<W>> {
    pub fn new_raw(destination: W) -> JSON2XMLConsumer<W, RawXMLWrite<W>> {
        JSON2XMLConsumer {
            xml_write: RawXMLWrite { destination },
            states_stack: vec!(),
            keys_stack: vec!(),
            phantom: PhantomData,
        }
    }
}

impl<W: Write, T: XMLWrite<W>> JSONParseConsumer for JSON2XMLConsumer<W, T> {
    fn consume(&mut self, token: Result<ParserToken, JSONParseError>) -> Result<(), ConsumeError> {
        let result = match token {
            Ok(BeginFile) => {
                self.xml_write.write_open()
            }
            Ok(EndFile) => {
                self.xml_write.write_close()
            }
            Ok(BeginObject) | Ok(BeginArray) => {
                let r = match self.states_stack.last() {
                    Some(BeginArray) => {
                        let cur_key = "li";
                        self.keys_stack.push(cur_key.into());
                        self.xml_write.write_begin(self.states_stack.len() * 4, cur_key)
                    }
                    Some(_) => {
                        let cur_key = self.keys_stack.last().unwrap();
                        self.xml_write.write_begin(self.states_stack.len() * 4, cur_key)
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
                        self.xml_write.write_end(self.states_stack.len() * 4, &previous_key)
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
                self.xml_write.write_value(self.states_stack.len() * 4, cur_key, "boolean", value)
            }
            Ok(NullValue) => {
                let cur_key = self.get_cur_key()?;
                self.xml_write.write_value(self.states_stack.len() * 4, cur_key, "null", String::from("null"))
            }
            Ok(StringValue(s)) => {
                let cur_key = self.get_cur_key()?;
                self.xml_write.write_string_value(self.states_stack.len() * 4, cur_key, s)
            }
            Ok(IntValue(s)) => {
                let cur_key = self.get_cur_key()?;
                self.xml_write.write_value(self.states_stack.len() * 4, cur_key, "int", s)
            }
            Ok(FloatValue(s)) => {
                let cur_key = self.get_cur_key()?;
                self.xml_write.write_value(self.states_stack.len() * 4, cur_key, "float", s)
            }
            Err(_) => { return Err(ConsumeError); }
        };
        match result {
            Ok(_) => { Ok(()) }
            Err(_) => { Err(ConsumeError) }
        }
    }
}


impl<W: Write, T: XMLWrite<W>> JSON2XMLConsumer<W, T> {
    fn get_cur_key(&mut self) -> Result<String, ConsumeError> {
        match self.states_stack.last() {
            Some(BeginArray) => { Ok("li".into()) }
            Some(_) => { Ok(self.keys_stack.pop().unwrap()) }
            None => { return Err(ConsumeError); }
        }
    }
}


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

use std::io::{Write};

use crate::json_lexer::ConsumeError;
use crate::json_parser::{JSONParseConsumer, JSONParseError, ParserToken};
use crate::json_parser::ParserToken::{BeginFile, BeginObject, EndObject, BeginArray, EndArray, Key, BooleanValue, StringValue, IntValue, FloatValue, NullValue};


pub struct JSON2XMLConsumer<W: Write> {
    pub destination: W,
    pub states_stack: Vec<ParserToken>,
    pub keys_stack: Vec<String>,
}

impl<W: Write> JSON2XMLConsumer<W> {
    pub fn new(destination: W) -> Self {
        return JSON2XMLConsumer {
            destination,
            states_stack: vec!(),
            keys_stack: vec!()
        };
    }
}

impl <W: Write> JSONParseConsumer for JSON2XMLConsumer<W> {
    fn consume(&mut self, token: Result<ParserToken, JSONParseError>) -> Result<(), ConsumeError> {
        let result = match token {
            Ok(BeginFile) => {
                self.destination.write_fmt(format_args!("{}\n", "<?xml version=\"1.0\" encoding=\"utf-8\"?>"))
            }
            Ok(BeginObject) | Ok(BeginArray) => {
                let r = match self.states_stack.last() {
                    Some(BeginArray) => {
                        let cur_key = "li";
                        self.destination.write_fmt(format_args!("{}{}", "", cur_key)) // spaces
                    }
                    Some(_) => {
                        let cur_key = self.keys_stack.last().unwrap();
                        self.destination.write_fmt(format_args!("{}<{}>", "", cur_key)) // spaces
                    }
                    None => { Ok(()) }
                };
                match r {
                    Err(_) => { return Err(ConsumeError) }
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
                        self.destination.write_fmt(format_args!("{}</{}>", "", previous_key)) // spaces
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
            Ok(t) => {
                let cur_key = match self.states_stack.last() {
                    Some(BeginArray) => { "li".into() }
                    Some(_) => { self.keys_stack.pop().unwrap() }
                    None => { return Err(ConsumeError); }
                };
                let value = match t {
                    BooleanValue(b) => { if b { "true".into() } else { "false".into() } }
                    NullValue => { "null".into() }
                    StringValue(s) => { s }
                    IntValue(s) => { s }
                    FloatValue(s) => { s}
                    _ => { return Err(ConsumeError); }
                };
                self.destination.write_fmt(format_args!("<{}>{}</{}>", cur_key, value, cur_key))

            }
            _ => { return Err(ConsumeError); }
        };
        match result {
            Ok(_) => { Ok(())}
            Err(_) => { Err(ConsumeError) }
        }
    }
}

fn _main() {

}
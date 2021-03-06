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

use std::{fs, io};
use std::io::{Write, ErrorKind};

use r_json_event_parser::byte_source::ByteSource;
use r_json_event_parser::json2xml::{JSON2XMLConsumer};
use r_json_event_parser::json_parser::JSONParser;

#[test]
fn lex_example1() {
    let path = "tests/files/example1.json";
    let expected = fs::read_to_string("tests/files/example1.xml").unwrap();
    let expected_argument = expected.as_str();
    let f = fs::File::open(path).expect("no file found");
    let read = f;
    let expected = expected_argument;
    let byte_source = ByteSource::new(read);
    let mut buf = [0u8; 1024*1024];
    let mut destination = BufWrite::new(&mut buf);
    let mut consumer= JSON2XMLConsumer::new_formatted_and_typed(&mut destination);
    let mut parser = JSONParser::new(byte_source, false);
    let _ = parser.parse(&mut consumer);
    assert_eq!(expected, destination.to_str());
}

#[test]
fn lex_example1_no_type() {
    let path = "tests/files/example1.json";
    let f = fs::File::open(path).expect("no file found");
    let byte_source = ByteSource::new(f);
    let mut buf = [0u8; 1024*1024];
    let mut destination = BufWrite::new(&mut buf);
    let mut consumer = JSON2XMLConsumer::new_formatted(&mut destination);
    let mut parser = JSONParser::new(byte_source, false);
    let _ = parser.parse(&mut consumer);
    assert_eq!(r#"<?xml version="1.0" encoding="utf-8"?>
<root>
    <glossary>
        <title>example glossary</title>
        <GlossDiv>
            <title>S</title>
            <GlossList>
                <GlossEntry>
                    <ID>SGML</ID>
                    <SortAs>SGML</SortAs>
                    <GlossTerm>Standard Generalized Markup Language</GlossTerm>
                    <Acronym>SGML</Acronym>
                    <Abbrev>ISO 8879:1986</Abbrev>
                    <GlossDef>
                        <para>A meta-markup language, used to create markup languages such as DocBook.</para>
                        <GlossSeeAlso>
                            <li>GML</li>
                            <li>XML</li>
                        </GlossSeeAlso>
                    </GlossDef>
                    <GlossSee>markup</GlossSee>
                </GlossEntry>
            </GlossList>
        </GlossDiv>
    </glossary>
</root>
"#, destination.to_str());
}

#[test]
fn lex_example1_no_format() {
    let path = "tests/files/example1.json";
    let f = fs::File::open(path).expect("no file found");
    let byte_source = ByteSource::new(f);
    let mut buf = [0u8; 1024*1024];
    let mut destination = BufWrite::new(&mut buf);
    let mut consumer = JSON2XMLConsumer::new_typed(&mut destination);
    let mut parser = JSONParser::new(byte_source, false);
    let _ = parser.parse(&mut consumer);
    assert_eq!(r#"<?xml version="1.0" encoding="utf-8"?>
<root><glossary><title type="string">example glossary</title><GlossDiv><title type="string">S</title><GlossList><GlossEntry><ID type="string">SGML</ID><SortAs type="string">SGML</SortAs><GlossTerm type="string">Standard Generalized Markup Language</GlossTerm><Acronym type="string">SGML</Acronym><Abbrev type="string">ISO 8879:1986</Abbrev><GlossDef><para type="string">A meta-markup language, used to create markup languages such as DocBook.</para><GlossSeeAlso><li type="string">GML</li><li type="string">XML</li></GlossSeeAlso></GlossDef><GlossSee type="string">markup</GlossSee></GlossEntry></GlossList></GlossDiv></glossary></root>"#, destination.to_str());
}

#[test]
fn lex_example1_no_format_no_type() {
    let path = "tests/files/example1.json";
    let f = fs::File::open(path).expect("no file found");
    let byte_source = ByteSource::new(f);
    let mut buf = [0u8; 1024*1024];
    let mut destination = BufWrite::new(&mut buf);
    let mut consumer = JSON2XMLConsumer::new(&mut destination);
    let mut parser = JSONParser::new(byte_source, false);
    let _ = parser.parse(&mut consumer);
    assert_eq!(r#"<?xml version="1.0" encoding="utf-8"?>
<root><glossary><title>example glossary</title><GlossDiv><title>S</title><GlossList><GlossEntry><ID>SGML</ID><SortAs>SGML</SortAs><GlossTerm>Standard Generalized Markup Language</GlossTerm><Acronym>SGML</Acronym><Abbrev>ISO 8879:1986</Abbrev><GlossDef><para>A meta-markup language, used to create markup languages such as DocBook.</para><GlossSeeAlso><li>GML</li><li>XML</li></GlossSeeAlso></GlossDef><GlossSee>markup</GlossSee></GlossEntry></GlossList></GlossDiv></glossary></root>"#, destination.to_str());
}

#[test]
fn lex_example2() {
    let path = "tests/files/example2.json";
    let expected = fs::read_to_string("tests/files/example2.xml").unwrap();
    let expected_argument = expected.as_str();
    let f = fs::File::open(path).expect("no file found");
    let read = f;
    let expected = expected_argument;
    let byte_source = ByteSource::new(read);
    let mut buf = [0u8; 1024*1024];
    let mut destination = BufWrite::new(&mut buf);
    let mut consumer = JSON2XMLConsumer::new_formatted_and_typed(&mut destination);
    let mut parser = JSONParser::new(byte_source, false);
    let _ = parser.parse(&mut consumer);
    assert_eq!(expected, destination.to_str());
}

#[test]
fn lex_example3() {
    let path = "tests/files/example3.json";
    let expected = fs::read_to_string("tests/files/example3.xml").unwrap();
    let expected_argument = expected.as_str();
    let f = fs::File::open(path).expect("no file found");
    let read = f;
    let expected = expected_argument;
    let byte_source = ByteSource::new(read);
    let mut buf = [0u8; 1024*1024];
    let mut destination = BufWrite::new(&mut buf);
    let mut consumer = JSON2XMLConsumer::new_formatted_and_typed(&mut destination);
    let mut parser = JSONParser::new(byte_source, false);
    let _ = parser.parse(&mut consumer);
    assert_eq!(expected, destination.to_str());
}

#[test]
fn lex_example4() {
    let path = "tests/files/example4.json";
    let expected = fs::read_to_string("tests/files/example4.xml").unwrap();
    let expected_argument = expected.as_str();
    let f = fs::File::open(path).expect("no file found");
    let read = f;
    let expected = expected_argument;
    let byte_source = ByteSource::new(read);
    let mut buf = [0u8; 1024*1024];
    let mut destination = BufWrite::new(&mut buf);
    let mut consumer = JSON2XMLConsumer::new_formatted_and_typed(&mut destination);
    let mut parser = JSONParser::new(byte_source, false);
    let _ = parser.parse(&mut consumer);
    assert_eq!(expected, destination.to_str());
}

#[test]
fn lex_example5() {
    let path = "tests/files/example5.json";
    let expected = fs::read_to_string("tests/files/example5.xml").unwrap();
    let expected_argument = expected.as_str();
    let f = fs::File::open(path).expect("no file found");
    let read = f;
    let expected = expected_argument;
    let byte_source = ByteSource::new(read);
    let mut buf = [0u8; 1024*1024];
    let mut destination = BufWrite::new(&mut buf);
    let mut consumer = JSON2XMLConsumer::new_formatted_and_typed(&mut destination);
    let mut parser = JSONParser::new(byte_source, false);
    let _ = parser.parse(&mut consumer);
    assert_eq!(expected, destination.to_str());
}

struct BufWrite<'a> {
    index: usize,
    buf: &'a mut [u8]
}

impl <'a> BufWrite<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        BufWrite {
            index: 0,
            buf
        }
    }

    pub fn to_str(&self) -> &str {
        std::str::from_utf8(&self.buf[..self.index]).unwrap()
    }
}

impl <'a> Write for &mut BufWrite<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.index >= self.buf.len() {
            return Err(io::Error::from(ErrorKind::OutOfMemory));
        }
        let n = (&mut self.buf[self.index..]).write(buf)?;
        self.index += n;
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}


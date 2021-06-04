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
use std::io::{Read, Write};

use r_json_event_parser::byte_source::ByteSource;
use r_json_event_parser::json2xml::JSON2XMLConsumer;
use r_json_event_parser::json_parser::JSONParser;

#[test]
fn lex_example1() {
    let path = "tests/files/example1.json";
    test_file(path, "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<glossary><title>example glossary</title><GlossDiv><title>S</title><GlossList><GlossEntry><ID>SGML</ID><SortAs>SGML</SortAs><GlossTerm>Standard Generalized Markup Language</GlossTerm><Acronym>SGML</Acronym><Abbrev>ISO 8879:1986</Abbrev><GlossDef><para>A meta-markup language, used to create markup languages such as DocBook.</para><GlossSeeAlso><li>GML</li><li>XML</li></GlossSeeAlso></GlossDef><GlossSee>markup</GlossSee></GlossEntry></GlossList></GlossDiv></glossary>");
}

fn test_file(path: &str, expected: &str) {
    let f = fs::File::open(path).expect("no file found");
    test_read(f, expected);
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
        let n = (&mut self.buf[self.index..]).write(buf)?;
        self.index += n;
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn test_read<R: Read>(read: R, expected: &str) {
    let byte_source = ByteSource::new(read);
    let mut buf = [0u8; 1024*1024];
    let mut destination = BufWrite::new(&mut buf);
    let mut consumer = JSON2XMLConsumer::new(&mut destination);
    let mut parser = JSONParser::new(byte_source);
    let _ = parser.parse(&mut consumer);
    assert_eq!(expected, destination.to_str());
}

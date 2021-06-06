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

use r_json_event_parser::byte_source::ByteSource;
use r_json_event_parser::json2xml::JSON2XMLConsumer;
use r_json_event_parser::json_parser::JSONParser;
use std::io::{Write, BufWriter};


fn main() {
    extern crate clap;
    use clap::{Arg, App};
    let matches = App::new("R-Json2XML")
        .version("0.0.1")
        .author("Julien Férard <github.com/jferard>")
        .about("Convert JSON file to XML")
        .arg(Arg::with_name("infile")
            .help("JSON file")
            .index(1))
        .arg(Arg::with_name("outfile")
            .help("XML file")
            .index(2))
        .get_matches();

    let inpath = matches.value_of("infile").unwrap_or("-");
    let outpath = matches.value_of("outfile").unwrap_or("-");

    let infile: Box<dyn io::Read> = if inpath == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(fs::File::open(inpath).expect("no file found"))
    };
    let outfile: Box<dyn io::Write> = if outpath == "-" {
        Box::new(BufWriter::new(io::stdout()))
    } else {
        Box::new(BufWriter::new(fs::File::create(outpath).expect("no file found")))
    };
    let byte_source = ByteSource::new(infile);
    let mut consumer = JSON2XMLConsumer::new_formatted_and_typed(outfile);
    let mut parser = JSONParser::new(byte_source, true);
    match parser.parse(&mut consumer) {
        Ok(_) => {}
        Err(e) => { io::stderr().write_fmt(format_args!("Err {:?}", e)).unwrap(); }
    }
}
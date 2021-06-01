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

pub struct ByteSource<R: Read> {
    source: R,
    unget_byte: Option<u8>,
    buffer: [u8; 32768],
    i: usize,
    limit: usize,
}

impl<R: Read> ByteSource<R> {
    pub fn new(source: R) -> Self {
        ByteSource {
            source,
            unget_byte: None,
            buffer: [0u8; 32 * 1024],
            i: 0,
            limit: 0,
        }
    }

    pub(crate) fn get(&mut self) -> Option<u8> {
        if let Some(b) = self.unget_byte {
            self.unget_byte = None;
            Some(b)
        } else {
            if self.i >= self.limit {
                self.i = 0;
                loop {
                    match self.source.read(&mut self.buffer[..]) {
                        Ok(0) => { return None; }
                        Ok(n) => {
                            self.limit = n;
                            break;
                        }
                        Err(_) => {} // retry
                    };
                }
            }
            let j = self.i;
            self.i += 1;
            Some(self.buffer[j])
        }
    }

    pub(crate) fn unget(&mut self) {
        self.unget_byte = Some(self.buffer[self.i-1]);
    }
}


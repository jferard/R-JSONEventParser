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
    buffer: [u8; 32769],
    i: usize,
    limit: usize,
}

impl<R: Read> ByteSource<R> {
    pub fn new(source: R) -> Self {
        ByteSource {
            source,
            buffer: [0u8; 32 * 1024 + 1],
            i: 1,
            limit: 1,
        }
    }

    pub(crate) fn get(&mut self) -> Option<u8> {
        if self.i >= self.limit {
            self.i = 1;
            self.buffer[0] = self.buffer[self.limit-1];
            loop {
                match self.source.read(&mut self.buffer[1..]) {
                    Ok(0) => { return None; }
                    Ok(n) => {
                        self.limit = n+1;
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

    pub(crate) fn unget(&mut self) {
        self.i -= 1;
    }
}


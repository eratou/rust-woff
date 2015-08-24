// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate woff;

use std::env;
use std::fs::File;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("usage: woffdec font.woff font.otf");
        return
    }

    let mut input = File::open(&args[1]).unwrap();
    let mut output = File::create(&args[2]).unwrap();
    woff::convert_woff_to_otf(&mut input, &mut output).unwrap()
}


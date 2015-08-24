/* Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */

extern crate woff;

use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};

#[test]
pub fn basic() {
    let mut input = File::open("testdata/opensans.woff").unwrap();
    let mut output = Cursor::new(Vec::new());
    woff::convert_woff_to_otf(&mut input, &mut output).unwrap();

    input.seek(SeekFrom::Start(0)).unwrap();
    output.seek(SeekFrom::Start(0)).unwrap();
    let mut expected_file = File::open("testdata/opensans.otf").unwrap();
    let mut expected = Vec::new();
    expected_file.read_to_end(&mut expected).unwrap();
    assert_eq!(output.into_inner(), expected);
}



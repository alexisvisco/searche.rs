use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str;

/// Fill the first time the internal buffer of BufReader and then look if the last char is a
/// newline.
///
/// If it has a newline it's fine it can return the chunk.
/// Else there is a second read until it find a newline character.
///
/// The result of this a either:
///    - None (this would signify that we got an EOF)
///    - A string which end surely with a end of line.
///    - Panic because of an unknown reason.
///
/// Why chunk need to end by a newline: this is because when you are computing a search across
/// multiple threads you can't just join the other missing part, because it will be in another
/// thread.
pub fn read(buffer_reader: &mut BufReader<&File>) -> Option<String> {
    let mut full_line = String::new();
    'outer: loop {

        if full_line.len() != 0 {
            return read_until_newline(buffer_reader, &mut full_line)
        }

        // First read
        match buffer_reader.fill_buf() {
            Ok(buffer) => {
                let buffer_str = String::from_utf8(buffer.to_vec()).unwrap_or(String::new());

                let buffer_str_len = buffer_str.len();
                buffer_reader.consume(buffer_str_len);

                // There is no more characters to read.
                if buffer_str_len == 0 {
                    return None;
                }

                full_line.push_str(buffer_str.as_str());

                if end_with_newline(buffer_str) {
                    return Some(full_line);
                } else {
                    // When there is no '\n' continue reading to find the next '\n'
                    continue 'outer;
                }
            }

            Err(err) => {
                panic!("bgrep: err: {}", err);
            }
        }
    }
}

fn read_until_newline(buffer_reader: &mut BufReader<&File>, full_line: &mut String) -> Option<String> {
    let mut buf = vec![];

    let read = buffer_reader.read_until(b'\n', &mut buf);
    if read.is_err() {
        panic!("bgrep: err: {}", read.err().unwrap());
    }

    match str::from_utf8(&buf) {
        Ok(s) => {
            return Some(format!("{}{}", full_line, s));
        }
        Err(err) => {
            panic!("bgrep: err: {}", err);
        }
    }
}

fn end_with_newline(buffer_str: String) -> bool {
    buffer_str.chars().last().unwrap_or(' ') == '\n'
}

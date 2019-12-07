use std::collections::HashMap;
use crate::searcher;
use std::io::{BufWriter, Write};


const BUFFER_SIZE: usize = 1 << 16;

/// Take all occurrences grouped by chunk and print them in order.
pub fn print(occurrences: &mut HashMap<usize, searcher::Occurrences>) {

    let mut buffer = BufWriter::with_capacity(BUFFER_SIZE, std::io::stdout());
    let mut keys : Vec<&usize> = occurrences.keys().into_iter().collect();

    // Ordering chunks by ASC.
    keys.sort();

    for key in keys {
        // Print the whole matched results highlighted.
        write!(buffer, "{}", occurrences.get(key).unwrap().bulk_lines);
    }

    if let Err(e) = buffer.flush() {
        panic!("bgrep: err: {} : unable to flush buffer", e)
    }
}

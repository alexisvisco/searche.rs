use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::mem;

mod searcher;
mod reader;
mod printer;

const BUFFER_SIZE: usize = 4096 * 20;

/// This program is a grep that dispatch the searcher process into different threads.
/// Then join the matched lines through a channel and finally print the lines.
fn main() {
    if args().len() < 3 {
        println!("bgrep: <file> <pattern>");
        return;
    }

    let filename = args().nth(1).unwrap();
    let search : String = args().nth(2).unwrap();
    let static_search_to_free: &'static str = string_to_static_str(search);

    let file = File::open(filename.as_str());

    match file {
        Ok(f) => search_in_file(&f, static_search_to_free),
        Err(err) => eprintln!("bgrep: {} : unable to read file {}", err, filename),
    }

    mem::forget(static_search_to_free);
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}


fn search_in_file(file: &File, search: &'static str) {
    let mut buffer_reader = BufReader::with_capacity(BUFFER_SIZE, file);
    let mut chunk_index = 0;

    let (tx, rx) = mpsc::channel::<searcher::Occurrences>();

    loop {
        let buffer = reader::read(&mut buffer_reader);

        match buffer {
            None => break,
            Some(buffer) => searcher::search(tx.clone(), buffer, search, chunk_index),
        }

        chunk_index += 1;
    }

    drop(tx);

    let mut occurrences: HashMap<usize, searcher::Occurrences> = HashMap::new();
    for occurrence in rx {
        occurrences.insert(occurrence.chunk_index, occurrence);
    }

    printer::print(&mut occurrences)
}


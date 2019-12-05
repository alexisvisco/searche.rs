use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use crate::searcher::Occurrences;
use std::future::Future;
use futures::executor::block_on;
use futures::future::join_all;

mod searcher;
mod reader;
mod printer;

const BUFFER_SIZE: usize = 4096 * 10;

/// This program is a grep that dispatch the searcher process into different threads.
/// Then join the matched lines through a channel and finally print the lines.
fn main() {
    if args().len() < 3 {
        println!("bgrep: <file> <pattern>");
        return;
    }

    let filename = args().nth(1).unwrap();
    let search = args().nth(2).unwrap();

    let file = File::open(filename.as_str());

    match file {
        Ok(f) => block_on(search_in_file(&f, search)),
        Err(err) => eprintln!("bgrep: {} : unable to read file {}", err, filename),
    }

}


async fn search_in_file(file: &File, search: String) {
    let mut buffer_reader = BufReader::with_capacity(BUFFER_SIZE, file);
    let mut chunk_index = 0;

    let mut futures_vec = Vec::new();

    loop {
        let buffer = reader::read(&mut buffer_reader);

        match buffer {
            None => break,
            Some(buffer) => futures_vec.push(searcher::search(buffer, search.clone(), chunk_index)),
        }

        chunk_index += 1;
    }


    let mut occurrences: HashMap<usize, searcher::Occurrences> = HashMap::new();
    let result = join_all(futures_vec).await;

    for may_occurrence in result {
        if may_occurrence.is_some() {
            let occurrence = may_occurrence.unwrap();
            occurrences.insert(occurrence.chunk_index, occurrence);
        }
    }

    printer::print(&mut occurrences);
}


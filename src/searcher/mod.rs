use std::sync::mpsc;
use std::collections::HashSet;

const BOLD_RED: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0;0m";

#[derive(Debug)]
pub struct Occurrences {
    pub chunk_index: usize,
    pub bulk_lines: String,
}

pub fn search(
    pool_of_thread: &rayon::ThreadPool,
    producer: mpsc::Sender<Occurrences>,
    chunk: String,
    search: &'static str,
    chunk_index: usize,
) {
    pool_of_thread.spawn(move || {
        let positions = get_positions(&chunk, search);
        let mut lines = chunk.split('\n');

        let mut line = lines.next().unwrap_or("");

        let mut total_chars = 0;
        let mut line_index = 0;

        let mut bulk_lines: String = String::new();
        let mut cache_bulk_line = HashSet::new();

        'positions_loop: for pos in positions {
            loop {
                if pos >= total_chars && pos <= total_chars + (line.len()) {
                    if cache_bulk_line.contains(&line_index) {
                        continue 'positions_loop;
                    }

                    cache_bulk_line.insert(line_index.clone());

                    bulk_lines.push_str(format!("{}{}", line, '\n').as_str());
                    continue 'positions_loop;
                }

                total_chars += line.len() + 1;
                line_index += 1;

                match lines.next() {
                    Some(l) => line = l,
                    None => break 'positions_loop
                }
            }
        }

//        // It's the red underscore you're used to with grep.
//        let replacer = format!("{}{}{}", BOLD_RED, search, RESET);
//        let replacer_str = replacer.as_str();

        if bulk_lines.len() > 0 {
            producer
                .send(Occurrences {
                    chunk_index,
                    bulk_lines,
                })
                .unwrap();
        }

        drop(producer)
    });
}

/// Return the list of match occurrences found in a text chunk.
fn get_positions(chunk: &String, search: &str) -> Vec<usize> {
    let mut positions = chunk
        .match_indices(search)
        .collect::<Vec<(usize, &str)>>()
        .iter()
        .fold(Vec::new(), |mut p, (position, _)| {
            p.push(*position);
            p
        });

    positions.sort();

    positions
}

use std::collections::HashMap;
use crate::searcher;

/// Take all occurrences grouped by chunk and print them in order.
pub fn print(occurrences: &mut HashMap<usize, searcher::Occurrences>) {

    let mut keys : Vec<&usize> = occurrences.keys().into_iter().collect();

    // Ordering chunks by ASC.
    keys.sort();

    for key in keys {
        // Print the whole matched results highlighted.
        print!("{}", occurrences.get(key).unwrap().bulk_lines)
    }
}

// A script to determine if s1 exists in s2 in any of it's (s1's) permutations
// ie. s1 = 'xkcd', s2 = 'hello dkcx world' would be true as 'dkcx' is a permutation of 'xkcd'
use std::collections::HashMap;

fn hash_counts(s1: Vec<char>) -> HashMap<char, i32> {
    // create an empty hashmap/dict to store letters and counts
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in s1.into_iter() {
        let letter_count = map.entry(c).or_insert(0);
        *letter_count += 1
    }
    map
}

fn comparison() -> bool {
    let s1 = "abc";
    let s2 = "asdcabjk";
    let s1_letters: Vec<char> = s1.chars().collect();
    let s2_letters: Vec<char> = s2.chars().collect();
    let target = hash_counts(s1_letters);

    let (mut window_start, mut window_end): (usize, usize) = (0, s1.len() - 1);

    let mut comparison_map: HashMap<char, i32> = hash_counts(s2_letters[window_start..window_end].to_vec()); 

    while window_start < s2.len() - window_end {
        if comparison_map.eq(&target) {
            return true
        }

        window_start += 1;
        window_end += 1;
    }
    false
}

fn main() {
    comparison();
}
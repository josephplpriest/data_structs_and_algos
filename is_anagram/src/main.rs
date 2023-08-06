

// A script for checking if a given string is an anagram or not
fn is_anagram(string: &str) -> bool {
    let (mut a, mut b): (usize, usize) = (0, (string.len() - 1));

    let char_vec: Vec<char> = string.chars().collect();
    println!("Checking if word {} is an anagram.", string);

    while a <= b {
        
        println!("Comparing {} and {}", char_vec[a], char_vec[b]);
        if char_vec[a] != char_vec[b] {
            println!("False: Not an Anagram");
            return false
        }
        *&mut a += 1;
        *&mut b -= 1;
    }
    println!("True: Is an Anagram");
    true
}

fn main() {
    is_anagram("hello");
    is_anagram("bookkeeperrepeekkoob");
    is_anagram("darioirad");
}

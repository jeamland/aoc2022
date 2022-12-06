use std::collections::HashSet;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1]).unwrap();
    let input: Vec<char> = input.trim_end().chars().collect();

    for pos in 0..input.len() - 13 {
        let chars: HashSet<char> = input[pos..pos + 14].iter().cloned().collect();
        if chars.len() == 14 {
            println!("start-of-message marker at {}", pos + 14);
            break;
        }
    }
}

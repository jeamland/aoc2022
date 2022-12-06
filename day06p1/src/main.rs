use itertools::Itertools;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1]).unwrap();
    let input = input.trim_end();

    for (pos, (a, b, c, d)) in input.chars().tuple_windows().enumerate() {
        println!("{pos:05} - {a}{b}{c}{d}");
        if a != b && a != c && a != d && b != c && b != d && c != d {
            println!("start-of-packet marker at {}", pos + 4);
            break;
        }
    }
}

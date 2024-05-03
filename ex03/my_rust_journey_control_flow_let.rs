use std::env;

fn main() {
    match env::args().nth(1) {
        Some(argument) => {
            let mut chars_iter = argument.chars();

            while let Some(char) = chars_iter.next() {
                println!("{}", char);
            }
        }
        None => {
            return;
        }
    }
}

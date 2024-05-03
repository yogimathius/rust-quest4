use std::env;
use std::panic;

fn is_it_a_good_ide(ide: &str) {
    let good_ide = ["vscode", "cloud9", "docode", "emacs", "nano"];
    let not_good = ["vim", "atom"];
    let is_good = good_ide.contains(&ide);
    let is_not_good = not_good.contains(&ide);

    match is_good {
        true => println!("Good"),
        false => match is_not_good {
            true => println!("Not good"),
            false => panic!("{} is not in the list", ide),
        },
    }
}

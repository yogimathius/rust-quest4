fn list_of_plastic(plastic: &str) -> Option<&str> {
    match plastic {
        Some(plastic) => println!("{} is a plastic", plastic),
        None => println!("Not found or not a plastic"),
    }
}

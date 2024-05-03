fn list_of_plastic(plastic: &str) -> Option<&str> {
    let list_plastics = [
        "Polyethylene Terephthalate",
        "High-Density Polyethylene",
        "Polyvinyl Chloride",
        "Low-Density Polyethylene",
        "Polypropylene",
        "Polystyrene",
        "Polycarbonate",
    ];

    if list_plastics.contains(&plastic) {
        Some(plastic)
    } else {
        None
    }
}

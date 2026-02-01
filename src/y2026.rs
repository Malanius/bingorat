use crate::app::{Bingo, cell::Cell};

pub fn get_year_predictions() -> Bingo {
    let cells = vec![
        Cell::new("Survelliance enforced on HW level"),
        Cell::new("Chat Control gets mandatory"),
        Cell::new("UK doubles down on full 1984 and tracks everyone everywhere"),
        Cell::new("Trump declares US independence, leaves NATO and UN"),
        Cell::new("Unexpected backup of major video streaming service"),
        Cell::new("Major game studio officially supports Linux first"),
        Cell::new("GPU prices hit over $10k"),
        Cell::new("Trump gets Kirked"),
        Cell::new("Military conflict triggered by satellite malfunction"),
        Cell::new("Country admits election results were AI-assisted"),
        Cell::new("Disney+ cancelled due to AI slop"),
        Cell::new("WW3 starts"),
        Cell::new("FREE"),
        Cell::new("City loses power for multiple day due to software update"),
        Cell::new("Squadron 42 release"),
        Cell::new("Nuclear weapon gets lost"),
        Cell::new("A typo causes billions in damage"),
        Cell::new("Unpatchable vulnerability found in critical software"),
        Cell::new("Windows 11 flops, 2026 true year of Linux"),
        Cell::new("New pandemic sweeps the world"),
        Cell::new("Social media introduces GLOBAL mandatory age verification"),
        Cell::new("AI-generated evidence is accepted in court"),
        Cell::new("Country replaces entire goverment with AI"),
        Cell::new("Half-Life 3 announced"),
        Cell::new("China 'temporarily safeguards' Russian strategic assets"),
    ];

    Bingo::new(
        "Bingo of 2026",
        "What a fucked up year, huh?",
        5,
        cells,
        (2, 2),
    )
}

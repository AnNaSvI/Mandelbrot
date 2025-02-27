//use std::fs;
//use std::env;
use std::num::ParseIntError;
use crate::image::Image; 


pub fn parse_args() -> Result<(usize, usize, usize), ParseIntError> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <width> <height> <max_iterations>", args[0]);
        std::process::exit(1);
    }

    match (
        args[1].parse(),
        args[2].parse(),
        args[3].parse(),
    ) {
        (Ok(width), Ok(height), Ok(max_iterations)) => Ok((width, height, max_iterations)),
        _ => {
            eprintln!("Error: Failed to parse one or more arguments. Ensure all arguments are positive integers.");
            std::process::exit(1);
        }
    }
}





pub fn save_to_file(image: &Image, filename: &str) {
    
    let mut output = format!("P3\n{} {}\n255\n", image.width, image.height);

    
    for y in 0..image.height {
        for x in 0..image.width {
            if let Some(pixel) = image.get(x, y) {
                output.push_str(&format!("{}\n", pixel));
            } else {
                eprintln!("Warning: Pixel at ({}, {}) is missing!", x, y);
                output.push_str("0 0 0\n"); // Platzhalter für fehlende Pixel
            }
        }
    }

    
    if let Err(err) = std::fs::write(filename, output) {
        eprintln!("Error writing to disk: {}", err);
        std::process::exit(1);
    }
}

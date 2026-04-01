use std::io;

use crate::cli::{new_astro, new_nodejs, new_react, new_svelte};
mod cli;

fn main() {
    let options = [
        "1. Nodejs Vacío",
        "2. Proyecto de react",
        "3. Proyecto de svelte",
        "4. Proyecto de Astro",
    ];

    println!("\n\nBienvenido a tu starter de proyectos de CLI favorito!!!");
    for x in options {
        println!("{x}");
    }

    let option_idx: u8 = loop {
        let mut input_line = String::new();

        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        match input_line.trim().parse::<u8>() {
            // Ok(result) if (result >= 1 && result <= 5) break result,
            Ok(result) if result <= 4 && result >= 1 => break result,
            Ok(_) => println!("El número debe estar entre 1 y 4"),
            Err(_) => {
                println!("Introduce un número válido");
                continue;
            }
        }
    };

    println!("the result is: {}", option_idx);

    // cli::new_nodejs();
    // cli::new_react();
    // cli::new_svelte();
    // cli::new_astro();

    match option_idx {
        1 => new_nodejs(),
        2 => new_react(),
        3 => new_svelte(),
        4 => new_astro(),
        _ => {
            panic!("wtf bro this ain't any shi");
        }
    }
}

use dialoguer::{Input, Select, theme::ColorfulTheme};
use regex::Regex;
mod cli;

fn main() {
    let options = &[
        "Nodejs template",
        "React Template",
        "Svelte template",
        "Astro template",
    ];

    println!("\n\nBienvenido a tu starter de proyectos de CLI favorito!!!");
    // for x in options {
    //     println!("{x}");
    // }

    // let option_idx: u8 = loop {
    //     let mut input_line = String::new();

    //     io::stdin()
    //         .read_line(&mut input_line)
    //         .expect("Failed to read line");

    //     match input_line.trim().parse::<u8>() {
    //         // Ok(result) if (result >= 1 && result <= 5) break result,
    //         Ok(result) if result <= 4 && result >= 1 => break result,
    //         Ok(_) => println!("El número debe estar entre 1 y 4"),
    //         Err(_) => {
    //             println!("Introduce un número válido");
    //             continue;
    //         }
    //     }
    // };

    let re = Regex::new(r"^[a-zA-Z0-9_-]*$").unwrap();

    let mut proj_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Introduce the name of your new big project!")
        .allow_empty(true)
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() || re.is_match(input) {
                Ok(())
            } else {
                Err("El nombre solo puede contener letras, números, - o _")
            }
        })
        .interact_text()
        .unwrap();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a project to start")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    println!("Let's start our {}!", options[selection]);

    let option_idx = selection as u8;

    if proj_name.trim().is_empty() {
        proj_name = match option_idx {
            0 => "nodeJS_template".to_string(),
            1 => "React_template".to_string(),
            2 => "Svelte_template".to_string(),
            3 => "Astro_template".to_string(),
            _ => panic!("Opción inválida"),
        }
    }

    match option_idx {
        0 => cli::new_nodejs(Some(&proj_name)),
        1 => cli::new_react(Some(&proj_name)),
        2 => cli::new_svelte(Some(&proj_name)),
        3 => cli::new_astro(Some(&proj_name)),
        _ => {
            panic!("wtf bro this ain't any shi");
        }
    }
}

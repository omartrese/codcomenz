use dialoguer::{Select, theme::ColorfulTheme};
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

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a project to start")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    println!("Let's start our {}!", options[selection]);

    let option_idx = selection as u8;

    println!("{}", option_idx);
    match option_idx {
        0 => cli::new_nodejs(),
        1 => cli::new_react(),
        2 => cli::new_svelte(),
        3 => cli::new_astro(),
        _ => {
            panic!("wtf bro this ain't any shi");
        }
    }
}

use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

pub fn new_nodejs(dir_name: Option<&str>) {
    let proj_name = dir_name.unwrap_or("nodeJS_template");
    fs::create_dir(format!("./{}", proj_name)).expect("fallo al intentar crear directorio");

    let mut script =
        File::create(format!("./{}/index.js", proj_name)).expect("No se pudo crear el archivo");

    script
        .write_all(
            b"console.log('bienvenido desde rust');
console.log('4 + 6 equivale a ' + sumar(4,6));

function sumar(a, b) {
    return a + b;
}
        ",
        )
        .expect("No se pudo escribir en el archivo");

    Command::new("npm")
        .arg("init")
        .arg("-y")
        .current_dir(format!("./{}", proj_name))
        .output()
        .expect("Failed to execute command");

    // println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn new_react(dir_name: Option<&str>) {
    let proj_name = dir_name.unwrap_or("React_template");

    let output = Command::new("npm")
        .arg("create")
        .arg("vite@latest")
        .arg(proj_name)
        .arg("--")
        .arg("--template")
        .arg("react-ts")
        .output()
        .expect("Failed to execute command");

    // println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn new_svelte(dir_name: Option<&str>) {
    let proj_name = dir_name.unwrap_or("Svelte_template");

    let output = Command::new("npm")
        .arg("create")
        .arg("vite@latest")
        .arg(proj_name)
        .arg("--")
        .arg("--template")
        .arg("svelte-ts")
        .output()
        .expect("Failed to execute command");

    // println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn new_astro(dir_name: Option<&str>) {
    let proj_name = dir_name.unwrap_or("Astro_template");

    let output = Command::new("npm")
        .arg("create")
        .arg("astro@latest")
        .arg(proj_name)
        .arg("--")
        .arg("--template")
        .arg("minimal")
        .status()
        .expect("Failed to execute command");
}

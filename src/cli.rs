use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

pub fn new_nodejs() {
    fs::create_dir("./nodeJS_template").expect("fallo al intentar crear directorio");

    let mut script =
        File::create("./nodeJS_template/index.js").expect("No se pudo crear el archivo");

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
        .current_dir("./nodeJS_template")
        .output()
        .expect("Failed to execute command");

    // println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn new_react() {
    let output = Command::new("npm")
        .arg("create")
        .arg("vite@latest")
        .arg("reactJS_template")
        .arg("--")
        .arg("--template")
        .arg("react-ts")
        .output()
        .expect("Failed to execute command");

    // println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn new_svelte() {
    let output = Command::new("npm")
        .arg("create")
        .arg("vite@latest")
        .arg("svelte_template")
        .arg("--")
        .arg("--template")
        .arg("svelte-ts")
        .output()
        .expect("Failed to execute command");

    // println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn new_astro() {
    let output = Command::new("npm")
        .arg("create")
        .arg("astro@latest")
        .arg("astro_template")
        .arg("--")
        .arg("--template")
        .arg("minimal")
        .status()
        .expect("Failed to execute command");
}

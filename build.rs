use std::{env, path::Path, fs};

use const_gen::{const_declaration, CompileConst};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=res/");

    // Use the OUT_DIR environment variable to get an
    // appropriate path.
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("const_images.rs");

    let const_declarations = vec![
        // And here are constant definitions for particular
        // values.
        const_declaration!(IMAGE_OFF = process_image("sun-off.png")),
        const_declaration!(IMAGE_READY = process_image("sun-ready.png")),
        const_declaration!(IMAGE_ACTIVE = process_image("sun-active.png")),
    ]
    .join("\n");


    fs::write(&dest_path, const_declarations).unwrap();
    let a = fs::read_to_string(&dest_path).unwrap();
    println!("{}:{}", a.len(), a);
}

fn process_image(filename: &str) -> Vec<u8> {
    let path = env::current_dir()
        .unwrap()
        .join(format!("res/{}", filename));
    let dyn_image = image::open(&path).expect(&format!("Could not find '{filename}'."));
    let image = dyn_image
        .as_rgba8()
        .expect(&format!("Could not convert '{filename}' to rgba8."));

    if image.width() != image.height() {
        panic!("Image is not square!");
    }

    if (image.width() % 4) != 0 {
        panic!("Image dimensions not divisible by four.")
    }

    let mut pixels: Vec<u8> = Vec::new();
    for pix in image.pixels() {
        pixels.push(pix.0[3]);
        pixels.push(pix.0[0]);
        pixels.push(pix.0[1]);
        pixels.push(pix.0[2]);
    }

    pixels
}

use std::{env, path::Path, fs};

use const_gen::{const_declaration, CompileConst};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=res/");

    generate_icon_consts();
}

/// Reads the image files for the icons and encodes them as a constant.
fn generate_icon_consts() {
    // Use the OUT_DIR environment variable to get an
    // appropriate path.
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("const_images.rs");
    let image_data = vec![
        const_declaration!(IMAGE_OFF = process_image("sun-off.png")),
        const_declaration!(IMAGE_READY = process_image("sun-ready.png")),
        const_declaration!(IMAGE_ACTIVE = process_image("sun-active.png")),
    ]
    .join("\n");
    fs::write(&dest_path, image_data).unwrap();
}

/// Read a image in and encodes it as a ARGB32 bytes.
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

    // Change it from RGBA32 to ARGB32
    let mut pixels: Vec<u8> = Vec::new();
    for pix in image.pixels() {
        pixels.push(pix.0[3]);
        pixels.push(pix.0[0]);
        pixels.push(pix.0[1]);
        pixels.push(pix.0[2]);
    }

    pixels
}

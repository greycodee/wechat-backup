use std::path::Path;

fn main() {
    let mut files = Vec::new();
    recursion(&mut files, "silk/interface").unwrap();
    recursion(&mut files, "silk/src").unwrap();
    println!("cargo:rustc-link-lib=static=silk");

    cc::Build::new()
        .includes(["silk/src", "silk/interface"])
        .files(files)
        .compile("silk");

    let bindings = bindgen::Builder::default()
        .header("silk/include/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}

fn recursion<P: AsRef<Path>>(v: &mut Vec<String>, dir: P) -> std::io::Result<()> {
    let rd = std::fs::read_dir(dir)?;
    for x in rd {
        let de = x?;
        let path = de.path();
        if path.is_dir() {
            recursion(v, path)?;
        } else {
            let path = path.into_os_string().into_string().unwrap();
            if path.ends_with(".c") {
                v.push(path);
            }
        }
    }
    Ok(())
}

use std::path::Path;

const BUILD_DIR: &str = "build";
const ASSETS_DIR: &str = "static";

fn main() {
    println!("cargo:rerun-if-changed={}/", ASSETS_DIR);
    println!("cargo:rerun-if-changed=src/");
    if let (Ok(target), Ok(host), Ok(profile)) = (
        std::env::var("TARGET"),
        std::env::var("HOST"),
        std::env::var("PROFILE"),
    ) {
        let mut output_directory = String::from("target/");
        if target != host {
            output_directory.push_str(format!("{}/", target).as_str())
        }
        output_directory.push_str(format!("{}/", profile).as_str());

        let build_path = Path::new(BUILD_DIR);
        match std::fs::remove_dir_all(build_path) {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => panic!("Could not remove directory: {}", e),
        }

        std::fs::create_dir_all(build_path)
            .unwrap_or_else(|_| panic!("Unable to create {} directory", BUILD_DIR));
        copy_dir_recursively(Path::new(ASSETS_DIR), build_path).unwrap_or_else(|_| {
            panic!("Could not copy assets from {} to {}", ASSETS_DIR, BUILD_DIR)
        });
        if let Ok(pkg_name) = std::env::var("CARGO_PKG_NAME") {
            let output_bin_path = format!("{}{}", output_directory, pkg_name.clone());
            std::fs::copy(output_bin_path, format!("{}/{}", BUILD_DIR, pkg_name))
                .expect("Could not copy binary to build directory");
        } else {
            panic!("Unable to get CARGO_PKG_NAME");
        }
    } else {
        panic!("Unable to get TARGET, HOST, or PROFILE value");
    }
}

fn copy_dir_recursively(source: &Path, destination: &Path) -> std::io::Result<()> {
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = destination.join(path.file_name().unwrap());

        if path.is_dir() {
            std::fs::create_dir_all(dest_path.clone())?;
            copy_dir_recursively(&path, &dest_path)?;
        } else {
            std::fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

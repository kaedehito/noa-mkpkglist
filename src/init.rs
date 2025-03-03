use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use crate::{info, warn};

#[allow(dead_code)]
const NOA_PKGLIST: &[u8] = include_bytes!("../noa_pkglist.default.toml");

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let toml_path = Path::new("./.noa-pkglist.toml");

    if toml_path.exists() {
        warn!(
            "{} is alredy exists! rewrite .noa-pkglist.toml?",
            toml_path.display()
        );
        let mut input = String::new();
        print!("[y/n] ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        if matches!(input.trim(), "y" | "yes" | "") {
            let mut file = fs::File::create(toml_path)?;
            file.write_all(NOA_PKGLIST)?;
            info!("Created {}", toml_path.display());
        }
    } else {
        let mut file = fs::File::create(".noa-pkglist.toml")?;
        file.write_all(NOA_PKGLIST)?;
        info!("Created {}", toml_path.display());
    }

    let pkglist_path = Path::new("./noa");
    if pkglist_path.exists() {
        warn!(
            "{} is alredy exists! recreate ./noa?",
            pkglist_path.display()
        );
        let mut input = String::new();
        print!("[y/n] ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        if matches!(input.trim(), "y" | "yes" | "") {
            fs::remove_dir_all("./noa/")?;
            fs::create_dir_all("./noa/package-list/")?;
            info!("Created {}", pkglist_path.display());
        }
    } else {
        fs::create_dir_all("./noa/package-list/")?;
        info!("Created {}", pkglist_path.display());
    }

    Ok(())
}

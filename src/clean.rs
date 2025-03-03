use std::{fs, io};

use crate::{err, info};

pub fn clean() {
    err!("Do you really want to delete all templates?");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    if !matches!(buf.trim(), "y" | "yes" | "") {
        info!("Clean canceled");
        return;
    }

    info!("Cleaning ./out...");
    fs::remove_dir_all("./out").unwrap_or_else(|e| {
        err!("Failed to remove ./out: {e}");
    });
    info!("Cleaning ./package-list...");
    fs::remove_dir_all("./package-list").unwrap_or_else(|e| {
        err!("Failed to remove ./noa: {e}");
    });

    info!("Clean completed successfully!");
    return;
}

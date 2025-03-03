use blake3::Hasher;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use tar::Builder;
use xz2::write::XzEncoder;

use crate::info;

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    let outdir = Path::new("./out/");
    if !outdir.exists() {
        fs::create_dir_all("./out/")?;
    }

    let outfile = fs::File::create("./out/packagelist.tar.xz")?;

    let buf_writer = BufWriter::new(outfile);

    let xz_encoder = XzEncoder::new(buf_writer, 6);

    let mut tar_builder = Builder::new(xz_encoder);

    let packagelist = Path::new("./noa/package-list/");

    for n in packagelist.read_dir()? {
        let path = n?;
        info!("Compressing {}...", path.path().display());
        tar_builder.append_path(path.path())?;
    }

    tar_builder.into_inner()?.finish()?;
    info!("Archive created at ./out/packagelist.tar.xz!");

    let mut hasher = Hasher::new();
    let mut tar_reader = BufReader::new(File::open("./out/packagelist.tar.xz")?);
    let mut buffer = [0; 8192];

    info!("Generating BLAKE3 hash...");
    while let Ok(n) = tar_reader.read(&mut buffer) {
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let hash = hasher.finalize();

    info!("Generated BLEAKE3 hash: {}", hash.to_hex());

    let mut hash_file = fs::File::create("./out/packagelist.hash")?;
    hash_file.write_all(hash.to_string().as_bytes())?;
    info!("Write to ./out/packagelist.hash");

    info!("Finish!");

    Ok(())
}

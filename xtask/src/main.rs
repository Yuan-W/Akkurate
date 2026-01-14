use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use xshell::{cmd, Shell};

#[derive(Parser)]
#[command(name = "xtask")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build and package the Chrome Extension
    Dist,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Dist => dist()?,
    }

    Ok(())
}

fn dist() -> Result<()> {
    let sh = Shell::new()?;
    let project_root = project_root();
    sh.change_dir(&project_root);

    println!("📦 Building Akkurate Extension...");

    // Ensure wasm-pack is installed
    if cmd!(sh, "wasm-pack --version").read().is_err() {
        println!("⚠️  wasm-pack not found. Installing...");
        cmd!(sh, "cargo install wasm-pack").run()?;
    }

    let extension_dir = project_root.join("extension");
    let assets_dir = extension_dir.join("assets");
    let pkg_dir = extension_dir.join("pkg");

    // Clean previous build
    if pkg_dir.exists() {
        sh.remove_path(&pkg_dir)?;
    }

    // Build WASM
    // We explicitly set target check to avoid rustup issues if user has environment issues,
    // but assuming user has fixed it or running where it works.
    // Use --no-typescript to keep it simple if not needed
    {
        let _guard = sh.push_dir(&extension_dir);
        cmd!(sh, "wasm-pack build --target web --out-dir pkg").run()?;
    }

    println!("📂 Copying assets...");
    for file in [
        "manifest.json",
        "content.js",
        "background.js",
        "popup.html",
        "popup.js",
        "style.css",
    ] {
        sh.copy_file(assets_dir.join(file), &pkg_dir)?;
    }

    // Handle icon: prefer asset icon, fallback to generated minimal PNG
    let asset_icon = assets_dir.join("icon.png");
    let pkg_icon = pkg_dir.join("icon.png");

    if asset_icon.exists() {
        sh.copy_file(&asset_icon, &pkg_dir)?;
    } else if !pkg_icon.exists() {
        println!("⚠️  icon.png not found in assets, generating placeholder...");
        let png_data: [u8; 67] = [
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00,
            0x00, 0x1F, 0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78,
            0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00,
            0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ];
        sh.write_file(&pkg_icon, &png_data)?;
    }

    println!("🤐 Creating zip archive...");
    let zip_file = std::fs::File::create(project_root.join("akkurate-extension.zip"))?;
    let mut zip = zip::ZipWriter::new(zip_file);
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // We don't have walkdir dependency in xtask cargo.toml?

    // For now, let's list them explicitly or re-implement recursion to avoid another dep if possible?
    // No, better to be robust. I'll add walkdir to Cargo.toml too.
    let mut buffer = Vec::new();
    for entry in walkdir::WalkDir::new(&pkg_dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let name = path.strip_prefix(&pkg_dir)?.to_str().unwrap();

            // On Windows, paths might have backslashes, zip spec wants forward slashes.
            // But let's assume 'zip' crate handles or we convert?
            // Usually standard is forward slash.
            let name = name.replace('\\', "/");

            zip.start_file(name, options)?;
            use std::io::Read;
            let mut f = std::fs::File::open(path)?;
            f.read_to_end(&mut buffer)?;
            use std::io::Write;
            zip.write_all(&buffer)?;
            buffer.clear();
        }
    }
    zip.finish()?;

    println!("🎉 Packaged: akkurate-extension.zip");

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

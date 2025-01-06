use std::fs;
use std::path::Path;
use std::process::Command;
use temp_dir::TempDir;

const FONT_URL: &str =
    "https://github.com/adobe-fonts/source-han-sans/releases/download/2.004R/SourceHanSansCN.zip";
const FONT_PATH: &str = "assets/fonts/SourceHanSansCN-Regular.otf";
const LICENSE_PATH: &str = "assets/fonts/LICENSE.txt";

fn main() {
    if Path::new(FONT_PATH).exists() {
        return;
    }
    let d = TempDir::new().unwrap();
    let download_file = d.path().join("font.zip");
    println!("cargo::warning=using temp dir {:?}", d.path());
    println!("cargo::warning=downloading font with `curl`...");
    Command::new("curl")
        .args(["-L", FONT_URL, "-o", download_file.to_str().unwrap()])
        .status()
        .expect("Failed to download font");
    println!("cargo::warning=extracting font with `unzip`...");
    Command::new("unzip")
        .args([
            download_file.to_str().unwrap(),
            "-d",
            d.path().to_str().unwrap(),
        ])
        .status()
        .expect("Failed to extract font");
    fs::copy(
        d.path().join("SubsetOTF/CN/SourceHanSansCN-Regular.otf"),
        FONT_PATH,
    )
    .expect("Failed to move font");
    fs::copy(d.path().join("LICENSE.txt"), LICENSE_PATH).expect("Failed to move license");
}

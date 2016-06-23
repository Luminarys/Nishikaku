extern crate flate2;
extern crate tar;

use std::process::Command;
use std::env;
use std::fs::rename;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() {
    let target = env::var("TARGET").unwrap(); 
    if target == "x86_64-pc-windows-msvc"{
        download_files();
        rename("target\\Nishikaku-Bin-0.1.0\\x86_64\\dll", "target\\win_rel").unwrap();
    } else if target == "i686-pc-windows-msvc" {
        download_files();
        rename("target\\Nishikaku-Bin-0.1.0\\i686\\dll", "target\\win_rel").unwrap();
    }
}

fn download_files() {
    let output = Command::new("curl").arg("-s").arg("-L").arg("https://github.com/Luminarys/Nishikaku-Bin/archive/0.1.0.tar.gz")
       .output()
       .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    let resp = output.stdout;
    let d = GzDecoder::new(&resp[..]).unwrap();
    let mut a = Archive::new(d);
    a.unpack("./target/").unwrap();
}

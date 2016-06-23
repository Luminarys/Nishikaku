extern crate flate2;
extern crate tar;

use std::process::Command;
use flate2::read::GzDecoder;
use tar::Archive;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap(); 
    if target == "x86_64-pc-windows-msvc" || target == "i686-pc-windows-msvc" {
        let output = Command::new("curl").arg("-s").arg("-L").arg("https://github.com/Luminarys/Nishikaku-Bin/archive/0.1.0.tar.gz")
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

        let resp = output.stdout;
        let d = GzDecoder::new(&resp[..]).unwrap();
        let mut a = Archive::new(d);
        a.unpack("./target/").unwrap();
    }
}

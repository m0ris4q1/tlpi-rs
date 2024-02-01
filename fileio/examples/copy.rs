use alt::fs::OpenOptions;
use std::io::Read;

fn main() {
    let mut file = OpenOptions::new()
        .read(true)
        .open("Cargo.toml").unwrap();

    let mut buffer = [0; 1024];
    let n = file.read(&mut buffer[..]).unwrap();

    let content = String::from_utf8_lossy(&buffer[..n]);

    println!("{}", content);
}

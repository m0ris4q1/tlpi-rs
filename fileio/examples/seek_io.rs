use anyhow::{Context, Result};
use alt::fs::OpenOptions;
use std::env;
use std::io::{Read, Seek, SeekFrom, Write};
use tlpi::{
    GN_ANY_BASE, GN_NONNEG,
    cmd_line_err, get_long, usage_err,
};

fn main() -> Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 3 || argv[1] == "--help" {
        usage_err!("{} file {}...\n", argv[0],
                   "{r<length>|R<length>|w<string>|s<offset>}");
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o600)
        .open(&argv[1])
        .context("open")?;

    for arg in argv.into_iter().skip(2) {
        match arg.bytes().next() {
            Some(b'r') | Some(b'R') => {
                let flags = GN_NONNEG | GN_ANY_BASE;
                let len: usize = get_long(&arg[1..], flags, &arg)?
                    .try_into().unwrap();
                let mut buf = vec![0; len];
                let num_read = file.read(&mut buf).context("read")?;
                if num_read == 0 {
                    println!("{}: end-of-file", arg);
                } else {
                    print!("{}: ", arg);
                    for j in 0..num_read {
                        match arg.bytes().next().unwrap() {
                            b'r' => {
                                let is_printable = unsafe {
                                    let byte = (buf[j] as i8) as i32;
                                    libc::isprint(byte) > 0
                                };
                                match is_printable {
                                    true => print!("{}", buf[j] as char),
                                    false => print!("{}", '?'),
                                }
                            }
                            b'R' => {
                                print!("{:02x} ", buf[j]);
                            }
                            _ => unreachable!(),
                        }
                    }
                    print!("\n");
                }
            }
            Some(b'w') => {
                let buf = &arg[1..].as_bytes();
                let num_written = file.write(buf).context("write")?;
                println!("{}: wrote {} bytes", arg, num_written);
            }
            Some(b's') => {
                let flags = GN_NONNEG | GN_ANY_BASE;
                let offset: u64 = get_long(&arg[1..], flags, &arg)?
                    .try_into().unwrap();
                file.seek(SeekFrom::Start(offset)).context("lseek")?;
                println!("{}: seek succeeded", arg);
            }
            _ => {
                cmd_line_err!("Argument must start with [rRws]: {}\n", arg);
            }
        }
    }

    Ok(())
}

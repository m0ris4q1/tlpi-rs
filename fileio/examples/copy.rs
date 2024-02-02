use alt::fs::OpenOptions;
use anyhow::{Context, Result};
use std::env;
use std::io::{Read, Write};
use tlpi::{fatal, usage_err};

fn main() -> Result<()> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 3 || argv[1] == "--help" {
        usage_err!("{} old-file new-file\n", argv[0]);
    }

    /* Open input and output files */

    let mut input_file = OpenOptions::new()
        .read(true)
        .open(&argv[1])
        .with_context(|| format!("opening file {}", argv[1]))?;

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(&argv[2])
        .with_context(|| format!("opening file {}", argv[2]))?;


    /* Transfer data until we encounter end of input or an error */

    let mut buffer = [0; 1024];

    loop {
        let num_read = input_file.read(&mut buffer).context("read")?;
        if num_read == 0 {
            break;
        }
        let num_written = output_file.write(&buffer[..num_read]).context("write")?;
        if num_written != num_read {
            fatal!("could not write whole buffer");
        }
    }

    input_file.close().context("close input")?;
    output_file.close().context("close output")?;

    Ok(())
}

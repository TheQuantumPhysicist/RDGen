use std::io::{Read, Write};

use anyhow::Context;
use clap::Parser;
use rdgen_lib::FiniteDataWriter;

mod program_options;

fn main() -> anyhow::Result<()> {
    let args: program_options::RDGenOptions = program_options::RDGenOptions::parse();

    let data_writer = match args.file {
        Some(f) => {
            let reader = open_file(f)?;
            FiniteDataWriter::new_from_stream(reader, Some(args.length))?
        }
        None => {
            let stdin = std::io::stdin();
            FiniteDataWriter::new_from_stream(stdin, Some(args.length))?
        }
    };

    {
        let stdout = std::io::stdout();
        let mut stdout_handle = stdout.lock();

        for data in data_writer {
            stdout_handle
                .write_all(&data)
                .expect("Writing result to stdout failing")
        }
    }

    Ok(())
}

fn open_file(p: impl AsRef<std::path::Path>) -> anyhow::Result<impl Read> {
    let p = p.as_ref();
    if !p.exists() {
        return Err(anyhow::anyhow!("File not found: {}", p.display()));
    }

    if !p.is_file() {
        return Err(anyhow::anyhow!(
            "Path provided is not a file or unreadable: {}",
            p.display()
        ));
    }

    let f = std::fs::File::open(p).context(format!("Opening file failed: {}", p.display()))?;

    Ok(f)
}

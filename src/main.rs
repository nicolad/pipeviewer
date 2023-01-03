use clap::{App, Arg};
use std::env;
use std::io::{self, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let matches = App::new("pipeviewer")
        .arg(Arg::with_name("infile").help("Read from file instead of stdin"))
        .arg(
            Arg::with_name("outfile")
                .help("Write to file instead of stdout")
                .short("o")
                .long("outfile")
                .help("Write to file instead of stdout"),
        )
        .arg(
            Arg::with_name("silent")
                .help("Don't print the number of bytes read")
                .short("s")
                .long("silent"),
        )
        .get_matches();

    let infile = matches.value_of("infile").unwrap_or_default();
    let outfile = matches.value_of("outfile").unwrap_or_default();
    let silent = if matches.is_present("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };

    dbg!(infile, outfile, silent);

    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    let mut total_bytes = 0;
    loop {
        let mut buffer = [0; CHUNK_SIZE];

        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == io::ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }
    if !silent {
        eprintln!("\r{}", total_bytes);
    }
    Ok(())
}

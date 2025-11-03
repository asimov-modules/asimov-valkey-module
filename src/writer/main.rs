// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-valkey-writer requires the 'std' feature");

use asimov_module::SysexitsError::{self, *};
use asimov_module::getenv;
use clap::Parser;
use clientele::StandardOptions;
use redis::Commands as _;
use std::error::Error;
use std::io::{self, BufRead};

/// asimov-valkey-writer
#[derive(Debug, Parser)]
#[command(about = "Publish stdin to Valkey channels")]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// Copy stdin to stdout
    #[arg(short = 'U', long)]
    union: bool,

    /// Channels to subscribe to
    #[arg(required = true)]
    channels: Vec<String>,
}

pub fn main() -> Result<SysexitsError, Box<dyn Error>> {
    // Load environment variables from `.env`:
    asimov_module::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = asimov_module::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    let db_url = getenv::var("ASIMOV_VALKEY_URL").unwrap_or("redis://localhost:6379/0".into());

    let mut client = redis::Client::open(db_url)?;

    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        match stdin.lock().read_line(&mut buffer)? {
            0 => break,
            n => {
                let input = &buffer[..n];
                let input_msg = input.trim_end_matches(['\r', '\n']);

                for channel_name in &options.channels {
                    client.publish::<_, _, ()>(channel_name, input_msg)?;
                }

                if options.union {
                    io::Write::write_all(&mut io::stdout(), input_msg.as_bytes())?;
                    io::Write::write_all(&mut io::stdout(), b"\n")?;
                }
            },
        }
    }

    Ok(EX_OK)
}

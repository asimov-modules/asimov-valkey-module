// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-valkey-cataloger requires the 'std' feature");

use asimov_module::SysexitsError::{self, *};
use clap::{Parser, ValueEnum};
use clientele::StandardOptions;
use redis::Commands as _;
use std::error::Error;
use std::io::{self, Write};
use asimov_module::getenv;

/// asimov-valkey-cataloger
#[derive(Debug, Parser)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// Specify the output format [jsonl, url]
    #[arg(short, long, value_enum, default_value_t = Output::Jsonl)]
    output: Output,
}

/// Output format for --output
#[derive(Copy, Clone, Debug, ValueEnum, Eq, PartialEq)]
enum Output {
    /// Emit values (one JSON object per line)
    Jsonl,
    /// Emit keys (one URL per line)
    #[value(alias = "urls")]
    Url,
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

    let db_url = getenv::var("ASIMOV_VALKEY_URL")
        .unwrap_or("redis://localhost:6379/0".into());

    let client = redis::Client::open(db_url)?;
    let mut conn = client.get_connection()?;

    let mut stdout = io::stdout().lock();
    let mut cursor: u64 = 0;

    loop {
        let (next_cursor, keys): (u64, Vec<Vec<u8>>) =
            redis::cmd("SCAN").arg(cursor).query(&mut conn)?;

        for resource_url in keys {
            if resource_url.starts_with(b"asimov:") {
                continue;
            }

            if options.flags.debug {
                let url_str = String::from_utf8_lossy(&resource_url);
                eprintln!("Fetching `{}`...", url_str);
            }

            let output = match options.output {
                Output::Url => resource_url,
                Output::Jsonl => {
                    if let Ok(Some(resource_data)) = conn.get::<_, Option<Vec<u8>>>(&resource_url) {
                        resource_data
                    } else {
                        continue;
                    }
                }
            };

            if let Err(e) = stdout
                .write_all(&output)
                .and_then(|_| stdout.write_all(b"\n"))
                .and_then(|_| stdout.flush())
            {
                if e.kind() == io::ErrorKind::BrokenPipe {
                    return Ok(EX_OK);
                }
                return Err(e.into());
            }
        }

        cursor = next_cursor;
        if cursor == 0 {
            break;
        }
    }

    Ok(EX_OK)
}

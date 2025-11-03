// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-valkey-reader requires the 'std' feature");

use asimov_module::SysexitsError::{self, *};
use asimov_module::getenv;
use clap::{Parser, ValueEnum};
use clientele::StandardOptions;
use redis::{ControlFlow, PubSubCommands as _};
use std::error::Error;
use std::io::{self, Write};

/// asimov-valkey-reader
#[derive(Debug, Parser)]
#[command(about = "Subscribe to Valkey channels")]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// Specify the output format [jsonl, url]
    #[arg(value_name = "FORMAT", short, long, value_enum, default_value_t = Output::Jsonl)]
    output: Output,

    /// Channels to subscribe to
    #[arg(required = true)]
    channels: Vec<String>,
}

/// Output format for --output
#[derive(Copy, Clone, Debug, ValueEnum, Eq, PartialEq)]
enum Output {
    /// Emit raw message as a JSON line
    Jsonl,
    /// Emit "@id" value from the JSON (one per line)
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

    let db_url = getenv::var("ASIMOV_VALKEY_URL").unwrap_or("redis://localhost:6379/0".into());

    let client = redis::Client::open(db_url)?;

    let mut stdout = io::stdout().lock();

    client
        .get_connection()?
        .subscribe(&options.channels, move |msg| {
            let Ok(payload) = msg.get_payload::<Vec<u8>>() else {
                return ControlFlow::Continue;
            };
            let Ok(input_msg) =
                String::from_utf8(payload).map(|s| s.trim_end_matches(['\r', '\n']).to_string())
            else {
                return ControlFlow::Continue;
            };

            let output = match options.output {
                Output::Url => match serde_json::from_str::<serde_json::Value>(&input_msg) {
                    Ok(json_obj) => {
                        let Some(id) = json_obj.get("@id") else {
                            eprintln!("Ignored invalid input: {}", input_msg);
                            return ControlFlow::Continue;
                        };
                        match id {
                            serde_json::Value::String(s) => s.clone(),
                            other => other.to_string(),
                        }
                    }
                    Err(_) => {
                        eprintln!("Ignored invalid input: {}", input_msg);
                        return ControlFlow::Continue;
                    }
                },
                Output::Jsonl => input_msg,
            };

            match writeln!(&mut stdout, "{output}").and_then(|_| stdout.flush()) {
                Err(e) if e.kind() == io::ErrorKind::BrokenPipe => ControlFlow::Break(()),
                _ => ControlFlow::Continue,
            }
        })?;

    Ok(EX_OK)
}
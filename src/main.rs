use chrono::{DateTime, SecondsFormat, Utc};
use clap::Parser;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[arg(
        short,
        long,
        help = "File to same start and stop time to. Needs to be the same between matching starts and stops. Stored in TMPDIR/ and defaults to rtime"
    )]
    time_file_path: Option<String>,

    #[arg(long, help = "Stop time capture using file")]
    stop: bool,

    #[arg(
        short,
        long,
        help = "Clean up time file. Only applies if --stop is used."
    )]
    clean: bool,

    #[arg(long, help = "Start time capture using file")]
    start: bool,

    #[arg(short, long, help = "Enable limited debug logging to stdout")]
    debug: bool,
}

fn command_start(file_path: PathBuf, debug: bool) -> i32 {
    let start_time = Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true);

    let mut file = match OpenOptions::new().write(true).append(true).open(&file_path) {
        Ok(f) => f,
        Err(_) => match OpenOptions::new().write(true).create(true).open(&file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error opening or creating file: {}", e);
                return 1;
            }
        },
    };

    if let Err(e) = writeln!(file, "Start: {}", start_time) {
        eprintln!("Error writing start time to file: {}", e);
        return 1;
    }

    if debug {
        println!("Start capture at {}", start_time);
    }

    0
}

fn command_stop(file_path: PathBuf, debug: bool, clean: bool) -> i32 {
    let stop_time_parsed = Utc::now(); // Store precise stop time for elapsed calculation

    {
        let file = match OpenOptions::new().read(true).open(&file_path) {
            Ok(f) => f,
            Err(_) => {
                eprintln!(
                    "Start file doesn't exist at {}",
                    Error::from(ErrorKind::NotFound)
                );
                return 1;
            }
        };
        let reader = BufReader::new(file);
        let mut last_start_time = String::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("Start: ") {
                last_start_time = line;
            }
        }

        if last_start_time.is_empty() {
            eprintln!("No start time found in the file.");
            return 1;
        }

        let start_time_str = &last_start_time[7..]; // Remove "Start: " prefix
        let start_time_parsed = match DateTime::parse_from_rfc3339(start_time_str) {
            Ok(dt) => dt.with_timezone(&Utc),
            Err(e) => {
                eprintln!("Error parsing start time: {}", e);
                return 1;
            }
        };

        let duration = stop_time_parsed.signed_duration_since(start_time_parsed);
        let elapsed_time = format!(
            "{}.{} seconds",
            duration.num_seconds(),
            (duration.num_nanoseconds().unwrap_or(0) % 1_000_000_000)
        );

        if debug {
            println!("Last start time: {}", last_start_time);
        }

        println!("{}", elapsed_time);
    }

    if clean {
        let clean_reuslt = std::fs::remove_file(&file_path);

        match clean_reuslt {
            Err(e) => {
                eprintln!("{}", e);
            }
            Ok(..) => {
                if debug {
                    println!("Deleted {}", file_path.display());
                }
            }
        }
    }

    0
}

fn main() {
    let args = CliArgs::parse();

    let file_path: PathBuf;

    if args.time_file_path.is_none() {
        file_path = std::env::temp_dir().join("rtime");
    } else {
        file_path = std::env::temp_dir().join(args.time_file_path.unwrap());
    }

    if args.debug {
        println!("Time file path is: {:?}", file_path.display());
    }

    if args.stop {
        std::process::exit(command_stop(file_path, args.debug, args.clean));
    } else if args.start {
        std::process::exit(command_start(file_path, args.debug));
    } else {
        eprintln!("Need either start or stop command")
    }
}

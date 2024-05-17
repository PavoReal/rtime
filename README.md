# rtime

`rtime` is a simple command-line tool written in Rust for capturing start and stop times and calculating the elapsed duration. It reads and writes timestamps to a file, and optionally cleans up the file after stopping. It supports limited debug logging to stdout.

## Usage

```sh
rtime.exe [OPTIONS]
```

### Options:

- `-t, --time-file-path <TIME_FILE_PATH>`

  Specify a file to save the start and stop times. The same file needs to be used between matching starts and stops. The file is stored in `TMPDIR/` and defaults to `rtime`.

- `--stop`

  Stop time capture using the specified file.

- `-c, --clean`

  Clean up the time file. Only applies if `--stop` is used.

- `--start`

  Start time capture using the specified file.

- `-d, --debug`

  Enable limited debug logging to stdout.

- `-h, --help`

  Print help information.

- `-V, --version`

  Print the version information.

## Installation

To install `rtime`, you'll need to have Rust installed. You can install rust [here](https://www.rust-lang.org/tools/install) if you don't already have it. You can then build the application using `cargo`:

```sh
git clone https://github.com/PavoReal/rtime.git
cd rtime
cargo build --release
```

The compiled executable will be available at `target/release/rtime`.

## Examples

### Start Capturing Time

To start capturing time:

```sh
rtime --start
```

### Stop Capturing Time and Calculate Duration

To stop capturing time and see the elapsed duration since the last start:

```sh
rtime.exe --stop
```

### Clean Up the Time File

To clean up the time file after stopping:

```sh
rtime.exe --stop --clean
```

### Specify a Custom Time File Path

To specify a custom file for storing timestamps:

```sh
rtime.exe -t .time_file --start
```

To use the same file when stopping:

```sh
rtime.exe -t .time_file --stop
```

### Complete Example

```sh
rtime --start && sleep 10 && rtime --stop
10.28896561 seconds
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

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

To install `rtime`, you'll need to have Rust installed. You can then build the application using `cargo`:

```sh
git clone https://github.com/yourusername/rtime.git
cd rtime
cargo build --release
```

The compiled executable will be available at `target/release/rtime`.

## Examples

### Start Capturing Time

To start capturing time:

```sh
rtime.exe --start
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
rtime.exe --time-file-path custom_timelog --start
```

To use the same file when stopping:

```sh
rtime.exe --time-file-path custom_timelog --stop
```

### Enable Debug Logging

To start capturing with debug logging enabled:

```sh
rtime.exe --start --debug
```

To stop capturing with debug logging enabled:

```sh
rtime.exe --stop --debug
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

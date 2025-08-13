# fuzz

```bash
$ spacejam fuzz -h
Spacejam fuzz command

Usage: spacejam fuzz [OPTIONS] <COMMAND>

Commands:
  target  Fuzz with local unix socket
  fuzzer  Fuzz with a fuzzer
  tx      Run trace test via the given trace file
  help    Print this message or the help of the given subcommand(s)

Options:
  -v...         The verbosity level (repeat for more verbosity)
  -n, --noansi  Disable ANSI colors
  -h, --help    Print help
```

## Usage

```bash
# Start a fuzz target
spacejam fuzz target -v

# Run a fuzzer
spacejam fuzz fuzzer -t jam-test-vectors/traces/storage -v

# Run a trace test
spacejam fuzz tx jam-test-vectors/traces/storage/00000001.json -v
```

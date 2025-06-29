Fuzz configuration of `spacejam` according to [davxy/jam-stuff][jam-stuff]

## Spacejam as a fuzz target

```bash
spacejam fuzz -h
Spacejam fuzz command

Usage: spacejam fuzz [OPTIONS] <COMMAND>

Commands:
  local  Fuzz with local unix socket
  trace  Run trace tests via the given trace folder
  help   Print this message or the help of the given subcommand(s)

Options:
  -v...       The verbosity level (repeat for more verbosity)
  -h, --help  Print help
```

### <kbd>fuzz trace</kbd>

```bash
spacejam fuzz trace jam-test-vectors/traces/reports-l1
```

### <kbd>fuzz local</kbd>

```bash
spacejam fuzz local tmp/jam_target.sock
```

[jam-stuff]: https://github.com/davxy/jam-stuff/blob/main/fuzz-proto/README.md

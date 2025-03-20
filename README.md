# Specjam

A tool which can <kbd>spawn</kbd> [binary runner](#binary-runner) to test the JAM spec.

## Usage

```bash
# for installing specjam without full test vectors, you can just install it from crates.io.
cargo install specjam

# for installing specjam with full test vectors
JAM_TEST_VECTORS=/path/to/jamtestvectors cargo install specjam --features full
```

```bash
# Spawn a binary runner
specjam spawn <my-binary>
```

Apart from `spawn`, you can run `specjam --help` to get the full usage.

```text
The JAM spec test engine developed by spacejam

Usage: specjam [OPTIONS] [COMMAND]

Commands:
  dummy   Try out the dummy test runner
  input   Print the input of a test
  list    List the tests of a section
  output  Print the output of a test
  spawn   Spawn a binary runner
  spec    Prints the version of the JAM spec
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  The verbosity level
  -h, --help        Print help
  -V, --version     Print version
```

## Binary Runner

The binary runner must follow the following interface:

```text
USAGE:
  <binary> [OPTIONS]

OPTIONS:
  --section <section> the name of the section
  --name <name> the name of the test
  --input <input> The file path of the input JSON
```

The stdout of the binary runner must be a valid JSON which matches the following schema:

```json
{
  "output": {
    "output": "...",
    "post_state": "..."
  }
}
```

There are different schema for `codec`, `pvm`, `trie` and `shuffle`. you can check out the schema
with `specjam input <section> <name>` and `specjam output <section> <name>`.

# LICENSE

GPL-3.0

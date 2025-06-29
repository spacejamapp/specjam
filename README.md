## Spacejam Releases

> _NOTE: The previous testing toolkit in this repo is now archived since now polkajam provides the tracing tests, you can find it in the history commits if needed._

- You can find the releases of `spacejam` in the [releases](https://github.com/polkajam/spacejam/releases) page.
- You can find the guide of using `spacejam` as a fuzz target in [FUZZ.md](./FUZZ.md).

### Spacejam Testnet

There is also a `testnet` binary provided in the releases, which is a testnet manager developed by spacejam.
Check out the template config at [testnet.toml](./testnet.toml).

```
$ testnet --help
The command line interface for testnet

Usage: testnet [OPTIONS] <COMMAND>

Commands:
  generate  Generate a new testnet configuration file
  prune     Prune the testnet
  start     Start the testnet
  help      Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  The path to the testnet configuration file
  -n, --noansi           Whether to use ANSI colors in the output
  -h, --help             Print help

$ testnet -c testnet.toml
```

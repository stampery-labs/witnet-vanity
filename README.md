# witnet-vanity

A simple CLI tool to generate [Witnet](https://witnet.io/) vanity addresses, i.e. starting with `wit` or `twit`.
Optionally, other prefixes can be configured by using the CLI option `hrp`.

Similar to a vanity license plate, a vanity cryptocurrency address starts with a specific pattern.
Generating such an addresses requires a significant amount of work as they are found by generating random `Secp256k1` keypairs
and checking if the derived addresses follow the requested patterns. Therefore, the longer the pattern the harder it is to found a match.

>I just thought of something. Eventually there'll be some interest in brute force scanning 
>bitcoin addresses to find one with the first few characters customized to your name, kind
>of like getting a phone number that spells out something. Just by chance I have my initials.
>
> — Satoshi Nakamoto in an email to Hal Finney in 2009

Witnet vanity addresses are are inspired in the BIP 0173 and they follow the Bech32 format.
However, in Witnet `SHA256` is used in order to derived addresses from `Secp256k1` public keys.


## Usage

The CLI tool provides the following options:

```bash
⇒  cargo run --release -- -h

Witnet vanity address generator 0.1.0
Stampery Labs
Vanity address generator using curve Secp256k1 and in Bech32 format: <hrp>1<string>

USAGE:
    witnet-vanity [OPTIONS] <vanity-string>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -H, --hrp <hrp>            Human-readable part of the vanity address (e.g. wit, twit, bc) [default: wit]
    -t, --threads <threads>    Number of running threads executed in parallel [default: threads = num_cpus]

ARGS:
    <vanity-string>    Vanity prefix string to generate
```   


## Example

In the following example, an address with prefix 'm00n':

```bash
⇒  cargo run --release -- m00n

Searching vanity addresses with the prefix: wit1m00n

  [00:00:19] [################################################################################] address found! (ETA: 0s)

Vanity address found:
        SK bytes:       6c3a17f2e4c6bcd62da0b661904dd0508fb5bbcf6267f8ceedfc3a60849a7b2d
        Private key:    xprv1qq9qjt4xft2zyty744t7nuyergtulqhs52xrpcmt22hs3wzuk6tz6qrv8gtl9exxhntzmg9kvxgym5zs376mhnmzvluvam0u8fsgfxnm95s96kwv
        Address:        wit1m00nj3eyzl5prcluexusfrcaf80fds6hhu5rxd
```

The tool outputs:

 - private key bytes
 - private key in `xprv` format (as it could be used to imported in [witnet-rust](https://github.com/witnet/witnet-rust/) node)
 - address in bech32 format


## License

`witnet-vanity` is published under the [MIT license][license].

[license]: https://github.com/stampery-labs/witnet-vanity/blob/master/LICENSE

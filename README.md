# witnet-vanity

A simple CLI tool to generate [Witnet](https://witnet.io/) vanity addresses, i.e. starting with the network prefix (`wit` or `twit`) and a custom text.

Similar to a vanity license plate, a vanity cryptocurrency address starts with a specific pattern.
Generating such an addresses requires a significant amount of work as they are found by generating random `Secp256k1` keypairs
and checking if the derived addresses follow the requested patterns. Therefore, the longer the pattern the harder it is to found a match.

>I just thought of something. Eventually there'll be some interest in brute force scanning 
>bitcoin addresses to find one with the first few characters customized to your name, kind
>of like getting a phone number that spells out something. Just by chance I have my initials.
>
> — Satoshi Nakamoto in an email to Hal Finney in 2009

Witnet vanity addresses are are inspired in the [BIP 0173](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki), i.e. they follow the Bech32 format. They are derived based on `Secp256k1` public keys and the hash function `SHA256`.


## Bech32 addresses

The Bech32 address format is structured as:

 - Human readeable part (hrp): type of data, or anything else that is relevant to the reader (MUST contain 1 to 83 US-ASCII characters).
 - Separator: always "1" by specification.
 - Data part: only consists of alphanumeric characters excluding "1", "b", "i", and "o". The last six characters of the data part is a checksum of `hrp` and `data` parts. The checksum is used to detect errors that may be introduced in transmission or storage.

More information can be found in the [BIP 0173](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki).


## Usage

The tool has been written in Rust and it can be executed by using `cargo`. Installation can be found [here](https://www.rust-lang.org/tools/install).

It is encouraged to build the CLI tool with the `--release` tag for a better performance:

```bash
$ cargo build --release

    Compiling...

    Finished dev [unoptimized + debuginfo] target(s) in 14.69s
$ 
```

Then it can be executed by using the binary file. Here for example we are using the `--help` flag to show the tool arguments:

```bash
$ ./target/release/witnet-vanity --help

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

The CLI tool arguments are:

 - \<vanity-string\>: mandatory argument to define the the vanity address starting prefix.
 - `--hrp` \<string\>: optional argument to define the human readable part of the Bech32 address. By default is set to `wit`.
 - `--threads` \<number\>: optional argument to set how many CPUs will be used for generating the vanity address. By default is set to use all available CPUs.


The CLI tool will show an estimation of the progress while finding a valid vanity address, showing the amount of addresses checked and an estimated processing time.

```bash
$ ./target/release/witnet-vanity example

Searching vanity addresses with the prefix: wit1example (threads: 8)

⠒ [00:00:31] [>-------------------------------------------------------------------------------] 3.2M/34.36G estimated tries (ETA: 3d)

```


### Alternative usage

Alternatively, `witnet-vanity` can be installed as a binary in the system:

```bash
$ cargo install --git https://github.com/stampery-labs/witnet-vanity
$ witnet-vanity --help
```


### Additional considerations

Please take into account that valid vanity addresses are found by generating random `Secp256k1` key pairs. Therefore the estimation is just orientative and finding a valid address may take longer (or shorter) than the estimated time.

Take into consideration that the average required time to find a valid vanity address grows exponentially with the number of characters of the `vanity string`.


## Generating addresses

As previously mentioned, an address in Bech32 format is composed of a human readable part (`hrp`), a separator and the data.

The `hrp` of an address is often used to indicate the type of network the address is being used for.

In the case of Witnet, the `hrp` is used to tag the network they belong to:

 - mainnet addresses start with `wit`
 - testnet addresses start with `twit`

The separator in both cases is always `1`, leading to `wit1` and `twit1` respectively.


### Generating a mainnet Witnet address (`wit1`)

In the following example, an address with prefix 'm00n' will be generated. As no `--hrp` argument is given, the default one `wit` will be used. If found, the generated address will start with `wit1m00n`.

```bash
$ ./target/release/witnet-vanity m00n

Searching vanity addresses with the prefix: wit1m00n (threads: 8)

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


### Generating a testnet Witnet address (`twit1`)

In order to generate a testnet Witnet address the `--hrp` argument should be set the human readable part to `twit`:

```bash
$ ./target/release/witnet-vanity m00n --hrp twit

Searching vanity addresses with the prefix: twit1m00n (threads: 8)

  [00:00:06] [###########################################################>--------------------] address found! (ETA: 0s)

Vanity address found:
        SK bytes:       07ccbeccfce73c191e0f9c95034a575a676f311326751203bdbc110ad0db66e5
        Private key:    xprv1qpnxu2vcp2a47d6sv7h4lpygnujhydhnhxsnjghe67recaqcvv5e7qq8ejlvel888sv3uruuj5p55466vahnzyexw5fq80duzy9dpkmxu5hn79vx
        Address:        twit1m00n7jmwkg2w7rr669frmay3h6e2733zm0cppp

```


#### Reusing addresses between mainnet and testnet

Witnet testnet Bech32 addresses can be reused for different human readible parts (`hrp`). However, the last 6 address characters will change as they are a checksum of the `hrp` and the `data` itself. In other words, the same private key (e.g. in `xprv` format) can be used to derive public keys for which may have different `hrp` parts.

This is the case of the Witnet mainnet and testnet. For example, imagine we would like to reuse the previously found vanity address for mainnet (`wit1`):

```bash
Private key:    xprv1qq9qjt4xft2zyty744t7nuyergtulqhs52xrpcmt22hs3wzuk6tz6qrv8gtl9exxhntzmg9kvxgym5zs376mhnmzvluvam0u8fsgfxnm95s96kwv
Address:        wit1m00nj3eyzl5prcluexusfrcaf80fds6hhu5rxd
```

After importing the private key (with `xprv` format) in the node, the derived public key for Witnet testnet from the logs:

```bash
$ ./witnet node server --master-key-import master.key

[2020-06-09T18:15:42Z INFO  witnet_node::actors::chain_manager::actor] PublicKeyHash: twit1m00nj3eyzl5prcluexusfrcaf80fds6hefa8xu
```

It is shown that the same `xprv` produces the same valid vanity address but they differ in the `checksum` in their Bech32 format representation (last 6 characters):

- **wit1m00n**j3eyzl5prcluexusfrcaf80fds6h**hu5rxd**
- **twit1m00n**j3eyzl5prcluexusfrcaf80fds6h**efa8xu**


## License

`witnet-vanity` is published under the [MIT license][license].

[license]: https://github.com/stampery-labs/witnet-vanity/blob/master/LICENSE

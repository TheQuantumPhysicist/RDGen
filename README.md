# RDGen

A terminal program for generating reproducible random data for testing based on a provided seed.

## Why?

For [thash](https://github.com/TheQuantumPhysicist/thash/), I needed a way to generate reproducible random data for testing. A condition for this was that the seed will *always* reproduce the same data. So, I created RDGen.

## Installation

You can install this with [cargo](https://www.rust-lang.org/tools/install).

```bash
cargo install rdgen
```

Currently it's only available on crates.io with cargo. If it gets popular enough, I'll happily upload it elsewhere.

## How it works

This program simply takes the given seed, and recursively hashes it and creates an infinitely long string. The hashing algorithm chosen is blake2b, because it's secure and fast.

## Usage

Run `rdgen --help`, to see all available options.

The seed is passed through stdin. The output of the data is written to stdout (in binary). You can process it by either piping it to a file, or some other program (see the examples below).

For platforms that don't have commands like `cat` to pass a file through stdin, you can use the `-f` command line argument to specify the file that contains the seed, instead of using stdin. If no `-f` is specified, stdin will be used.

### Examples

- Say you want to generate 100 bytes of data, based on the seed "abc":

```
$ echo -n "abc" | rdgen -l100 | xxd -p -c 0
ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd400992366cb547665e462bbdd51d9b6ce1221116e9cfc6711c78d8798158349d12fa8ca513efb14
```

which will result in 100 bytes of binary data written out.

Notice that we use `-n` with `echo` to avoid adding a new line at the end.

Notice also that we also pipe the result to `xxd -p -c 0`, because otherwise the result will be printed to terminal using stdout. Since the result is binary, it will be nonsensical and unreadable.

- Though you can put that data into a binary file

```
$ echo -n "abc" | rdgen -l100 > random-data.bin
```

- If you want to pipe data written by hand, you can use `xxd -r -p` as well, then pipe it to `rdgen`

```
$ echo -n "deadbeef" | xxd -r -p | rdgen -l100 | xxd -p -c 0
```

this will decode `deadbeef` hex data into binary, then pipe it to rdgen as seed.

Happy testing!

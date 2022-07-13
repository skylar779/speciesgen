# Manual Installation

1. [Prerequisites](#prerequisites)
    1. [Source Code](#clone-the-source-code)
    2. [Installing `rustup`](#installing-rustup)
2. [Building](#building)
3. [Manual Page](#manual-page)

## Prerequisites

### Clone the source code

To compile speciesgen, please clone the source code:

```sh
git clone https://github.com/skylar779/speciesgen.git
cd speciesgen
```

### Installing `rustup`
`rustup` can be found [here](https://rustup.rs/).

## Building

To build a release version of speciesgen, run

```sh
cargo build --release
```

If all goes well, the application will be found under `target/release/speciesgen`.

## Manual Page

Installing the manual page requires `gzip`.

```sh
sudo mkdir -p /usr/local/share/man/man1
gzip -c man/speciesgen.1 | sudo tee /usr/local/share/man/man1/speciesgen.1.gz > /dev/null
```


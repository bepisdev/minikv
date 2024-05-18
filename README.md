# MiniKV

Miniature Key-Value Store over HTTP.

## Features

__FIXME__

## Usage

**Basic Setup**

``` shell
$ export MINIKV_USERNAME="admin"
$ export MINIKV_PASSWORD="amoresecurepaswordthandefault"
$ minikv -p 8899
```

### Configuration

MiniKV is configurable via CLI flags and environment variables.

#### CLI Flags

- `-p` or `--port` - Defines TCP/IP port to bind too (Default is `8899`).
- `--host` - Defines interface IP address to bind too (Default is `0.0.0.0` - leave this alone unless you know what you're doing).
- `--username` - Defines the *HTTP Basic Auth* username to use for authentication.
- `--password` - Defines the *HTTP Basic Auth* password to use for authentication.

#### Environment Variables

- `MINIKV_USERNAME` - Same as `--username`
- `MINIKV_PASSWORD` - Same as `--password`

## Installation

**Requirements**

- Rust toolchain
- Make

``` shell
$ git clone https://github.com/joshburnsxyz/minikv
$ cd minikv
$ make
$ sudo make install
```

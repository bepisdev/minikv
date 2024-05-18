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

#### Environment Variables

- `MINIKV_USERNAME` - Defines the *HTTP Basic Auth* username to use for authentication.
- `MINIKV_PASSWORD` - Defines the *HTTP Basic Auth* password to use for authentication.
- `MINIKV_LOG_LEVEL` - Defines the logging level for output, one of the following (in order of verbosity).
  - `error`
  - `warn`
  - `info`
  - `debug`
  - `trace`

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

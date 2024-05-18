# MiniKV

Miniature Key-Value Store over HTTP.

## Features

__FIXME__

## Usage

**Basic Setup**

``` shell
$ export MINIKV_USERNAME=admin
$ export MINIKV_PASSWORD=amoresecurepaswordthandefault
$ export MINIKV_LOG_LEVEL=info
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
  - `warn` - (*This is the default level, anything below this wont be logged by default*)
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

 Here's the updated README file with examples of interacting with the API using cURL:

```markdown
# MiniKV Server

MiniKV Server is a lightweight key-value store implemented in Rust using Actix Web. It provides a simple HTTP API for storing and retrieving key-value pairs.

## Installation

Clone the repository:

```bash
git clone https://github.com/your_username/minikv-server.git
cd minikv-server
```

Ensure you have Rust and Cargo installed. You can install them using [rustup](https://rustup.rs/).

## Usage

1. Set up environment variables for authentication:

```bash
export MINIKV_USERNAME=admin
export MINIKV_PASSWORD=admin
```

Replace `admin` with your desired username and password.

2. Build and run the server:

```bash
cargo run
```

The server will start running at `http://localhost:8899` by default.

## Configuration

You can configure the MiniKV server using the following environment variables:

- `MINIKV_USERNAME`: The username required for basic authentication (default: admin).
- `MINIKV_PASSWORD`: The password required for basic authentication (default: admin).
- `MINIKV_LOG_LEVEL`: The log level for the server (default: warn).

To configure these options, set the environment variables before running the server.

## API Endpoints

### Fetch Value
```
GET /keys/{key}
```
Fetches the value associated with the given key.

Example:
```bash
curl -u admin:admin http://localhost:8899/keys/my_key
```

### Set Value
```
POST /keys/{key}
```
Sets the value associated with the given key.

Example:
```bash
curl -u admin:admin -X POST -d "my_value" http://localhost:8899/keys/my_key
```

### Delete Value
```
DELETE /keys/{key}
```
Deletes the value associated with the given key.

Example:
```bash
curl -u admin:admin -X DELETE http://localhost:8899/keys/my_key
```

### View All Keys
```
GET /all
```
Fetches all key-value pairs stored in the server.

Example:
```bash
curl -u admin:admin http://localhost:8899/all
```

## Installation

``` shell
$ make
$ sudo make install
```

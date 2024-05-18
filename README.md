# MiniKV: Miniature Key-Value Store over HTTP

MiniKV is a lightweight Key-Value Store designed to be accessed over HTTP. It offers simple yet efficient storage and retrieval of data through HTTP requests.

## Usage

### Basic Setup

To get started with MiniKV, follow these steps:

1. Set up environment variables for authentication and logging:
    ```shell
    $ export MINIKV_USERNAME=admin
    $ export MINIKV_PASSWORD=amoresecurepasswordthandefault
    $ export MINIKV_LOG_LEVEL=info
    ```

2. Launch MiniKV specifying the port (default port is `8899`):
    ```shell
    $ minikv -p 8899
    ```

### Configuration

MiniKV can be configured using CLI flags and environment variables.

#### CLI Flags

- `-p` or `--port`: Specify the TCP/IP port to bind to.
- `--host`: Define the interface IP address to bind to.

#### Environment Variables

- `MINIKV_USERNAME`: Set the username for HTTP Basic Authentication.
- `MINIKV_PASSWORD`: Set the password for HTTP Basic Authentication.
- `MINIKV_LOG_LEVEL`: Define the logging level (`error`, `warn`, `info`, `debug`, `trace`).

## Installation

### Requirements

- Rust toolchain
- Make

Follow these steps to install MiniKV:

1. Clone the repository:
    ```shell
    $ git clone https://github.com/joshburnsxyz/minikv
    ```

2. Navigate to the project directory:
    ```shell
    $ cd minikv
    ```

3. Build the project using Make:
    ```shell
    $ make
    ```

4. Install MiniKV:
    ```shell
    $ sudo make install
    ```

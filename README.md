# DNS Resolver in Rust

This is a simple DNS resolver implemented in Rust. It provides an easy-to-use command-line interface (CLI) for resolving domain names to IP addresses.

Installation
To compile and use this program, you'll need to have Rust and Cargo installed on your system. If you don't have them installed yet, you can get them by following the instructions at rustup.rs.

Once Rust and Cargo are installed, you can clone this repository and build the program with the following command:

```bash
git clone https://github.com/your_username/resolve_dns.git
cd resolve_dns
cargo build --release
```

This will compile the program in release mode and generate an executable in the `target/release/` directory.

## Usage

The DNS resolver can be used from the command line as follows:

```bash
resolve_dns [OPTIONS] <hostname>
```

Where `<hostname>` is the domain name you want to resolve.

### Available Options

- `-d, --provider <dns-provider>`: Specifies the DNS provider to use. By default, **1.1.1.1** is used.
- `-h, --help`: Displays help information about how to use the program.
- `-V, --version`: Displays the version of the program.

## Examples

```bash
# Resolve the IP address of the example.com domain using the default DNS provider

resolve_dns example.com

# Resolve the IP address of the example.com domain using the DNS provider 8.8.8.8

resolve_dns -d 8.8.8.8 example.com
```

## Contributions

Contributions are welcome! If you find any issues or have any suggestions for improvement, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. For more details, see the LICENSE file.

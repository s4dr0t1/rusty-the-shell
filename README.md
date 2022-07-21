# rusty-the-shell

Rusty is a toy shell written in Rust.
The prototype barely works and should not be tested in production environments.

## Building

You need to build the project yourself and sadly releases will not be supported either until I change my mind or the project becomes more mature.

I have only tried it on Ubuntu based systems, if you're using some other distribution, sadly you're on your own.

```sh
# Install rustup
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install rust stable toolchain
$ rustup toolchain install stable

#Build the project in non-debug mode
$ cargo build --release
```

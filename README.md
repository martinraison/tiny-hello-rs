# Tiny "Hello, world!" binary in Rust

This shows how to create a tiny "Hello, world!" binary in Rust. On my system (macOS), the resulting binary size is 8432 bytes, which is the same as when compiling the following C program with `gcc -Os -o main main.c`:

```c
#include <stdio.h>

int main() {
  printf("Hello, World!\n");
}
```

The goal here is to stay as close as possible to the C "Hello, world!" program above. However it's possible to get even smaller binaries, as documented in [this post](https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html).

Look into the [official Rust documentation](https://doc.rust-lang.org/unstable-book/language-features/lang-items.html#using-libc) for additional information about creating Rust executables without the standard library.

## Get started

```
$ cargo +nightly build --release
$ ./target/release/tiny-hello-rs
Hello, world!
$ wc -c ./target/release/tiny-hello-rs
    8432 ./target/release/tiny-hello-rs
```

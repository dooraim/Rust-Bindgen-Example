# Rust Bindgen example

I wrote a simple example to use Bindgen with a few more files and folders such that there is an example in the wild using `clang_args()`

## Building + Run with Cargo
- `cargo build`
- `cargo run`

## Build + Run with Makefile

- `make clean all`
- `./make`

## Manually compile for `mybind`

Im uploading a prebuilt library. But you can compile for your own platform it you want.

You can compile the library mybind manually from your terminal using and running this from the root directory of the project

```
cd mybind \
&& clang -Wall \
-I./include -I./include2 \
-c test.c ./include/test2.c ./include2/test3.c \
&& ar -cvq libtest.a test.o test2.o test3.o \
&& rm *.o \
&& cd ..
```

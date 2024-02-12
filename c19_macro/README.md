
```shell

$ mkdir c19_macro
$ cd c19_macro/

## create two library projects (does not have to be nested though)

$ cargo new hello_macro --lib

$ cd hello_macro/
$ cargo new hello_macro_derive --lib

## adding code for hello_macro (the trait), hello_macro_derive (a derive macro).

$ cd .. && cargo new pancakes

$ cargo build

# run binary (only one binary)
$ cargo run

$ cargo run -p pancakes

# test
$ cargo test
$ cargo test -p hello_macro
$ cargo test -p pancakes

```

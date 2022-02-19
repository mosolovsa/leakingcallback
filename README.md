Cargo.toml contains 2 references to dependencies:

1. With fix:
```
# with leak fixes in singleshot callbacks
fltk = { git = "https://github.com/mosolovsa/fltk-rs", rev = "7be022d5bbb4de7303b589a67d2dc9ed15b9e506" }

2. 1. Without fix:
# without leak fixes in singleshot callbacks
#fltk = { git = "https://github.com/mosolovsa/fltk-rs", rev = "7e605ef118db59d65e610bb10c7763d424a32d8d" }
```

Each allcation and deallocation triggers message to stdout, so as far as I understood without fix we have memleak (16 + 4 bytes on x86_64) on each call of awake_callback and set_timeout which can be in turn can be called in other callback or tight loop so it's better to prevent memleak. 
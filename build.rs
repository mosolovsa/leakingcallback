extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/foo.c")
        .compile("libfoo.a");
}
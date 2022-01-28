fn main() {
    cc::Build::new()
        .file("mem-write/mem-write.c")
        .compile("libmemwrite.a");
}

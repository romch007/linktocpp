fn main() {
    println!("cargo::rerun-if-changed=src/stats.cpp");

    cc::Build::new()
        .cpp(true)
        .file("src/stats.cpp")
        .compile("stats");
}

fn main() {
    cc::Build::new().file(r#"c/stb.c"#).compile("stb_c");
}

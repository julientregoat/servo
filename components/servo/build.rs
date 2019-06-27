fn main() {
    let layout_2013 = std::env::var_os("CARGO_FEATURE_LAYOUT_2013").is_some();
    let layout_2020 = std::env::var_os("CARGO_FEATURE_LAYOUT_2020").is_some();

    if !(layout_2013 || layout_2020) {
        error("Must enable one of the `layout-2013` or `layout-2020` features.")
    }
    if layout_2013 && layout_2020 {
        error("Must not enable both of the `layout-2013` or `layout-2020` features.")
    }
}

fn error(message: &str) {
    print!("\n\n    Error: {}\n\n", message);
    std::process::exit(1)
}

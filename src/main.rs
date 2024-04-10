fn main() {
    let bust = bustdir::BustDir::new("./test/").unwrap();
    eprintln!("{bust:#?}");
}

fn main() -> Result<(), gog::errors::GogError> {
  Ok(gog::run()?)
}

// fn main() {
//    open::with("https://haider-ali.xyz/", "google-chrome-stable").unwrap();
// }
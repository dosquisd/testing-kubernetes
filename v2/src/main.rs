pub mod core;
use crate::core::config::Settings;

fn main() {
    let settings: Settings = Settings::load_settings();
    println!("{:#?}", settings);
}

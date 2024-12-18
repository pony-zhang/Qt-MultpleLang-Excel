
mod excel;
mod xml;
mod config;
use excel::*;
use xml::*;
use config::load_config;
use std::env::set_var;

fn main() {
    // set_var("RUST_BACKTRACE", "1");
    let config = load_config();
    let excel_path = config.location.first().unwrap().source_excel.as_str();
    let ts_path = config.location.first().unwrap().traget_ts.as_str();
    let sheet = config.sheet.as_str();
    let translations = read_excel(excel_path,sheet);

    config.location.iter().for_each(|location| {
        let ts_path = location.traget_ts.as_str();
        modify_ts_file(ts_path, &translations, &config);
    } );
    // modify_ts_file(ts_path, translations, &config);
}

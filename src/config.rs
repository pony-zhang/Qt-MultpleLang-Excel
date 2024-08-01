use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
pub struct LangData {
    pub lang: Vec<Lang>,
    pub location: Vec<FileLocation>,
    pub sheet: String,
}

#[derive(Deserialize, Debug)]
pub struct FileLocation {
    pub source_excel: String,
    pub traget_ts: String,
}


#[derive(Deserialize, Debug)]
pub struct Lang {
    pub ts: String,
    pub excel: String,
}

pub(crate) fn load_config() -> LangData {
    // 假设 JSON 数据存储在一个名为 "data.json" 的文件中
    let file = File::open("langconfig.json").expect("Unable to open file");
    let reader = BufReader::new(file);

    // 将 JSON 数据反序列化为 LangData 结构体
    let lang_data: LangData = serde_json::from_reader(reader).expect("Unable to parse JSON");

    lang_data
}

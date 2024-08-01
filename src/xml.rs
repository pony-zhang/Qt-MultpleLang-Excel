use calamine::{open_workbook, Reader, Xlsx};
use quick_xml::de::{from_reader, from_str};
use std::collections::HashMap;
use std::fs::File;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Reader as XmlReader;
use quick_xml::Writer;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};
use quick_xml::se::{to_string, to_writer, Serializer};
use std::fmt::Write as fmtw;
use serde::{Deserialize, Serialize};

use crate::config::LangData;


#[derive(Serialize, Deserialize, Debug)]
pub struct TS {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@language")]
    pub language: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub context: Vec<Context>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Context {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub name: String,
    pub message: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub location: Vec<Location>,
    pub source: String,
    pub translation: Translation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    #[serde(rename = "@filename")]
    pub filename: String,
    #[serde(rename = "@line")]
    pub line: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Translation {
    #[serde(rename = "@type")]
    pub translation_type: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}



// 修改 TS 文件，根据 Excel 文件中的翻译内容进行替换
pub fn modify_ts_file(ts_path: &str, translations: HashMap<String, HashMap<String, String>>, config:&LangData) {
    // 打开 TS 文件
    let file = File::open(ts_path).unwrap();
    // 创建缓冲读取器
    let buf_reader = BufReader::new(file);

    let mut ts: TS = from_reader(buf_reader).unwrap();

    // 当前语言类型
    let cur_lang = &ts.language;

    let mut target_lang = Option::None;

    for iter in config.lang.iter(){
        if cur_lang.clone() == iter.ts{
            target_lang.replace(iter.excel.clone());
            break;
        }
    }
    
    if target_lang.is_none() {
        println!( "unkonw lang is {}", cur_lang);
        return;
    }

    let content = &mut ts.context;

    for content in content.iter_mut() {
        for msg in content.message.iter_mut() {
            let inner_ts = &mut msg.translation;
            let source = msg.source.clone();
            if let Some(ref finish) = inner_ts.translation_type {
                if finish.is_empty(){
                    // println!("finish is empty, inner ts is{:?}", inner_ts.text.clone())
                }
                else{
                    // println!("finish is not empty, source is {:?}, inner ts is{:?}", source, inner_ts.text.clone());
                    if let Some(value) = translations.get(&source){
                        let replace_value = value.get(&target_lang.clone().unwrap()).unwrap();
                        println!("source is {:?}, not finish replace_value is {}",source, replace_value);
                        inner_ts.text.replace(replace_value.clone());
                        inner_ts.translation_type = Option::None;
                    }
                }
            }
            else {
                
            }
            // if inner_ts.translation_type.is_some(){
            //     let source = msg.source.clone();
            //     println!("source is {:?}, inner_ts.translation_type is {}",source, inner_ts.translation_type.clone().unwrap());
            //     if let Some(value) = translations.get(&source){
            //         if target_lang.is_some(){
            //             let replace_value = value.get(&target_lang.unwrap()).unwrap();
            //             println!("source is {:?}, not finish replace_value is {}",source, replace_value);
            //             inner_ts.text.replace(replace_value.clone());
            //             inner_ts.translation_type = Option::None;
            //         }
            //     }
            // }
            // else {
            //     let source = msg.source.clone();
            //     if let Some(value) = translations.get(&source){
            //         if target_lang.is_some() && inner_ts.text.is_some(){
            //             let replace_value = value.get(&target_lang.unwrap()).unwrap();
            //             if (replace_value.clone() != inner_ts.text.clone().unwrap())
            //             {
            //                 println!("source is {:?}, update replace_value is {}, raw is {:?}",source,replace_value.clone(), inner_ts.text.clone().unwrap());
            //                 inner_ts.text.replace(replace_value.clone());    
            //             }
            //         }
            //     }
            // }
        }
    }
    // 写回到同一文件
    let mut file = File::create(ts_path).unwrap();
    // let mut buf_write = String::new();
    let xml = to_string(&ts).unwrap();
    // let modified_data = buf_write.replace(">", ">\n");
    file.write_all(xml.as_bytes()).unwrap();
}

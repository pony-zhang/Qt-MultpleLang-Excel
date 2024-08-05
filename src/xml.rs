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
pub fn modify_ts_file(ts_path: &str, translations: &HashMap<String, HashMap<String, String>>, config:&LangData) {
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
        }
    }
    // 写回到同一文件
    let file = File::create(ts_path).unwrap();
    // let mut buf_write = String::new();
    let xml_string = to_string(&ts).unwrap();

    let mut buf_writer = BufWriter::new(file);

    // 创建一个XML写入器
    let mut writer = Writer::new_with_indent(&mut buf_writer, b' ', 4);

    // 使用quick-xml的Reader来解析XML字符串
    let mut reader = quick_xml::Reader::from_str(xml_string.as_str());
    // reader.trim_text(true);

    // 用于存储事件的缓冲区
    let mut depth = 0;

    
    // 遍历XML事件并写入到文件
    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                writer.write_event(Event::Start(e.clone())).expect("Failed to write event");
                depth += 1;
            }
            Ok(Event::End(ref e)) => {
                depth -= 1;
                writer.write_event(Event::End(e.clone())).expect("Failed to write event");
            }
            Ok(Event::Empty(ref e)) => {
                writer.write_event(Event::Empty(e.clone())).expect("Failed to write event");
            }
            Ok(Event::Text(ref e)) => {
                writer.write_event(Event::Text(e.clone())).expect("Failed to write event");
            }
            Ok(Event::Eof) => break, // 结束
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // 忽略其他事件
        }
    }

    // 刷新缓冲区并写入文件
    writer.into_inner().flush().expect("Failed to flush buffer");
}

use calamine::{open_workbook, Reader, Xlsx};
use std::collections::HashMap;
use std::fs::File;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Reader as XmlReader;
use quick_xml::Writer;
use std::io::{BufReader, BufWriter, Cursor, Write};

// 读取 Excel 文件并返回一个包含翻译内容的哈希映射
pub fn read_excel(path: &str, sheet:&str) -> HashMap<String, HashMap<String, String>> {
    // 打开 Excel 文件
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();
    // 获取第一个工作表的数据范围
    let range = workbook.worksheet_range(sheet).unwrap();
    // 获取行迭代器
    let mut iter = range.rows();
    // 获取表头
    let headers = iter.next().unwrap();
    // 初始化翻译内容的哈希映射
    let mut translations = HashMap::new();

    // 遍历每一行数据
    for row in iter {
        // 获取 ID 列的值
        let id = row[0].to_string();
        // 初始化当前行的翻译映射
        let mut translation_map = HashMap::new();
        // 遍历每一列数据
        for (i, cell) in row.iter().enumerate() {
            // 跳过第一列（ID 列）
            if i > 0 {
                // 将列头和单元格值插入翻译映射
                translation_map.insert(headers[i].to_string(), cell.to_string());
            }
        }
        // 将当前行的翻译映射插入总的翻译哈希映射
        translations.insert(id, translation_map);
    }
    // 返回翻译内容的哈希映射
    translations
}
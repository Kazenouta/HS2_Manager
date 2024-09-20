use std::fs;
use std::path::Path;
use std::error::Error;
use std::io::{self, BufReader, Read, Cursor};
use serde::de::DeserializeOwned;
use xml::reader::{EventReader, XmlEvent};
use csv::{ReaderBuilder, WriterBuilder};
use serde::Serialize;


// 检查文件是否存在
pub fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

pub fn find_files_of_dir(dir_path: &str, suffix: &str) -> std::io::Result<Vec<String>> {
    // 检查路径是否为目录
    let dir_path = Path::new(dir_path);
    if !dir_path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Provided path is not a directory",
        ));
    }
    // 尝试读取目录
    let entries = fs::read_dir(dir_path)?;
    // 遍历目录获取所有文件的路径
    let mut files = Vec::new();
    for entry in entries {
        // 文件读取失败时打印错误并继续循环
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Failed to read entry: {:?}", e);
                continue;
            }
        };
        let path = entry.path();
        if path.is_dir() {
            files.extend(find_files_of_dir(path.to_str().unwrap(), suffix)?);
        } else if path.is_file() {
            // 如果path后缀不是suffix，则跳过
            if path.to_str().unwrap().ends_with(suffix) {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }
    Ok(files)
}

// 从文件路径中获取文件名
pub fn get_file_name(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);
    match path.file_name() {
        Some(name) => name.to_str().map(|s| s.to_string()),
        None => {
            eprintln!("Failed to get file name from path: {}", file_path);
            None
        },
    }
}

// 从文件路径的Vec中获取文件名
pub fn get_file_names(file_paths: &Vec<String>) -> Vec<String> {
    file_paths.iter().filter_map(|file_path| get_file_name(file_path)).collect()
}

// 从 zipmod 文件夹的 xml 文件中读取指定元素的值
pub fn find_xml_element(xml_data: &str, element_name: &str) -> Option<String> {
    let cursor = Cursor::new(xml_data);
    let parser = EventReader::new(cursor);
    let mut in_element_tag = false;
    let mut element_content = String::new();

    for event in parser  {
        match event {
            Ok(XmlEvent::StartElement { name, .. }) 
                if name.local_name == element_name => {
                    in_element_tag = true;
            }
            Ok(XmlEvent::EndElement { name }) 
                if name.local_name == element_name => {
                    in_element_tag = false;
            }
            Ok(XmlEvent::Characters(text)) if in_element_tag => {
                element_content.push_str(&text);
            }
            Err(e) => {
                eprintln!("XML parsing error: {:?}", e);
                return None; // 遇到错误返回 None
            }
            _ => {}
        }
    }

    if !element_content.is_empty() {
        Some(element_content)
    } else {
        None
    }
}

// 写入csv文件
pub fn to_csv<T: Serialize>(file_path: &str, data_vec: &[T], mode: &str) -> Result<(), std::io::Error> {
    // 校验模式并设置 append 标志
    let append = match mode {
        "a" => true,
        "w" => false,
        _ => return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid mode, should be 'a' or 'w'",
        )),
    };

    // 检查文件是否存在
    let file_exists = file_exists(file_path);

    // 打开或创建文件
    let file = if append && file_exists {
        fs::OpenOptions::new().append(true).create(true).open(file_path)?
    } else {
        fs::File::create(file_path)?
    };

    // 如果文件为空，改为不追加
    let is_empty = file.metadata()?.len() == 0;
    let mut wtr = WriterBuilder::new().has_headers(!append || is_empty).from_writer(file);
    
    for data in data_vec {
        wtr.serialize(data)?;
    }
    wtr.flush()?; // 确保所有数据写入文件
    Ok(())
}

// 从csv文件中读取数据
pub fn read_csv<T: DeserializeOwned>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let file = fs::File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true) // 如果有表头，则设置为 true
        .from_reader(file);

    let mut records = Vec::new();
    
    let count: usize = rdr.records().count(); // 假设使用的是一个可以返回记录的迭代器
    println!("元素个数: {}", count);
    // 逐行读取 CSV 数据并解析为类型 T
    for result in rdr.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => print!("Error deserializing: {}", e)
        }
    }
    Ok(records) // 返回解析后的记录
}

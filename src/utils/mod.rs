use std::fs;
use std::path::Path;
#[allow(unused_imports)]
use std::io::{self, BufReader, Read, Cursor};
use xml::reader::{EventReader, XmlEvent};


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

// 从 zipmod 文件夹的 xml 文件中读取指定元素的值
pub fn find_xml_element(xml_data: &String, element_name: &str) -> Option<String> {
    let cursor = Cursor::new(xml_data);
    let parser = EventReader::new(cursor);
    let mut in_element_tag = false;
    let mut element_content = String::new();

    for event in parser  {
        match event {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == element_name {
                    in_element_tag = true;
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == element_name {
                    in_element_tag = false;
                }
            }
            Ok(XmlEvent::Characters(text)) => {
                if in_element_tag {
                    element_content.push_str(&text);
                }
            }
            _ => {}
        }
    }

    if element_content.is_empty() {
        None
    } else {
        Some(element_content)
    }
}

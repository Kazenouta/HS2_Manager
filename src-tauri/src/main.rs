// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
use serde::{de::value, Deserialize};
use serde_json::json;
use tauri::{command, InvokeError};
mod utils;
mod pages;
// mod error;
// mod prelude;
// use crate::prelude::*;

#[command]
fn update_config(name: String, value: String) -> Result<(), InvokeError>  {
    match utils::config::update_config(name, value) {
        Ok(_) => Ok(()),
        Err(e) => Err(InvokeError::from(e.to_string())),
    }
}

#[command]
fn get_base_info() -> Result<String, InvokeError> {
    match utils::config::read_config() {
        Ok(info) => Ok(json!(info).to_string()),
        Err(e) => Err(InvokeError::from(e.to_string())),
    }
}


// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![get_base_info, update_config])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }


// fn main() {
//     pages::zipmod::main();

// }


use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};

const BUFFER_SIZE: usize = 4096;

fn search_for_sequence<R: Read + Seek>(stream: &mut R, sequence: &[u8]) -> io::Result<i64> {
    let orig_pos = stream.seek(std::io::SeekFrom::Current(0))?; // 保存原始位置
    let mut buffer = [0; BUFFER_SIZE];
    
    let scan_byte = sequence[0];

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // EOF
        }

        let mut i = 0;
        while i < bytes_read {
            if buffer[i] != scan_byte {
                i += 1;
                continue;
            }

            let mut flag = true;
            for x in 1..sequence.len() {
                i += 1;

                if i >= bytes_read {
                    // 读取新的一块
                    let new_bytes_read = stream.read(&mut buffer)?;
                    if new_bytes_read == 0 {
                        return Ok(0); // EOF
                    }
                    i = 0; // 重置索引到新缓冲区开始
                }

                if buffer[i] != sequence[x] {
                    flag = false;
                    break;
                }
            }

            if flag {
                let result = (stream.seek(std::io::SeekFrom::Current(0))? + 1) as i64 - (BUFFER_SIZE as i64 - i as i64) - sequence.len() as i64;
                stream.seek(std::io::SeekFrom::Start(orig_pos))?; // 还原原始位置
                return Ok(result);
            }
        }
    }

    stream.seek(std::io::SeekFrom::Start(orig_pos))?; // 还原原始位置
    Ok(0)
}

const PNG_END_CHUNK: [u8; 8] = [0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82];

fn search_for_png_end<R: Read + Seek>(stream: &mut R) -> io::Result<i64> {
    let result = search_for_sequence(stream, &PNG_END_CHUNK)?;
    if result >= 0 {
        return Ok(result + PNG_END_CHUNK.len() as i64);
    }
    Ok(result)
}

// fn main() -> io::Result<()> {
//     // 示例使用
//     let file_path = "/mnt/e/Games/HS2/UserData/chara/female/AAAA/【腋猫子】魔法萝莉.png";  // 替换为你的文件路径

//     let mut file = std::fs::File::open(file_path)?;

//     let png_end_pos = search_for_png_end(&mut file)?;
    
//     if png_end_pos >= 0 {
//         println!("PNG结束位置: {}", png_end_pos);
//     } else {
//         println!("未找到PNG结束标志");
//     }

//     Ok(())
// }

fn read_string<R: Read + Seek>(reader: &mut R) -> io::Result<String> {
    // 读取字符串长度（以字符为单位）
    let mut length_buffer = [0; 4];

    reader.read_exact(&mut length_buffer)?;
    // 获取文件的当前长度（指针位置）
    let current_pos = reader.stream_position()?;
    // 移动到文件的末尾并获取长度
    let length = reader.seek(std::io::SeekFrom::End(0))?;
    // 移动回原来的位置
    reader.seek(std::io::SeekFrom::Start(current_pos))?;

    println!("reader 当前长度字节: {:?}", length);

    let length = u32::from_le_bytes(length_buffer); // 使用 u32 读取长度

    println!("字符串长度: {}", length);
    // 检查长度是否过大
    if length > (usize::MAX / 2) as u32 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "字符串长度过大，可能导致溢出"));
    }

    let length = length as usize; // 转换为 usize 用于切片

    // 读取 UTF-16 编码的内容（每个字符占两个字节）
    let mut string_buffer = vec![0; length * 2]; // 每个字符两个字节
    reader.read_exact(&mut string_buffer)?;

    // 将字节转换为字符
    let mut result = String::new();
    for chunk in string_buffer.chunks(2) {
        if chunk.len() == 2 {
            let code_unit = u16::from_le_bytes([chunk[0], chunk[1]]);
            if let Some(c) = char::from_u32(code_unit as u32) {
                result.push(c);
            } else {
                result.push('\u{FFFD}'); // 使用替代字符处理无效情况
            }
        }
    }

    Ok(result)
}

fn main() -> io::Result<()> {
    let file_path = "/mnt/e/Games/HS2/UserData/chara/female/AAAA/【腋猫子】嫁猫.png";  // 替换为你的文件路径
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let png_end = search_for_png_end(&mut reader)?;
    println!("PNG结束位置: {}", png_end);
    // 文件开始位置
    println!("文件开始位置: {}", reader.stream_position()?);
    // 文件结束位置
    println!("文件结束位置: {}", reader.seek(SeekFrom::End(0))?);
    if png_end == 0 {
        println!("未找到有效的PNG结束标志");
        return Ok(());  // 未找到有效的PNG结束标志
    }

    reader.seek(SeekFrom::Start(png_end as u64))?;

    // 读取加载产品编号
    let mut load_product_no = [0; 4];
    reader.read_exact(&mut load_product_no)?;
    let load_product_no = i32::from_le_bytes(load_product_no);
    println!("加载产品编号: {}", load_product_no);
    
    // 如果产品编号大于100，返回
    if load_product_no > 100 {
        return Ok(());
    }

    // 读取标记字符串
    // let mut marker = String::new();
    // reader.read_line(&mut marker)?;
    let marker = read_string(&mut reader)?;
    println!("文件开始位置: {}", reader.stream_position()?);

    
    
    Ok(())
}

// 假设这是根据标记获取游戏类型的函数
fn get_game_type(marker: &str, _flag: bool) -> String {
    // ... 根据marker返回游戏类型的逻辑
    marker.to_string()  // 示例直接返回标记
}

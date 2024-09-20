use core::error;
use std::fmt::format;
// use std::iter::Zip;
// use crate::{utils};
use std::path::{PathBuf, Path};
use std::vec;
use serde::{Deserialize, Serialize};
use tauri::api::dir;
use tauri::utils::config;
use tauri::AppHandle;
use zip::ZipArchive;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use crate::utils::files::{find_xml_element, find_files_of_dir, to_csv, read_csv};
use crate::utils::config::{read_config, Config};
use polars::prelude::CsvReader;
use std::collections::HashSet;
use png::Decoder;

// 创建zipmod文件的struct, 包含guid, 作者名, 名称, 版本等
#[derive(Debug, Serialize, Deserialize)]
struct ZipMod {
    path: String,
    guid: String,
    author: String,
    name: String,    
    version: String,
    new_name: String,
    size: String,  // 文件的大小, 单位为MB
}
// zipmod文件重命名后的new_name格式为[作者名] [名称] v[版本].zipmod
impl ZipMod {

    // 读取zipmod文件中的信息
    fn read_file(zipmod_path: &str) -> Option<Self> {
        // 获取文件大小, 单位为MB
        let size = match fs::metadata(zipmod_path) {
            Ok(metadata) => metadata.len() as f32 / 1024.0 / 1024.0,
            Err(_) => {
                eprint!("{}: 获取文件大小失败", zipmod_path);
                return None
            },
        };

        let zipmod_path_buf = PathBuf::from(zipmod_path);
        let zipmod_file = File::open(zipmod_path_buf).ok()?;
        let mut archive = ZipArchive::new(zipmod_file).ok()?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).ok()?;
            if !file.name().ends_with(".xml") {
                continue
            }
            let mut xml_data  = String::new();
            file.read_to_string(&mut xml_data ).ok()?;
            // guid是唯一标识, 该字段不能为空值
            let guid = match find_xml_element(&xml_data, "guid") {
                Some(guid) => guid,
                None => {
                    println!("{}: guid not found", file.name());
                    continue;
                },
            };
            // 获取作者名称
            let mut author = find_xml_element(&xml_data, "author").unwrap_or_else(|| String::from("Unknown"));
            author = author.replace("/", "_").replace(".", "");  // 将作者名中 / 转为 _
            author = author.replace("*", "").replace("[", "").replace("]", ""); // 将 * [ ] 去掉
            author = author.replace("【", "").replace("】", ""); // 将 【 】 去掉
            author = author.to_lowercase();  // 将大写字母转为小写字母
            // 获取mod名称, 没有name用guid代替
            let name = find_xml_element(&xml_data, "name").unwrap_or_else(|| guid.clone());
            // 获取版本号, 没有version用1.0代替
            let version = find_xml_element(&xml_data, "version").unwrap_or_else(|| String::from("1.0"));
            
            
            // println!("{}: guid={}, author={}, name={}, version={}", file.name(), guid, author, name, version);
            return Some(Self {
                path: zipmod_path.to_string(),
                guid: guid.to_string(),
                author: author.to_string(),
                name: name.to_string(),
                version: version.to_string(),
                new_name: format!("[{}] {} v{}.zipmod", author, name, version),
                size: format!("{:.2}", size),
            });
        } 
        // 如果没有找到xml文件, 则返回None
        None
    }

    // 刷新zipmod文件列表
    fn refresh_info_list(is_raw: bool) -> io::Result<()>{
        let config = match read_config() {
            Ok(config) => config,
            Err(e) => {
                eprintln!("读取配置文件失败: {}", e);
                return Err(io::Error::new(io::ErrorKind::Other, "读取配置文件失败"));
            },
        };

        let mut dir_mods = String::new();
        let mut path_zipmod_info = String::new();
        if is_raw {
            dir_mods = config.dir_mods_raw;
            path_zipmod_info = config.path_zipmod_info_raw;
        } else {
            dir_mods = config.dir_mods;
            path_zipmod_info = config.path_zipmod_info;
        }
        
        let path_zipmod_list = find_files_of_dir(&dir_mods, ".zipmod")?;
        let mut zipmod_info_list = Vec::new();
        for path in path_zipmod_list {
            match ZipMod::read_file(&path) {
                Some(zipmod) => {
                    zipmod_info_list.push(zipmod);
                }
                None => {
                    eprintln!("{}: 读取mod信息失败", &path);
                }
            };
        }

        match to_csv(&path_zipmod_info, &zipmod_info_list, "w") {
            Ok(_) => println!("写入csv文件成功"),
            Err(e) => eprintln!("写入csv文件失败: {}", e),
        };

        Ok(())
    }
    
    // 获取已有的zipmod文件列表
    fn get_info_list() -> Result<Vec<ZipMod>, io::Error> {
        match read_csv::<ZipMod>("./data/zipmod_info_list.csv") {
            Ok(info_list) => Ok(info_list),
            Err(e) => {
                eprintln!("读取csv文件失败: {}", e);
                return Err(io::Error::new(io::ErrorKind::Other, "读取csv文件失败"));
            },
        }
    }

    // 添加zipmod文件信息
    fn append_info_list(zipmod: &ZipMod) -> io::Result<()> {
        match to_csv("./data/zipmod_info_list.csv", &[zipmod], "a") {
            Ok(_) => println!("添加zipmod信息成功"),
            Err(e) => eprintln!("添加zipmod信息失败: {}", e),
        };
        Ok(())
    }

    // 将zipmod文件重命名并移动到指定目录下
    fn rename_and_restore(zipmod: &ZipMod) -> io::Result<()> {
        let dir_mymods = match read_config() {
            Ok(config) => config.dir_mymods,
            Err(e) => {
                eprintln!("读取配置文件失败: {}", e);
                return Err(io::Error::new(io::ErrorKind::Other, "读取配置文件失败"));
            },
        };
        // 如果to_dir不存在, 抛出错误
        let to_dir = PathBuf::from(dir_mymods);
        if !to_dir.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "to_dir not found"));
        }
        // 在to_dir中创建作者目录
        let to_dir_author = to_dir.join(&zipmod.author);
        if !to_dir_author.exists() {
            fs::create_dir_all(&to_dir_author)?;
        }
        // 构造新的zipmod文件路径
        let file_name = &zipmod.new_name.replace("/", "_").replace(",", " "); // 将文件名中的 / 转为 _
        let to_path = &to_dir_author.join(&file_name);
        let to_path = PathBuf::from(to_path);
        // 如果文件已经不存在, 则直接复制
        if !to_path.exists() {
            fs::copy(&zipmod.path, &to_path)?;
            ZipMod::append_info_list(&zipmod)?;
        }
        Ok(())
    }

    
}



pub fn main() {
    // run();

    // ZipMod::refresh_info_list(true).unwrap();

    // match ZipMod::get_info_list() {
    //     Ok(info_list) => {
    //         print!("info_list: {:#?}", info_list.len());
    //     }
    //     Err(e) => {
    //         eprintln!("get_info_list failed: {}", e);
    //     }
    // };
    // print!("info_list: {:#?}", info_list[info_list.len()-1]);

    // 筛选出为存储的zipmod文件
    // let config = read_config().unwrap();
    // let vec_mods: Vec<ZipMod> = read_csv(config.path_zipmod_info.as_str()).unwrap();
    // let set_mods: HashSet<String> = vec_mods.iter().map(|x| x.guid.to_string()).collect();
    // let vec_raw: Vec<ZipMod> = read_csv(config.path_zipmod_info_raw.as_str()).unwrap();
    // let set_raw: HashSet<String> = vec_raw.iter().map(|x| x.guid.to_string()).collect();

    // let vec_raw_filtered: Vec<ZipMod> = vec_raw.into_iter().filter(|x| !set_mods.contains(&x.guid.to_string())).collect();
    // print!("mods: {:#?}, raw: {:#?}, filtered_len: {:#?}", 
    //     set_mods.len(),
    //     set_raw.len(),
    //     vec_raw_filtered.len());

    let path = Path::new("/mnt/e/Games/HS2/UserData/chara/female/AAAA/【腋猫子】魔法萝莉.png");
    let file = File::open(&path).expect("无法打开文件");

    // 创建解码器
    let mut decoder = Decoder::new(file);

    // 读取图像信息
    let reader = decoder.read_info().expect("无法读取PNG文件信息");

    // 输出图像信息
    println!("宽度: {}", reader.info().width);
    println!("高度: {}", reader.info().height);
    println!("颜色类型: {:?}", reader.info().color_type);

}
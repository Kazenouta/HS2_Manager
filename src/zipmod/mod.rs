// 用于将杂乱的zipmod文件用统一规范命名并移动到指定目录下
use crate::{utils};
use std::path::{PathBuf};
use zip::ZipArchive;
use std::fs::{self, File};
use std::io::{self, Read};

#[allow(dead_code)]


// 创建zipmod文件的struct, 包含guid, 作者名, 名称, 版本等
#[derive(Debug)]
struct ZipMod {
    path: String,
    guid: String,
    author: String,
    name: String,    
    version: String,
    new_name: String,
}
// zipmod文件重命名后的new_name格式为[作者名] [名称] v[版本].zipmod
impl ZipMod {
    // 构造函数
    fn new(path: &String,guid: &String, author: &String, name: &String, version: &String) -> Self {
        let new_name = format!("[{}] {} v{}.zipmod", author, name, version);
        Self {
            path: path.clone(),
            guid: guid.clone(),
            author: author.clone(),
            name: name.clone(),
            version: version.clone(),
            new_name: new_name,
        }
    }
}

// 获取文件的guid
fn read_zipmod_guid(zipmod_path: &String) -> Option<String> {
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
        let guid = match utils::find_xml_element(&xml_data, "guid") {
            Some(guid) => guid,
            None => {
                println!("{}: guid not found", file.name());
                continue;
            },
        };
        return Some(guid);
    } 
    None
}

// 从指定的zipmod文件路径中读取该文件的基本信息并存入ZipMod结构体中
fn read_zipmod_info(zipmod_path: &String) -> Option<ZipMod> {
    let zipmod_path_buf = PathBuf::from(zipmod_path);
    let zipmod_file = File::open(zipmod_path_buf).ok()?;
    let mut archive = ZipArchive::new(zipmod_file).ok()?;

    let mut zipmod = ZipMod::new(
        &String::new(), 
        &String::new(), 
        &String::new(), 
        &String::new(),
        & String::new());

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).ok()?;
        if !file.name().ends_with(".xml") {
            continue
        }
        let mut xml_data  = String::new();
        file.read_to_string(&mut xml_data ).ok()?;
        // guid是唯一标识, 该字段不能为空值
        let guid = match utils::find_xml_element(&xml_data, "guid") {
            Some(guid) => guid,
            None => {
                println!("{}: guid not found", file.name());
                continue;
            },
        };
        // 获取作者名称
        let mut author = utils::find_xml_element(&xml_data, "author").unwrap_or_else(|| String::from("Unknown"));
        author = author.replace("/", "_").replace(".", "");  // 将作者名中 / 转为 _
        author = author.replace("*", "").replace("[", "").replace("]", ""); // 将 * [ ] 去掉
        author = author.to_lowercase();  // 将大写字母转为小写字母
        // 获取mod名称, 没有name用guid代替
        let name = utils::find_xml_element(&xml_data, "name").unwrap_or_else(|| guid.clone());
        // 获取版本号, 没有version用1.0代替
        let version = utils::find_xml_element(&xml_data, "version").unwrap_or_else(|| String::from("1.0"));
        // println!("{}: guid={}, author={}, name={}, version={}", file.name(), guid, author, name, version);
        zipmod = ZipMod::new(&zipmod_path, &guid, &author, &name, &version);
    } 
    Some(zipmod)
}


// 将zipmod文件重命名并移动到指定目录下
fn restore_renamed_zipmod(zipmod: &ZipMod, to_dir: &str) -> io::Result<()> {
    // 如果to_dir不存在, 抛出错误
    println!("{}",  &to_dir);

    let to_dir = PathBuf::from(to_dir);
    println!("{}",  &to_dir.display());

    if !to_dir.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "to_dir not found"));
    }
    println!("{}",  &to_dir.display());

    // 在to_dir中创建作者目录
    let to_dir_author = to_dir.join(&zipmod.author);
    if !to_dir_author.exists() {
        fs::create_dir_all(&to_dir_author)?;
    }
    // 构造新的zipmod文件路径
    let to_path = &to_dir_author.join(&zipmod.new_name);
    // 将路径中的 / 转为 _
    let to_path = to_path.to_str().unwrap().replace("/", "_");
    let to_path = PathBuf::from(to_path);
    // 如果文件已经存在, 则跳过
    if !to_path.exists() {
        fs::copy(&zipmod.path, &to_path)?;
    }
    // 移动文件
    Ok(())
}

pub fn run() {
    let suffix = ".zipmod";
    //存储所有处理后zipmod文件的文件夹
    let zipmod_dir = "/mnt/e/Games/HS2/mods";
    // 改名后存储的目录
    let to_dir = "/mnt/e/Games/HS2/mods/MyMods";
    // 指定要读取的目录
    let from_dir = "/mnt/d/Games/人物卡扩展/BepInEx_bak/mods";
    // let from_dir = "/mnt/e/HS2/mods/Sideloader Modpack/";

    // 全部处理过的zipmod文件
    let tot_files = utils::find_files_of_dir(zipmod_dir, suffix).unwrap_or_default();
    // let mut tot_guids = Vec::new();
    // for file in tot_files {
    //     let guid = read_zipmod_guid(&file).unwrap();
    //     tot_guids.push(guid);
    // }
    // println!("tot_guids: {:?}", tot_guids.len());
    // 待处理zipmop文件
    let files = utils::find_files_of_dir(from_dir, suffix).unwrap_or_default();
    println!("found {} zipmod files", files.len());

    for file in files {
        // 读取zipmod文件信息
        let _ = match read_zipmod_info(&file) {
            Some(zipmod) => {
                // // 如果guid已经存在, 则跳过
                // if tot_guids.contains(&zipmod.guid) {
                //     continue;
                // }
                // 重命名并移动zipmod文件
                println!("{}",  &to_dir);
                match restore_renamed_zipmod(&zipmod, &to_dir) {
                    Ok(_) => {
                        println!("{}: renamed and moved to {}", &zipmod.path, &to_dir);
                        continue;
                    },
                    Err(e) => {
                        println!("{}: error: {}", file, e);
                    }
                }
                continue
            },
            None => {
                println!("{}: invalid zipmod file", file);
                continue;
            }
        };
    }
}   

pub fn main() {
    run();
    // let path = "/mnt/e/HS2/mods/MyMods/国漫/dl51-长生殿.zipmod";
    // let zipmod_info = read_zipmod_info(&path.to_string());
    // println!("{:?}", zipmod_info);
}
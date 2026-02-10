//！这是一个文件读取库，支持读取多种文件格式的文件内容，如TXT、CSV、Markdown、Excel、PDF、ZIP、RAR、JSON、XML、PPT文本等。
//！支持读取各个类型文件的属性信息。
//！提供灵活的读取方式：支持按行、按块、按字节等方式读取文件内容，以满足不同场景下的需求。
//！实现对PDF文件内文本内容的提取，支持读取所有页的文本内容。
//！实现对Excel文件内文本内容的提取，支持xls和xlsx格式。
//！实现对PPT文件内文本内容的提取，支持ppt和pptx格式，支持读取所有幻灯片内的文本内容，支持处理文本框、标题、备注等不同文本元素。
//！实现对压缩包（ZIP、RAR等）内文件内容的读取。支持读取压缩包内所有文件和文件夹的结构，支持对压缩包内指定文件内容的提取。
// use std::collections::BTreeMap;
// use std::env::join_paths;
// use std::f32::consts::E;
// use std::fmt::Debug;
use core::str;
// use std::ffi::OsStr;


use std::fs::{self, File, Metadata};
use std::io::{BufRead, BufReader, Error, Read, Result};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use calamine::{open_workbook_auto, Data, Reader, Xlsx};
// use clap::builder::Str;
// use clap::Parser as CParser;
use csv::ReaderBuilder;
// use lopdf::{Document, Object};
// use pdf::content::Content;
// use pdf::file;
use pulldown_cmark::{html, Parser};
use quick_xml::events::Event;
use rayon::iter::ParallelIterator;
// use rayon::{join, range};
// use serde::{Deserialize, Serialize};
use serde_json::{from_reader, from_str, Value};
use serde_json;
use tempfile::{tempdir, NamedTempFile};
use unrar::error::UnrarError;
// use unrar::error::UnrarError;
// #[cfg(feature = "async")]
// use tokio::runtime::Builder;
use unrar::{Archive, UnrarResult};
use xml::reader::{EventReader, XmlEvent};
use zip::read::ZipArchive;
// use calamine::{Range};
// use std::io::{Error as IoError};
use std::io::prelude::*;
use std::io::Cursor;
// use quick_xml::events::{BytesStart};
// use quick_xml::Error as QuickXmlError;
// use quick_xml::events::{};
use std::io::{self};
// use std::borrow::Cow;
// use zip::ZipWriter;
// use zip::read::ZipFile;
// use quick_xml::events;



// use std::fs::File as StdFile;
use pdf_extract::extract_text;
// use std::process::Command;
// extern crate rayon;
use rayon::prelude::*;
// 定义获取属性的库
// use std::fs::{Permissions};
// use std::fs::{Metadata};
// use std::time::{SystemTime, UNIX_EPOCH};
// use chrono::format::parse;
use chrono::{DateTime, Local};
// use libc::{fstat, stat};
// use libc::{c_int, S_IFMT, S_IFREG};

// 定义读取文件结构体
pub struct FileReader;
// 定义需要忽略的字符串数组
static IGNORE: &[&str] = &[
    "Length",
    "BBox",
    "FormType",
    "Matrix",
    "Resources",
    "Type",
    "XObject",
    "Subtype",
    "Filter",
    "ColorSpace",
    "Width",
    "Height",
    "BitsPerComponent",
    "Length1",
    "Length2",
    "Length3",
    "PTEX.FileName",
    "PTEX.PageNumber",
    "PTEX.InfoDict",
    "FontDescriptor",
    "ExtGState",
    "Font",
    "MediaBox",
    "Annot",
];

// 定义常量
pub const FILE_OPEN_FAILED: &str = "文件打开失败";
pub const FILE_NOT_FOUND: &str = "文件未找到";

/// 详细实现了文件读取功能，提供了读取各种类型文件内容的方法，如TXT、CSV、Markdown、Excel、PDF、ZIP、RAR、JSON、XML、PPT
/// 同时支持多种方式读取文件内容，包括按行读取、按字节读取等。
/// 支持读取各个类型文件的属性信息。
impl FileReader {


    /// 一次性读取文本文件内容，适合小文件
    /// 
    /// ### 参数
    /// - `file_path:<&str>`: 文件路径
    /// 
    /// ### 返回值
    /// - `Result<String>`:返回字符串，字符串包含整个TXT文件的内容
    /// 
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_txt("src/test_examples/test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("文本文件内容: \n {}", content),
    ///   Err(e) => println!("读取文本文件失败: {}", e), 
    /// }
    pub fn read_txt(file_path: &str) -> Result<String> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".txt"), "文件类型错误,请检查文件类型是否为txt文件");

        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file); // 缓冲器
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(content)
    }

    /// 按行读取文本文件内容
    /// 
    /// ### 参数
    /// - `file_path:AsRef<Path>`: 文件路径
    /// 
    /// ### 返回值
    /// - `Result<Vec<String>>`:返回一个字符串可变数组，一个字符串一个内容
    /// 
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_txt_by_line("src/test_examples/test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("文本文件内容: \n {:?}",content),
    ///   Err(e) => println!("读取文本文件失败: {}", e), 
    /// }
    pub fn read_txt_by_line<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<String>> {
        assert!(std::fs::metadata(file_path.as_ref()).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.as_ref().to_str().unwrap().ends_with(".txt"), "文件类型错误,请检查文件类型是否为txt文件");
    
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();
    
        for line in reader.lines() {
            let line = line?;
            let trimmed_line = line.trim_end().to_string();
            lines.push(trimmed_line);
        }
    
        Ok(lines)
    }

    /// 按字节读取文本文件内容
    /// 
    /// ### 参数
    /// - `file_path:AsRef<Path>`: 文件路径
    /// 
    /// ### 返回值
    /// - `Result<Vec<u8>>`:返回一个u8字节数组，数组包含文本内容
    /// 
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_txt_by_byte("src/test_examples/test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("文本文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取文本文件失败: {}", e), 
    /// }
    pub fn read_txt_by_byte<P: AsRef<Path>>(file_path: P) -> Result<Vec<u8>> {
        assert!(fs::metadata(file_path.as_ref()).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.as_ref().to_str().unwrap().ends_with(".txt"), "文件类型错误,请检查文件类型是否为txt文件");

        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = [0u8;1]; // 默认值0，类型u8，数组大小1
        let mut bytes = Vec::new();
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if n == 1 {
                        bytes.push(buffer[0]);
                    } else {
                        bytes.extend_from_slice(&mut buffer[..n]);  // 这里是根据函数本身返回值需要
                    }

                },
                Err(e) => return Err(e),
            }
        }

        Ok(bytes)
    }

    /// 按块读取TXT文件
    ///
    /// ### 参数
    /// - `file_path:<&str>`: 文件路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    ///
    /// ### 返回值
    /// - `Result<Vec<String>>`:返回一个包含每个块的向量，每个块都是一个字符串
    /// 
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_txt_by_block("src/test_examples/test_txt.txt",4);
    /// match res {
    ///   Ok(content) => println!("文本文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取文本文件失败: {}", e), 
    /// }
    pub fn read_txt_by_block(file_path: &str, block_size: usize) -> Result<Vec<String>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".txt"), "文件类型错误,请检查文件类型是否为txt文件");

        let file = File::open(file_path).expect(FILE_OPEN_FAILED);
        let mut reader = BufReader::new(file);
        let mut blocks = Vec::new();
        let mut buffer = vec![0; block_size]; //创建一个包含block_size个0元素的向量。

        loop {
            let bytes_read = reader.read(&mut buffer)?; // 一次性读取buffer大小字节，不够，读取剩下的
            if bytes_read == 0 {
                break; // 读取到文件末尾
            }
            // 将读取的字节转换为字符串并添加到块中
            blocks.push(String::from_utf8_lossy(&buffer[..bytes_read]).to_string());
        }
        Ok(blocks)
    }

    /// 按行读取csv文件
    ///
    /// ### 参数
    /// - `file_path:<&str>`: 文件路径
    ///
    /// ### 返回值
    /// - `Result<Vec<Vec<String>>>`:返回一个二维String数组，一个字符串为一个数据
    /// 
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_csv_by_line("src/test_examples/test_csv.csv");
    /// match res {
    ///   Ok(content) => println!("csv文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取csv文件失败: {}", e), 
    /// }
    pub fn read_csv_by_line(file_path: &str) -> io::Result<Vec<Vec<String>>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".csv"), "文件类型错误,请检查文件类型是否为csv文件");
    
        let file = File::open(file_path)?;
        let mut rdr = ReaderBuilder::new().from_reader(file);
    
        let records = Arc::new(Mutex::new(Vec::new()));
    
        rdr.records()
            .par_bridge() // Convert the iterator to a parallel iterator
            .for_each(|result| {
                let record = result.expect("读取记录失败");
                let record_vec: Vec<String> = record.iter().map(|s| s.to_string()).collect();
    
                let mut records = records.lock().expect("获取互斥锁失败");
                records.push(record_vec);
            });
    
        let records = Arc::try_unwrap(records).expect("Arc解包失败").into_inner().expect("Mutex解包失败");
    
        Ok(records)
    }

    /// 按列读取csv文件
    ///
    /// ### 参数
    /// - `file_path:<&str>`: 文件路径
    ///
    /// ### 返回值
    /// - `Result<Vec<Vec<String>>>`:返回一个二维String数组，一个字符串为一个数据
    /// 
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_csv_by_column("src/test_examples/test_csv.csv");
    /// match res {
    ///   Ok(content) => println!("csv文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取csv文件失败: {}", e), 
    /// }
    pub fn read_csv_by_column(file_path: &str) -> Result<Vec<Vec<String>>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".csv"), "文件类型错误,请检查文件类型是否为csv文件");

        let file = File::open(file_path).expect(FILE_OPEN_FAILED);
        let mut rdr = ReaderBuilder::new().from_reader(file);
        // let mut records = Vec::new();  //之前的代码
        let mut records:Vec<Vec<String>> = Vec::new();

        // 读取所有行
        for result in rdr.records() {
            let record = result?;
            records.push(record.iter().map(|s| s.to_string()).collect());
        }
        // println!("temp test records is {:?}", records);
        // 按列转换
        let mut columns = vec![Vec::new(); records[0].len()];
        for record in records {
            for (i, field) in record.into_iter().enumerate() {
                columns[i].push(field);
            }
        }

        Ok(columns)
    }


    /// 按块读取CSV文件
    ///
    /// ### 参数
    /// - `file_path:<&str>`: 文件路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    ///
    /// ### 返回值
    /// - `Result<Vec<Vec<Vec<String>>>`:返回一个向量，每个向量元素即为csv文件块的内容，用二维向量存储
    /// 
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_csv_by_block("src/test_examples/test_csv.csv",4);
    /// match res {
    ///   Ok(content) => println!("csv文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取csv文件失败: {}", e), 
    /// }
    pub fn read_csv_by_block(file_path: &str, block_size: usize) -> Result<Vec<Vec<Vec<String>>>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".csv"), "文件类型错误,请检查文件类型是否为csv文件");

        let file = File::open(file_path).expect(FILE_OPEN_FAILED);
        let mut rdr = ReaderBuilder::new().from_reader(file);
        let mut blocks = Vec::new();
        let mut current_block = Vec::new();

        for result in rdr.records() {
            let record = result.expect("Failed to read record");  // 加了一个expect
            // current_block.push(record.iter().map(|s| s.to_string()).collect());
            let string_vec: Vec<String> = record.iter().map(String::from).collect(); // 一行数据
            current_block.push(string_vec);

            if current_block.len() == block_size { // 满足指定行数，就添加块到结果中
                blocks.push(current_block.clone());
                current_block.clear();
            }
        }
        // 如果最后一个块不满，仍然保存
        if !current_block.is_empty() {
            blocks.push(current_block);
        }

        Ok(blocks)
    }


    /// 读取JSON文件内容
    /// 
    /// ### 参数
    /// - `file_path:<&str>`: 文件路径
    /// 
    /// ### 返回值
    /// - `Result<String>`: 返回字符串，字符串包含整个json文件的内容
    /// 
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_json_text("src/test_examples/test_json.json");
    /// match res {
    ///   Ok(content) => println!("json文件内容: \n {}", content),
    ///   Err(e) => println!("读取json文件失败: {}", e), 
    /// }
    pub fn read_json_text(file_path: &str) -> Result<String> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".json"), "文件类型错误,请检查文件类型是否为json文件");

        //打开文件
        let file = File::open(file_path)?;
        // 创建缓冲区
        let mut reader = BufReader::new(file); // 缓冲器
        let mut content = String::new();
        // 将文件内容读入字符串并返回
        reader.read_to_string(&mut content)?;
        Ok(content)
        
    }

    /// 动态读取JSON文件
    /// 
    /// Value 是 serde_json 库中的类型，用于表示任何 JSON 数据结构
    /// 
    /// ### 参数
    /// - `file_path:<&str>`: 文件路径
    /// 
    /// ### 返回值
    /// - `serde_json::Result<Value>`:返回Value，Value包含整个json文件的内容
    /// 
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_json_dynamic("src/test_examples/test_json.json");
    /// match res {
    ///   Ok(content) => println!("json文件内容: \n {}", content),
    ///   Err(e) => println!("读取json文件失败: {}", e), 
    /// }
    pub fn read_json_dynamic(file_path: &str) -> serde_json::Result<Value> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".json"), "文件类型错误,请检查文件类型是否为json文件");

        let file = File::open(file_path).expect("文件打开失败");
        let reader = BufReader::new(file);

        // 直接解析为serde_json::Value
        let json_data: Value = serde_json::from_reader(reader)?; // 尝试从 reader 中解析 JSON 数据

        Ok(json_data)
    }

    /// 按行读取JSONL文件
    /// 
    /// Value 是 serde_json 库中的类型，用于表示任何 JSON 数据结构
    /// 
    /// ### 参数
    /// - `file_path:<&str>`: 文件路径
    /// 
    /// ### 返回值
    /// - `Result<Vec<Value>>`:返回Value数组，Value数组包含整个jsonl文件的内容
    /// 
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_jsonl_by_line("src/test_examples/test_jsonline.jsonl");
    /// match res {
    ///   Ok(content) => println!("jsonline文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取jsonline文件失败: {}", e), 
    /// }
    pub fn read_jsonl_by_line(file_path: &str) -> Result<Vec<Value>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".jsonl"), "文件类型错误,请检查文件类型是否为jsonl文件");

        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut vec_json = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let json_value = from_str(&line)?; // from_str 是 serde_json 库中的一个函数，用于将字符串解析为 serde_json::Value 类型
            vec_json.push(json_value);
        }
        Ok(vec_json)
    }


    /// 按块读取JSON文件
    ///
    /// ### 参数
    /// - `file_path:<&str>`: 文件路径
    /// - `block_size:<usize>`: 每次读取的块大小，以对象的数量为单位
    /// 
    /// json要求每个对象（{..}）必须在数组里，jsonl 即json line是每一行一条数据，所以分两种情况考虑
    /// ### 返回值
    /// - `Result<Vec<Vec<Value>>>`:返回包含每个块的向量，每个块都是一个对象向量
    /// 
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_json_by_block("src/test_examples/test_json.json",4);
    /// match res {
    ///   Ok(content) => println!("json文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取json文件失败: {}", e), 
    /// }
    pub fn read_json_by_block(file_path: &str, block_size: usize) -> Result<Vec<Vec<Value>>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".json"), "文件类型错误,请检查文件类型是否为json文件");

        let file = File::open(file_path).expect(FILE_OPEN_FAILED);
        let mut reader = BufReader::new(file);
        //这里的 <::Value> 指定了我们期望从迭代器中得到的类型是 serde_json::Value。serde_json::Value 是一个枚举类型，
        //它可以代表 JSON 中的任何值（例如对象、数组、字符串等）。rust不认识json，所以反序列化
        // 迭代地解析 JSON 文件，而不是一次性加载整个文件到内存中，这对于处理大文件非常有用。
        // let stream = Deserializer::from_reader(reader).into_iter::<Value>(); // 会一次性读一个数组，不对
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let json_array:Value = from_str(&content)?;
        let array = json_array.as_array().ok_or(Error::new(std::io::ErrorKind::Other, "Not an array"))?;

        let mut blocks = Vec::new();
        let mut current_block = Vec::with_capacity(block_size); //创建一个具有预设元素容量 block_size 的向量，元素类型是serde_json::Value
        for value in array {
            current_block.push(value.clone());
            if current_block.len() == block_size {
                blocks.push(current_block);
                current_block = Vec::with_capacity(block_size);
            }
        }
        // 如果最后一个块不满，仍然保存
        if !current_block.is_empty() {
            blocks.push(current_block);
        }

        Ok(blocks)
    }

    /// 读取XML文件内容
    /// 
    /// ### 参数
    /// - `file_path:<&str>`: 文件路径
    /// 
    /// ### 返回值
    /// - `Result<String>`:返回字符串，字符串包含整个XML文件的内容
    /// 
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_xml_text("src/test_examples/test_xml.xml");
    /// match res {
    ///   Ok(content) => println!("xml文件内容: \n {}", content),
    ///   Err(e) => println!("读取xml文件失败: {}", e), 
    /// }
    pub fn read_xml_text(file_path: &str) -> Result<String> { // 这里是简化写法，固定了错误类型
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".xml"), "文件类型错误,请检查文件类型是否为xml文件");

        //打开文件
        let file = File::open(file_path)?;
        // 创建缓冲区
        let mut reader = BufReader::new(file); // 缓冲器
        let mut content = String::new();
        // 将文件内容读入字符串并返回
        reader.read_to_string(&mut content)?;
        Ok(content)
    }

    /// 按事件处理XML文件
    ///
    /// ### 参数
    /// - `file_path:<&str>`: 文件路径
    /// - `on_start:<FnMut(&str)>`: 处理Start事件的函数
    /// - `on_text:<FnMut(&str)>`: 处理Text事件的函数
    /// - `on_end:<FnMut(&str)>`: 处理End事件的函数
    /// 
    /// 接受三个闭包类型的参数，从而获取对应XML的相关文件内容
    /// ### 返回值
    /// - `Result<()>`:无返回值
    /// 
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// // 读取xml文件标签
    /// let mut start_tags:Vec<String> = Vec::new();
    /// let mut end_tags:Vec<String> = Vec::new();
    /// let mut text:Vec<String> = Vec::new();
    /// // 处理标签的闭包
    /// let on_start = |tag_name:&str|{
    ///     start_tags.push(tag_name.to_string());
    /// };
    /// let on_end = |tag_name:&str| {
    ///     end_tags.push(tag_name.to_string());
    /// };
    /// let on_text = |text_content:&str| {
    ///     text.push(text_content.to_string());
    /// };
    /// let res = FileReader::read_xml_by_listener("src/test_examples/test_xml.xml", on_start, on_text, on_end);
    /// match res {
    ///     Ok(()) => {
    ///         println!("Start tags:");
    ///         for start in start_tags {
    ///             println!("  {}", start);
    ///         }
    ///
    ///         println!("Text nodes:");
    ///         for text in text {
    ///             println!("  {}", text);
    ///         }
    ///
    ///         println!("end tags:");
    ///         for end in end_tags {
    ///             println!("  {}", end);
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Error processing XML: {}", e),
    /// }
    pub fn read_xml_by_listener<S, T, E>(
        file_path: &str,
        mut on_start: S,
        mut on_text: T,
        mut on_end: E,
    ) -> Result<()> // 表示可能成功或失败
    where
        S: FnMut(&str), // 可以多次可变调用的闭包
        T: FnMut(&str),
        E: FnMut(&str),
    {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".xml"), "文件类型错误,请检查文件类型是否为xml文件");

        let file = File::open(file_path).expect(FILE_OPEN_FAILED);
        let reader = BufReader::new(file);
        let mut xml_reader = quick_xml::Reader::from_reader(reader); // 创建一个 XML 读取器，用于从缓冲读取器中读取 XML 数据
        xml_reader.trim_text(true); // 去除文本节点中的空白字符
        let mut buf = Vec::new();
        loop { // 无限循环，直至有break退出
            match xml_reader.read_event_into(&mut buf) { // 读取下一个 XML 事件，并将其存储到 buf 中
                Ok(Event::Start(ref e)) => { // 如果事件是一个开始标签，提取标签名称并调用 on_start 闭包
                    let element_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    on_start(&element_name);
                }
                Ok(Event::Text(e)) => { // 如果事件是一个文本节点，解码文本并调用 on_text 闭包
                    let text = e.unescape().unwrap().to_string();
                    on_text(&text); // 可根据需要获取文本节点内容
                }
                Ok(Event::End(ref e)) => { // 如果事件是一个结束标签，提取标签名称并调用 on_end 闭包。
                    let element_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    on_end(&element_name);
                }
                Ok(Event::Eof) => break, // 文件结束
                Err(e) => {
                    eprintln!("Error at position {}: {:?}", xml_reader.buffer_position(), e);
                    break;
                }
                _ => {}
            }

            buf.clear(); // 清除缓冲区
        }
        Ok(()) // 如果一切正常，返回一个成功结果。
    }

    /// 读取Markdown文件内容
    ///
    /// ### 参数
    /// - `file_path:<&str>`: Markdown文件路径
    ///
    /// ### 返回值
    /// - `Result<String>`:返回Markdown文件内容的字符串
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_markdown("src/test_examples/test_md.md");
    /// match res {
    ///   Ok(content) => println!("md文件内容: \n {}", content),
    ///   Err(e) => println!("读取md文件失败: {}", e), 
    /// }
    pub fn read_markdown(file_path: &str) -> Result<String> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".md"), "文件类型错误,请检查文件类型是否为md文件");

        let file = File::open(file_path).expect(FILE_OPEN_FAILED);
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(content)
    }

    /// 按行读取md文件
    ///
    /// ### 参数
    /// - `file_path:<&str>`: Markdown文件路径
    ///
    /// ### 返回值
    /// - `Result<Vec<String>>`:返回Markdown文件内容的字符串数组
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_md_by_line("src/test_examples/test_md.md");
    /// match res {
    ///   Ok(content) => println!("md文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取md文件失败: {}", e), 
    /// }
    pub fn read_md_by_line(file_path: &str) -> Result<Vec<String>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".md"), "文件类型错误,请检查文件类型是否为md文件");

        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut lines = Vec::new();

        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break, // 0 表示读到的字节数，0，表示读到文件末尾
                Ok(_) => lines.push(line.trim_end_matches('\n').to_string()), // 正常读取
                Err(e) => return Err(e), // 读取错误
            }
        }

        Ok(lines)
    }

    /// 按字节读取md文件
    ///
    /// ### 参数
    /// - `file_path:<&str>`: Markdown文件路径
    ///
    /// ### 返回值
    /// - `Result<Vec<u8>>`:返回Markdown文件内容的u8字节数组
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_md_by_byte("src/test_examples/test_md.md");
    /// match res {
    ///   Ok(content) => println!("md文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取md文件失败: {}", e), 
    /// }
    pub fn read_md_by_byte(file_path: &str) -> Result<Vec<u8>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".md"), "文件类型错误,请检查文件类型是否为md文件");

        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = [0u8;1]; // 默认值0，类型u8，数组大小1
        let mut bytes = Vec::new();
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if n == 1 {
                        bytes.push(buffer[0]);
                    } else {
                        bytes.extend_from_slice(&mut buffer[..n]);  // 这里是根据函数本身返回值需要
                    }

                },
                Err(e) => return Err(e),
            }
        }

        Ok(bytes)
    }

    /// 按块读取md文件
    /// 
    /// ### 参数
    /// - `file_path:<&str>`: Markdown文件路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    ///
    /// ### 返回值
    /// - `Result<Vec<String>>`:返回Markdown文件内容的字符串数组
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_md_by_block("src/test_examples/test_md.md",4);
    /// match res {
    ///   Ok(content) => println!("md文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取md文件失败: {}", e), 
    /// }
    pub fn read_md_by_block(file_path: &str, block_size: usize) -> Result<Vec<String>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".md"), "文件类型错误,请检查文件类型是否为md文件");

        let file = File::open(file_path).expect(FILE_OPEN_FAILED);
        let mut reader = BufReader::new(file);
        let mut blocks = Vec::new();
        let mut buffer = vec![0; block_size]; //创建一个包含block_size个0元素的向量。

        loop {
            let bytes_read = reader.read(&mut buffer)?; // 一次性读取buffer大小字节，不够，读取剩下的
            if bytes_read == 0 {
                break; // 读取到文件末尾
            }
            // 将读取的字节转换为字符串并添加到块中
            blocks.push(String::from_utf8_lossy(&buffer[..bytes_read]).to_string());
        }
        Ok(blocks)
    }

    /// 读取Markdown文件并转换为HTML
    ///
    /// ### 参数
    /// - `file_path:<&str>`: Markdown文件路径
    ///
    /// ### 返回值
    /// - `Result<String>`:返回转换后的HTML内容
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_markdown_and_convert_to_html("src/test_examples/test_md.md");
    /// match res {
    ///   Ok(content) => println!("md文件内容: \n {}", content),
    ///   Err(e) => println!("读取md文件失败: {}", e), 
    /// }
    pub fn read_markdown_and_convert_to_html(file_path: &str) -> Result<String> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".md"), "文件类型错误,请检查文件类型是否为md文件");

        // 读取Markdown文件内容
        let content = Self::read_markdown(file_path).unwrap();

        // 解析Markdown内容
        let parser = Parser::new(&content);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // 返回HTML内容
        Ok(html_output)
    }
    

    /// 读取PDF文件内容
    ///
    /// ### 参数
    /// - `file_path:<&str>`: pdf文件路径
    ///
    /// ### 返回值
    /// - `Result<String>`: 返回pdf所有页内容的字符串
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_pdf_as_string("src/test_examples/test_pdf.pdf");
    /// match res {
    ///   Ok(content) => println!("pdf文件内容: \n {}", content),
    ///   Err(e) => println!("读取pdf文件失败: {}", e), 
    /// }
    pub fn read_pdf_as_string(file_path: &str) -> Result<String> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".pdf"), "文件类型错误,请检查文件类型是否为pdf文件");

        let p = pdf_extract::extract_text(file_path);
        match p {
            Ok(s) => {
                Ok(s.trim_start_matches('\n').trim_end_matches('\n').to_string())
            },
            Err(err) => {
                Err(Error::new(std::io::ErrorKind::Other, err.to_string()))
            }
        }
    }


    /// 读取Excel文件中的所有内容，并以字符串形式返回
    ///
    /// ### 参数
    /// - `file_path:<&str>`: Excel文件路径
    ///
    /// ### 返回值
    /// - `Result<String>`: 成功时返回包含Excel内容的字符串，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_excel_as_string("src/test_examples/test_excel.xlsx");
    /// match res {
    ///   Ok(content) => println!("excel文件内容: \n {}", content),
    ///   Err(e) => println!("读取excel文件失败: {}", e), 
    /// }
    pub fn read_excel_as_string(file_path: &str) -> Result<String> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".xls") || file_path.ends_with(".xlsx"), "文件类型错误,请检查文件类型是否为xls或xlsx文件");
        // 打开Excel文件
        let mut workbook = open_workbook_auto(Path::new(file_path)).unwrap();
        let mut content = String::new();

        // 遍历所有工作表
        for sheet_name in workbook.sheet_names().to_owned() {
            content.push_str(&format!("Sheet: {}\n", sheet_name));

            match workbook.worksheet_range(&sheet_name) {
                Ok(range) => {
                    // 遍历工作表中的每一行
                    for row in range.rows() {
                        let mut row_content = String::new();
                        for cell in row {
                            let cell_content = match cell {
                                Data::String(s) => s.clone(),
                                Data::Float(f) => f.to_string(),
                                Data::Int(i) => i.to_string(),
                                Data::Bool(b) => b.to_string(),
                                // Handle other cell types if necessary
                                _ => "".to_string(),
                            };
                            row_content.push_str(&cell_content);
                            row_content.push('\t'); // Add a tab separator between cells
                        }
                        content.push_str(&row_content);
                        content.push('\n'); // Add a newline after each row
                    }
                },
                Err(e) => {
                    content.push_str(&format!("Cannot read sheet {}\n", sheet_name));
                    eprintln!("Failed to read worksheet {}: {}", sheet_name, e);
                }
            }
        }

        Ok(content)
    }


    /// 按照Sheet读取Excel文件中的所有内容，并以字符串形式返回
    ///
    /// ### 参数
    /// - `file_path:<&str>`: Excel文件路径
    ///
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回包含Excel内容的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_excel_as_string_by_sheet("src/test_examples/test_excel.xlsx");
    /// match res {
    ///   Ok(content) => println!("excel文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取excel文件失败: {}", e), 
    /// }
    pub fn read_excel_as_string_by_sheet(file_path: &str) -> Result<Vec<String>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".xls") || file_path.ends_with(".xlsx"), "文件类型错误,请检查文件类型是否为xls或xlsx文件");

        // 打开Excel文件
        let mut workbook = open_workbook_auto(Path::new(file_path)).unwrap();
        let mut contents: Vec<String> = Vec::new();

        // 遍历所有工作表
        for sheet_name in workbook.sheet_names().to_owned() {
            let mut content = String::new();
            content.push_str(&format!("Sheet: {}\n", sheet_name));
            
            match workbook.worksheet_range(&sheet_name) {
                Ok(range) => {
                    for row in range.rows() {
                        let mut row_content = String::new();
                        for cell in row {
                            let cell_content = match cell {
                                Data::String(s) => s.clone(),
                                Data::Float(f) => f.to_string(),
                                Data::Int(i) => i.to_string(),
                                Data::Bool(b) => b.to_string(),
                                Data::Empty => "".to_string(),
                                _ => "?".to_string(), // 未知或复杂类型
                            };
                            row_content.push_str(&cell_content);
                            row_content.push('\t'); // 每个单元格之间用制表符分隔
                        }
                        content.push_str(&row_content);
                        content.push('\n'); // 每行结束后换行
                    }
                },
                Err(e) => {
                    content.push_str(&format!("Cannot read sheet {}\n", sheet_name));
                    eprint!("Failed to read worksheet {}: {}", sheet_name, e);
                }
            }
            contents.push(content);
        }
        Ok(contents)
    }

    /// 读取Excel文件中的所有列的内容，并按列格式化为字符串
    ///
    /// ### 参数
    /// - `file_path:<&str>`: Excel文件路径
    ///
    /// ### 返回值
    /// - `Result<String>`: 成功时返回包含Excel内容的字符串，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_excel_by_column_as_string("src/test_examples/test_excel.xlsx");
    /// match res {
    ///   Ok(content) => println!("excel文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取excel文件失败: {}", e), 
    /// }
    pub fn read_excel_by_column_as_string(file_path: &str) -> Result<String> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".xls") || file_path.ends_with(".xlsx"), "文件类型错误,请检查文件类型是否为xls或xlsx文件");

        // 打开Excel文件
        let mut workbook = open_workbook_auto(Path::new(file_path)).unwrap();
        let mut content = String::new();

        // 遍历所有工作表
        for sheet_name in workbook.sheet_names().to_owned() {
            content.push_str(&format!("Sheet: {}\n", sheet_name));
            match workbook.worksheet_range(&sheet_name) {
                Ok(range) => {
                    let rows = range.rows().collect::<Vec<_>>(); // 先收集所有行

                    // 如果没有行，则继续下一个工作表
                    if rows.is_empty() {
                        continue;
                    }
    
                    let column_count = rows[0].len(); // 假设每行的列数相同
    
                    // 遍历列
                    for col_index in 0..column_count {
                        let mut col_content = String::new();
                        for row in &rows {
                            let cell = row.get(col_index).unwrap_or(&Data::Empty); // 处理缺少的单元格
                            let cell_content = match cell {
                                Data::String(s) => s.clone(),
                                Data::Float(f) => f.to_string(),
                                Data::Int(i) => i.to_string(),
                                Data::Bool(b) => b.to_string(),
                                Data::Empty => "".to_string(),
                                _ => "?".to_string(), // 未知或复杂类型
                            };
                            col_content.push_str(&cell_content);
                            col_content.push('\t'); // 每个单元格之间用制表符分隔
                        }
                        content.push_str(&col_content);
                        content.push('\n'); // 每列结束后换行
                    }
                },
                Err(e) => {
                    content.push_str(&format!("Cannot read sheet {}\n", sheet_name));
                    eprintln!("Failed to read worksheet {}: {}", sheet_name, e);
                }
            }
        }

        Ok(content)
    }

    /// 按行读取excel文件内容，每一行一个向量，每个元素用制表符分隔
    /// 
    /// ### 参数
    /// - `file_path:<&str>`: Excel文件路径
    ///
    /// ### 返回值
    /// - `Result<Vec<Vec<String>>>`: 成功时返回包含Excel内容的字符串向量，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_excel_by_row("src/test_examples/test_excel.xlsx");
    /// match res {
    ///   Ok(content) => println!("excel文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取excel文件失败: {}", e), 
    /// }
    pub fn read_excel_by_row(file_path: &str) -> Result<Vec<Vec<String>>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".xls") || file_path.ends_with(".xlsx"), "文件类型错误,请检查文件类型是否为xls或xlsx文件");

        // 打开Excel文件
        let mut workbook = open_workbook_auto(Path::new(file_path)).unwrap();
        let mut sheets = Vec::new();

        // 遍历所有工作表
        for sheet_name in workbook.sheet_names().to_owned() {
            // content.push_str(&format!("Sheet: {}\n", sheet_name));
            let mut sheet_content = Vec::new(); // 存储每个工作表的字符串
            match workbook.worksheet_range(&sheet_name) {
                Ok(range) => {
                    let rows = range.rows().collect::<Vec<_>>(); // 先收集所有行

                    // 如果没有行，则继续下一个工作表
                    if rows.is_empty() {
                        continue;
                    }
                    for row in rows {
                        let mut row_content = String::new();
                        for cell in row {
                            // let cell = row.get(col_index).unwrap_or(&Data::Empty); // 处理缺少的单元格
                            let cell_content = match cell {
                                Data::String(s) => s.clone(),
                                Data::Float(f) => f.to_string(),
                                Data::Int(i) => i.to_string(),
                                Data::Bool(b) => b.to_string(),
                                Data::Empty => "".to_string(),
                                _ => "?".to_string(), // 未知或复杂类型
                            };
                            row_content.push_str(&cell_content);
                            row_content.push('\t'); // 每个单元格之间用制表符分隔
                        }
                        sheet_content.push(row_content);
                    }
                },
                Err(e) => {
                    sheet_content.push(format!("Cannot read sheet {}\n", sheet_name));
                    eprintln!("Failed to read worksheet {}: {}", sheet_name, e);
                }
            }
            sheets.push(sheet_content);
        }

        Ok(sheets)
    }


    /// 按列读取excel文件内容，每一列一个向量，每个元素用制表符分隔
    /// 
    /// ### 参数
    /// - `file_path:<&str>`: Excel文件路径
    ///
    /// ### 返回值
    /// - `Result<Vec<Vec<String>>>`: 成功时返回包含Excel内容的字符串向量，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_excel_by_column("src/test_examples/test_excel.xlsx");
    /// match res {
    ///   Ok(content) => println!("excel文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取excel文件失败: {}", e), 
    /// }
    pub fn read_excel_by_column(file_path: &str) -> Result<Vec<Vec<String>>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".xls") || file_path.ends_with(".xlsx"), "文件类型错误,请检查文件类型是否为xls或xlsx文件");

        // 打开Excel文件
        let mut workbook = open_workbook_auto(Path::new(file_path)).unwrap();
        let mut sheets = Vec::new();

        // 遍历所有工作表
        for sheet_name in workbook.sheet_names().to_owned() {
            // content.push_str(&format!("Sheet: {}\n", sheet_name));
            let mut sheet_content = Vec::new(); // 存储每个工作表的字符串
            match workbook.worksheet_range(&sheet_name) {
                Ok(range) => {
                    let rows = range.rows().collect::<Vec<_>>(); // 先收集所有行

                    // 如果没有行，则继续下一个工作表
                    if rows.is_empty() {
                        continue;
                    }

                    let column_count = rows[0].len(); // 假设每行的列数相同
    
                    // 遍历列
                    for col_index in 0..column_count {
                        let mut col_content = String::new();
                        for row in &rows {
                            let cell = row.get(col_index).unwrap_or(&Data::Empty); // 处理缺少的单元格
                            let cell_content = match cell {
                                Data::String(s) => s.clone(),
                                Data::Float(f) => f.to_string(),
                                Data::Int(i) => i.to_string(),
                                Data::Bool(b) => b.to_string(),
                                Data::Empty => "".to_string(),
                                _ => "?".to_string(), // 未知或复杂类型
                            };
                            col_content.push_str(&cell_content);
                            col_content.push('\t'); // 每个单元格之间用制表符分隔
                        }
                        sheet_content.push(col_content);
                    }
                },
                Err(e) => {
                    sheet_content.push(format!("Cannot read sheet {}\n", sheet_name));
                    eprintln!("Failed to read worksheet {}: {}", sheet_name, e);
                }
            }
            sheets.push(sheet_content);
        }

        Ok(sheets)
    }

    /// 读取Excel文件中的内容并按块格式化为字符串，返回就是一个字符串，用来展示的
    ///
    /// ### 参数
    /// - `file_path:<&str>`: Excel文件路径
    /// - `block_size:(usize,usize)`: 读取块的大小 (rows, cols)
    ///
    /// ### 返回值
    /// - `Result<String>`: 成功时返回包含Excel内容的字符串，失败时返回错误信息,只是返回一个字符串,块都是按行展开
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_excel_by_block_as_string("src/test_examples/test_excel.xlsx",(4,4));
    /// match res {
    ///   Ok(content) => println!("excel文件内容: \n {}", content),
    ///   Err(e) => println!("读取excel文件失败: {}", e), 
    /// }
    pub fn read_excel_by_block_as_string(file_path: &str, block_size: (usize, usize)) -> Result<String> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".xls") || file_path.ends_with(".xlsx"), "文件类型错误,请检查文件类型是否为xls或xlsx文件");

        // 打开Excel文件
        let mut workbook = open_workbook_auto(Path::new(file_path)).unwrap();
        let mut content = String::new();
        let (block_rows, block_cols) = block_size;

        // 遍历所有工作表
        for sheet_name in workbook.sheet_names().to_owned() {
            content.push_str(&format!("Sheet: {}\n", sheet_name));

        match workbook.worksheet_range(&sheet_name) {
            Ok(range) => {
                let rows = range.rows().collect::<Vec<_>>(); // 先收集所有行
                let total_rows = rows.len();
                let total_cols = rows.get(0).map_or(0, |row| row.len()); // 使用get可能会返回none，map_or提供默认值0，和闭包，当get返回none值是，使用默认值，否则使用闭包计算

                // 按块遍历工作表内容
                for start_row in (0..total_rows).step_by(block_rows) { // 每次处理block_rows行数据
                    for start_col in (0..total_cols).step_by(block_cols) {
                        let mut block_content = String::new();

                        for r in start_row..(start_row + block_rows).min(total_rows) { //确保结束索引不会超过总行数 total_rows
                            for c in start_col..(start_col + block_cols).min(total_cols) {
                                let cell = rows[r].get(c).unwrap_or(&Data::Empty);
                                let cell_content = match cell {
                                    Data::String(s) => s.clone(),
                                    Data::Float(f) => f.to_string(),
                                    Data::Int(i) => i.to_string(),
                                    Data::Bool(b) => b.to_string(),
                                    Data::Empty => "".to_string(),
                                    _ => "?".to_string(),
                                };
                                block_content.push_str(&cell_content);
                                block_content.push('\t');
                            }
                            block_content.push('\n');
                        }
                        content.push_str(&block_content);
                        content.push('\n'); // 每块结束后添加换行符
                    }
                }
            },
            Err(e) => {
                content.push_str(&format!("Cannot read sheet {}\n", sheet_name));
                eprintln!("Failed to read worksheet {}: {}", sheet_name, e);
            }
        }

    }
        Ok(content)
    }


    /// 读取Excel文件中的内容并按块格式化为字符串
    ///
    /// ### 参数
    /// - `file_path:<&str>`: Excel文件路径
    /// - `block_size:(usize,usize)`: 读取块的大小 (rows, cols)
    ///
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回包含Excel内容的字符串，失败时返回错误信息,每一个块是一个字符串
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_excel_by_block("src/test_examples/test_excel.xlsx",(4,4));
    /// match res {
    ///   Ok(content) => println!("excel文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取excel文件失败: {}", e), 
    /// }
    pub fn read_excel_by_block(file_path: &str, block_size: (usize, usize)) -> Result<Vec<String>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".xls") || file_path.ends_with(".xlsx"), "文件类型错误,请检查文件类型是否为xls或xlsx文件");

        // 打开Excel文件
        let mut workbook = open_workbook_auto(Path::new(file_path)).unwrap();
        let mut content =Vec::new();
        let (block_rows, block_cols) = block_size;

        // 遍历所有工作表
        for sheet_name in workbook.sheet_names().to_owned() {
            // content.push_str(&format!("Sheet: {}\n", sheet_name));

        match workbook.worksheet_range(&sheet_name) {
            Ok(range) => {
                let rows = range.rows().collect::<Vec<_>>(); // 先收集所有行
                let total_rows = rows.len();
                let total_cols = rows.get(0).map_or(0, |row| row.len()); // 使用get可能会返回none，map_or提供默认值0，和闭包，当get返回none值是，使用默认值，否则使用闭包计算

                // 按块遍历工作表内容
                for start_row in (0..total_rows).step_by(block_rows) { // 每次处理block_rows行数据
                    for start_col in (0..total_cols).step_by(block_cols) {
                        let mut block_content = String::new();

                        for r in start_row..(start_row + block_rows).min(total_rows) { //确保结束索引不会超过总行数 total_rows
                            for c in start_col..(start_col + block_cols).min(total_cols) {
                                let cell = rows[r].get(c).unwrap_or(&Data::Empty);
                                let cell_content = match cell {
                                    Data::String(s) => s.clone(),
                                    Data::Float(f) => f.to_string(),
                                    Data::Int(i) => i.to_string(),
                                    Data::Bool(b) => b.to_string(),
                                    Data::Empty => "".to_string(),
                                    _ => "?".to_string(),
                                };
                                block_content.push_str(&cell_content);
                                block_content.push('\t');
                            }
                            block_content.push('\n');
                        }
                        content.push(block_content);
                        // content.push('\n'); // 每块结束后添加换行符
                    }
                }
            },
            Err(e) => {
                content.push(format!("Cannot read sheet {}\n", sheet_name));
                eprintln!("Failed to read worksheet {}: {}", sheet_name, e);
            }
        }

    }
        Ok(content)
    }


    /// 从PPTX文件中提取文本内容
    ///
    /// ### 参数
    /// - `file_path:<&str>`: PPTX文件路径
    ///
    /// ### 返回值
    /// - `Result<String>`: 成功时返回包含PPTX内容的字符串，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_pptx_text("src/test_examples/test_ppt.pptx");
    /// match res {
    ///   Ok(content) => println!("pptx文件内容: \n {}", content),
    ///   Err(e) => println!("读取pptx文件失败: {}", e), 
    /// }
    pub fn read_pptx_text(file_path: &str) -> Result<String> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".ppt") || file_path.ends_with(".pptx"), "文件类型错误,请检查文件类型是否为ppt或pptx文件");
        
        // 打开PPTX文件
        let file = File::open(Path::new(file_path))?;
        let mut archive = ZipArchive::new(file)?;
        let mut slide_texts = String::new();

        // 遍历PPTX文件中的所有文件
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_name = file.name().to_string();

            // 查找幻灯片文件（通常在ppt/slides/目录下，文件名以"slide"开头）
            if file_name.starts_with("ppt/slides/slide") && file_name.ends_with(".xml") {
                let mut xml_content = String::new();
                file.read_to_string(&mut xml_content)?;

                // 解析XML内容，提取文本
                let mut slide_text = String::new();
                let parser = EventReader::from_str(&xml_content);
                for event in parser {
                    match event {
                        Ok(XmlEvent::Characters(text)) => {
                            slide_text.push_str(&text);
                            slide_text.push('\n');
                        }
                        _ => {}
                    }
                }
                slide_texts.push_str(&slide_text.to_string());
            }
        }

        Ok(slide_texts)
    }


    /// 按页读取pptx文件内容，每一页一个向量
    ///
    /// ### 参数
    /// - `file_path:<&str>`: PPTX文件路径
    ///
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回包含PPTX内容的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_pptx_text_by_slide("src/test_examples/test_ppt.pptx");
    /// match res {
    ///   Ok(content) => println!("pptx文件内容: \n {:?}", content),
    ///   Err(e) => println!("读取pptx文件失败: {}", e), 
    /// }
    pub fn read_pptx_text_by_slide(file_path: &str) -> Result<Vec<String>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".ppt") || file_path.ends_with(".pptx"), "文件类型错误,请检查文件类型是否为ppt或pptx文件");

        // 打开PPTX文件
        let file = File::open(Path::new(file_path))?;
        let mut archive = ZipArchive::new(file)?;
        let mut slide_texts = Vec::new();

        // 遍历PPTX文件中的所有文件
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_name = file.name().to_string();

            // 查找幻灯片文件（通常在ppt/slides/目录下，文件名以"slide"开头）
            if file_name.starts_with("ppt/slides/slide") && file_name.ends_with(".xml") {
                let mut xml_content = String::new();
                file.read_to_string(&mut xml_content)?;

                // 解析XML内容，提取文本
                let mut slide_text = String::new();
                let parser = EventReader::from_str(&xml_content);
                for event in parser {
                    match event {
                        Ok(XmlEvent::Characters(text)) => {
                            slide_text.push_str(&text);
                            // slide_text.push('\n');
                        }
                        _ => {}
                    }
                }
                slide_texts.push(slide_text);
            }
        }

        Ok(slide_texts)
    }

    /// 读取ppt备注
    ///
    /// ### 参数
    /// - `file_path:<&str>`: PPTX文件路径
    ///
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回包含PPTX备注的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_pptx_notes("src/test_examples/test_ppt.pptx");
    /// match res {
    ///   Ok(content) => println!("pptx文件备注: \n {:?}", content),
    ///   Err(e) => println!("读取pptx文件备注失败: {}", e), 
    /// }
    pub fn read_pptx_notes(file_path: &str) -> Result<Vec<String>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".ppt") || file_path.ends_with(".pptx"), "文件类型错误,请检查文件类型是否为ppt或pptx文件");
    
        let file = File::open(Path::new(file_path))?;
        let mut archive = ZipArchive::new(file)?;
    
        let mut notes = Vec::new();
    
        // 遍历PPTX文件中的所有文件
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            if file.name().starts_with("ppt/notesSlides/notesSlide") && file.name().ends_with(".xml") {
                let mut xml_content = String::new();
                file.read_to_string(&mut xml_content)?;
    
                // Parse XML content
                let parser = EventReader::from_str(&xml_content);
                let mut note_content = String::new();
                let mut in_text_run = false;
    
                for event in parser {
                    match event {
                        Ok(XmlEvent::StartElement { name, .. }) => {
                            if name.local_name == "t" {
                                // Start text node
                                in_text_run = true;
                            }
                        },
                        Ok(XmlEvent::Characters(text)) if in_text_run => {
                            // 去除每个文本节点前面的空格
                            note_content.push_str(&text.trim_start());
                        },
                        Ok(XmlEvent::EndElement { name }) => {
                            if name.local_name == "t" {
                                in_text_run = false;
                            }
                        },
                        Err(_) => {
                            // Handle error
                            break;
                        },
                        _ => {}
                    }
                }
    
                if !note_content.is_empty() {
                    // 去除每条备注开头的空格
                    let note_content = note_content.trim_start().to_string();
                    // 去除每条备注之后的序号
                    let note_content = note_content.trim_end_matches(|c: char| c.is_digit(10) || c.is_whitespace()).to_string();
                    notes.push(note_content);
                }
            }
        }
    
        Ok(notes)
    }


    /// 列出zip文件中所有文件目录
    ///
    /// ### 参数
    /// - `file_path:<&str>`: zip文件路径
    /// - `exclude_directories:<bool>`: 是否包含压缩包里的文件目录
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回包含zip所有文件目录的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::list_zip_filenames("src/test_examples/test_zip.zip",true);
    /// match res {
    ///   Ok(content) => println!("zip文件中所以文件目录: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn list_zip_filenames(file_path: &str, mut exclude_directories: bool) -> Result<Vec<String>> {
        assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(file_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        exclude_directories = !exclude_directories;
        let file = File::open(file_path)?;
        let mut archive = ZipArchive::new(file)?;
    
        let file_list = (0..archive.len())
            .filter_map(|i| {
                archive.by_index(i).ok()
                    .filter(|entry| !exclude_directories || !entry.is_dir())
                    .map(|entry| entry.name().to_string())
            })
            .collect::<Vec<_>>();
    
        Ok(file_list)
    }
    
    // 拼接路径函数
    // fn join_path(path1: &str, path2: &str) -> String {
    //     let mut full_path = PathBuf::from(path1);
    //     full_path.push(path2);
    //     full_path.to_string_lossy().into_owned().replace("\\", "/")
    // }

    /// 读取zip内文本文件内容
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: zip文件路径
    /// - `file_name:<&str>`: 要打开的文本文件的路径
    /// ### 返回值
    /// - `Result<String>`: 成功时返回包含指定的zip文本文件内容的字符串，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_txt("src/test_examples/test_zip.zip","test_rar/test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定文本文件的内容: \n {}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_txt(zip_path: &str, file_name: &str) -> Result<String> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
    
        Ok(contents)
    }

    /// 按行读取文本文件内容
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: zip文件路径
    /// - `file_name:<&str>`: 要打开的文本文件的路径
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回包含指定的zip文本文件内容的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_txt_by_line("src/test_examples/test_zip.zip","test_rar/test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定文本文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_txt_by_line(zip_path: &str, file_name: &str) -> Result<Vec<String>> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;

        let mut reader = BufReader::new(file);
        let mut lines = Vec::new();

        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break, // 0 表示读到的字节数，0，表示读到文件末尾
                Ok(_) => lines.push(line.trim_end_matches('\n').to_string()), // 正常读取
                Err(e) => return Err(e), // 读取错误
            }
        }

        Ok(lines)
    }

    
    /// 按字节读取文本文件内容
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: zip文件路径
    /// - `file_name:<&str>`: 要打开的文本文件的路径
    /// ### 返回值
    /// - `Result<Vec<u8>>`: 成功时返回包含指定的zip文本文件内容的字节数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_txt_by_byte("src/test_examples/test_zip.zip","test_rar/test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定文本文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_txt_by_byte(zip_path: &str, file_name: &str) -> Result<Vec<u8>> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;

        let mut reader = BufReader::new(file);
        let mut buffer = [0u8;1]; // 默认值0，类型u8，数组大小1
        let mut bytes = Vec::new();
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if n == 1 {
                        bytes.push(buffer[0]);
                    } else {
                        bytes.extend_from_slice(&mut buffer[..n]);  // 这里是根据函数本身返回值需要
                    }

                },
                Err(e) => return Err(e),
            }
        }

        Ok(bytes)
    }
    
    /// 按块读取TXT文件
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的文本文件的路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    ///
    /// ### 返回值
    /// - `Result<Vec<String>>`返回一个包含每个块的向量，每个块都是一个字符串
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_txt_by_byte("src/test_examples/test_zip.zip","test_rar/test_txt.txt",1024);
    /// match res {
    ///   Ok(content) => println!("zip文件中指定文本文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_txt_by_block(zip_path: &str, file_name: &str, block_size: usize) -> Result<Vec<String>> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;

        let mut reader = BufReader::new(file);
        let mut blocks = Vec::new();
        let mut buffer = vec![0; block_size]; //创建一个包含block_size个0元素的向量。

        loop {
            let bytes_read = reader.read(&mut buffer)?; // 一次性读取buffer大小字节，不够，读取剩下的
            if bytes_read == 0 {
                break; // 读取到文件末尾
            }
            // 将读取的字节转换为字符串并添加到块中
            blocks.push(String::from_utf8_lossy(&buffer[..bytes_read]).to_string());
        }
        Ok(blocks)
    }

    /// 按行读取zip内csv文件内容
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的csv文件的路径
    ///
    /// ### 返回值
    /// - `Result<Vec<Vec<String>>>`：返回一个二维字符串数组
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_csv("src/test_examples/test_zip.zip","test_rar/test_csv.csv");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定csv文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_csv(zip_path: &str, file_name: &str) -> Result<Vec<Vec<String>>> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;
        //ReaderBuilder 是一个用于构建 Reader 实例的构造器，允许用户配置 Reader 的行为，创建一个 CSV 读取器
        let mut rdr = ReaderBuilder::new().from_reader(file);
        let mut records = Vec::new();
        // 每一行是一条记录，vec<String>
        for result in rdr.records() {
            let record = result?;
            let record_vec: Vec<String> = record.iter().map(|s| s.to_string()).collect();
            records.push(record_vec);
        }
    
        Ok(records)
    }

    /// 按列读取zip中csv文件内容，默认是没有读取列名的 
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的csv文件的路径
    ///
    /// ### 返回值
    /// - `Result<Vec<Vec<String>>>`：返回一个二维字符串数组
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_csv_by_column("src/test_examples/test_zip.zip","test_rar/test_csv.csv");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定csv文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_csv_by_column(zip_path: &str, file_name: &str) -> Result<Vec<Vec<String>>> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;

        let mut rdr = ReaderBuilder::new().from_reader(file);
        let mut records:Vec<Vec<String>> = Vec::new();

        // 读取所有行
        for result in rdr.records() {
            let record = result?;
            records.push(record.iter().map(|s| s.to_string()).collect());
        }
        // println!("temp test records is {:?}", records);
        // 按列转换
        let mut columns = vec![Vec::new(); records[0].len()];
        for record in records {
            for (i, field) in record.into_iter().enumerate() {
                columns[i].push(field);
            }
        }

        Ok(columns)
    }

    /// 按块读取zip中csv文件内容
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的csv文件的路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    ///
    /// ### 返回值
    /// - `Result<Vec<Vec<Vec<String>>>>`：返回一个向量，每个向量元素即为csv文件块的内容，用二维向量存储
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_csv_by_block("src/test_examples/test_zip.zip","test_rar/test_csv.csv",4);
    /// match res {
    ///   Ok(content) => println!("zip文件中指定csv文件的内容:\n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_csv_by_block(zip_path: &str, file_name: &str, block_size: usize) -> Result<Vec<Vec<Vec<String>>>> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;

        let mut rdr = ReaderBuilder::new().from_reader(file);
        let mut blocks = Vec::new();
        let mut current_block = Vec::new();

        for result in rdr.records() {
            let record = result.expect("Failed to read record");  // 加了一个expect
            // current_block.push(record.iter().map(|s| s.to_string()).collect());
            let string_vec: Vec<String> = record.iter().map(String::from).collect(); // 一行数据
            current_block.push(string_vec);

            if current_block.len() == block_size { // 满足指定行数，就添加块到结果中
                blocks.push(current_block.clone());
                current_block.clear();
            }
        }
        // 如果最后一个块不满，仍然保存
        if !current_block.is_empty() {
            blocks.push(current_block);
        }

        Ok(blocks)
    }

    /// 动态读取zip中json文件
    /// 
    /// Value 是 serde_json 库中的类型，用于表示任何 JSON 数据结构
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的json文件的路径
    ///
    /// ### 返回值
    /// - `serde_json::Result<Value>`：返回Value，Value包含整个json文件的内容
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_json("src/test_examples/test_zip.zip","test_rar/test_json.json");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定json文件的内容:\n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_json(zip_path: &str, file_name: &str) -> serde_json::Result<Value> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path).unwrap();
        let mut archive = ZipArchive::new(zip_file).unwrap();
        // 打开指定文件
        let file = archive.by_name(file_name).unwrap();

        let reader = BufReader::new(file);

        // 直接解析为serde_json::Value
        let json_data: Value = serde_json::from_reader(reader)?; // 尝试从 reader 中解析 JSON 数据

        Ok(json_data)
    }


    /// 按块读取json文件
    /// 
    /// Value 是 serde_json 库中的类型，用于表示任何 JSON 数据结构
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的json文件的路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    ///
    /// ### 返回值
    /// - `Result<Vec<Vec<Value>>>`：返回包含每个块的向量，每个块都是一个对象向量
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_json_by_block("src/test_examples/test_zip.zip","test_rar/test_json.json",4);
    /// match res {
    ///   Ok(content) => println!("zip文件中指定json文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_json_by_block(zip_path: &str, file_name: &str, block_size: usize) -> Result<Vec<Vec<Value>>> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path).unwrap();
        let mut archive = ZipArchive::new(zip_file).unwrap();
        // 打开指定文件
        let file = archive.by_name(file_name).unwrap();

        let mut reader = BufReader::new(file);
        //这里的 <::Value> 指定了我们期望从迭代器中得到的类型是 serde_json::Value。serde_json::Value 是一个枚举类型，
        //它可以代表 JSON 中的任何值（例如对象、数组、字符串等）。rust不认识json，所以反序列化
        // 迭代地解析 JSON 文件，而不是一次性加载整个文件到内存中，这对于处理大文件非常有用。
        // let stream = Deserializer::from_reader(reader).into_iter::<Value>(); // 会一次性读一个数组，不对
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let json_array:Value = from_str(&content)?;
        let array = json_array.as_array().ok_or(Error::new(std::io::ErrorKind::Other, "Not an array"))?;

        let mut blocks = Vec::new();
        let mut current_block = Vec::with_capacity(block_size); //创建一个具有预设元素容量 block_size 的向量，元素类型是serde_json::Value
        for value in array {
            current_block.push(value.clone());
            if current_block.len() == block_size {
                blocks.push(current_block);
                current_block = Vec::with_capacity(block_size);
            }
        }
        // 如果最后一个块不满，仍然保存
        if !current_block.is_empty() {
            blocks.push(current_block);
        }

        Ok(blocks)
    }

    /// 按行读取zip内jsonl文件
    ///
    /// Value 是 serde_json 库中的类型，用于表示任何 JSON 数据结构
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的jsonl文件的路径
    ///
    /// ### 返回值
    /// - `Result<Vec<Value>>`：返回Value数组，Value数组包含整个jsonl文件的内容
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_jsonl("src/test_examples/test_zip.zip","test_rar/test_jsonline.jsonl");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定jsonl文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_jsonl(zip_path: &str, file_name: &str) -> Result<Vec<Value>> { 
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;
        let reader = BufReader::new(file);

        let mut vec_json = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let json_value = from_str(&line)?; // from_str 是 serde_json 库中的一个函数，用于将字符串解析为 serde_json::Value 类型
            vec_json.push(json_value);
        }
        Ok(vec_json)
    }


    /// 直接读取zip文件中XML文件内容
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的xml文件的路径
    ///
    /// ### 返回值
    /// - `Result<String>`：返回一个字符串，字符串中包含指定xml文件全部内容
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_xml_text("src/test_examples/test_zip.zip","test_rar/test_xml.xml");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定xml文件的内容:\n {}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_xml_text(zip_path: &str, file_name: &str) -> Result<String> { // 这里是简化写法，固定了错误类型
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;

        let mut reader = BufReader::new(file); // 缓冲器
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(content)
    }

    /// 按事件处理读取zip文件中xml文件内容
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的xml文件的路径
    /// - `on_start:<FnMut(&str)>`: 处理Start事件的函数
    /// - `on_text:<FnMut(&str)>`: 处理Text事件的函数
    /// - `on_end:<FnMut(&str)>`: 处理End事件的函数
    /// 
    /// 接受三个闭包类型的参数，从而获取对应XML的相关文件内容
    /// ### 返回值
    /// - `Result<()>`：无返回值
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// // 读取xml文件标签
    /// let mut start_tags:Vec<String> = Vec::new();
    /// let mut end_tags:Vec<String> = Vec::new();
    /// let mut text:Vec<String> = Vec::new();
    /// // 处理标签的闭包
    /// let on_start = |tag_name:&str|{
    ///     start_tags.push(tag_name.to_string());
    /// };
    /// let on_end = |tag_name:&str| {
    ///     end_tags.push(tag_name.to_string());
    /// };
    /// let on_text = |text_content:&str| {
    ///     text.push(text_content.to_string());
    /// };
    ///
    /// let res = FileReader::read_zip_xml("src/test_examples/test_zip.zip", "test_rar/test_xml.xml", on_start, on_text, on_end);
    /// match res {
    ///     Ok(()) => {
    ///         println!("Start tags:");
    ///         for start in start_tags {
    ///             println!("  {}", start);
    ///         }
    ///
    ///         println!("Text nodes:");
    ///         for text in text {
    ///             println!("  {}", text);
    ///         }
    ///
    ///         println!("end tags:");
    ///         for end in end_tags {
    ///             println!("  {}", end);
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Error processing XML: {}", e),
    /// }
    pub fn read_zip_xml<S, T, E>(
        zip_path: &str, 
        file_name: &str,
        mut on_start: S,
        mut on_text: T,
        mut on_end: E,
    ) -> Result<()> // 表示可能成功或失败
    where
        S: FnMut(&str), // 可以多次可变调用的闭包
        T: FnMut(&str),
        E: FnMut(&str),
    {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;
        let reader = BufReader::new(file);
        let mut xml_reader = quick_xml::Reader::from_reader(reader); // 创建一个 XML 读取器，用于从缓冲读取器中读取 XML 数据
        xml_reader.trim_text(true); // 去除文本节点中的空白字符
        let mut buf = Vec::new();
        loop { // 无限循环，直至有break退出
            match xml_reader.read_event_into(&mut buf) { // 读取下一个 XML 事件，并将其存储到 buf 中
                Ok(Event::Start(ref e)) => { // 如果事件是一个开始标签，提取标签名称并调用 on_start 闭包
                    let element_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    on_start(&element_name);
                }
                Ok(Event::Text(e)) => { // 如果事件是一个文本节点，解码文本并调用 on_text 闭包
                    let text = e.unescape().unwrap().to_string();
                    on_text(&text); // 可根据需要获取文本节点内容
                }
                Ok(Event::End(ref e)) => { // 如果事件是一个结束标签，提取标签名称并调用 on_end 闭包。
                    let element_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    on_end(&element_name);
                }
                Ok(Event::Eof) => break, // 文件结束
                Err(e) => {
                    eprintln!("Error at position {}: {:?}", xml_reader.buffer_position(), e);
                    break;
                }
                _ => {}
            }

            buf.clear(); // 清除缓冲区
        }
        Ok(()) // 如果一切正常，返回一个成功结果。
    }

    /// 一次性读取zip文件中md文件内容
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的md文件的路径
    ///
    /// ### 返回值
    /// - `Result<String>`：返回一个字符串，字符串中包含指定md文件全部内容
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_markdown("src/test_examples/test_zip.zip","test_rar/test_md.md");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定md文件的内容: \n {}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_markdown(zip_path: &str, file_name: &str) -> Result<String> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;

        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(content)
    }

    /// 按行读取md文件
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的md文件的路径
    ///
    /// ### 返回值
    /// - `Result<Vec<String>>`：返回一个字符串数组，每个字符串中包含指定md文件的内容
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_md_by_line("src/test_examples/test_zip.zip","test_rar/test_md.md");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定md文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_md_by_line(zip_path: &str, file_name: &str) -> Result<Vec<String>> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;

        let mut reader = BufReader::new(file);
        let mut lines = Vec::new();

        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break, // 0 表示读到的字节数，0，表示读到文件末尾
                Ok(_) => lines.push(line.trim_end_matches('\n').to_string()), // 正常读取
                Err(e) => return Err(e), // 读取错误
    
            }
        }

        Ok(lines)
    }

    /// 按字节读取md文件
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的md文件的路径
    ///
    /// ### 返回值
    /// - `Result<Vec<u8>>`：返回一个u8字节数组，包含指定md文件全部内容
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_md_by_byte("src/test_examples/test_zip.zip","test_rar/test_md.md");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定md文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_md_by_byte(zip_path: &str, file_name: &str) -> Result<Vec<u8>> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;

        let mut reader = BufReader::new(file);
        let mut buffer = [0u8;1]; // 默认值0，类型u8，数组大小1
        let mut bytes = Vec::new();
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if n == 1 {
                        bytes.push(buffer[0]);
                    } else {
                        bytes.extend_from_slice(&mut buffer[..n]);  // 这里是根据函数本身返回值需要
                    }

                },
                Err(e) => return Err(e),
            }
        }

        Ok(bytes)
    }

    /// 按块读取md文件
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的md文件的路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    ///
    /// ### 返回值
    /// - `Result<Vec<String>>`：返回指定Markdown文件内容的字符串数组
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_md_by_block("src/test_examples/test_zip.zip","test_rar/test_md.md",4);
    /// match res {
    ///   Ok(content) => println!("zip文件中指定md文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_md_by_block(zip_path: &str, file_name: &str, block_size: usize) -> Result<Vec<String>> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let file = archive.by_name(file_name)?;

        let mut reader = BufReader::new(file);
        let mut blocks = Vec::new();
        let mut buffer = vec![0; block_size]; //创建一个包含block_size个0元素的向量。

        loop {
            let bytes_read = reader.read(&mut buffer)?; // 一次性读取buffer大小字节，不够，读取剩下的
            if bytes_read == 0 {
                break; // 读取到文件末尾
            }
            // 将读取的字节转换为字符串并添加到块中
            blocks.push(String::from_utf8_lossy(&buffer[..bytes_read]).to_string());
        }
        Ok(blocks)
    }



    /// 读取zip文件中excel文件内容
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的excel文件的路径
    ///
    /// ### 返回值
    /// - `Result<Vec<String>>`：返回指定excel文件内容的字符串数组
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_excel("src/test_examples/test_zip.zip","test_rar/test_excel.xlsx");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定excel文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_excel(zip_path: &str, file_name: &str) -> Result<Vec<String>> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 打开指定文件
        let mut file = archive.by_name(file_name)?;

        // 创建一个临时缓冲区来存储Excel文件内容
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // 使用calamine读取Excel文件
        let mut workbook = Xlsx::new(std::io::Cursor::new(buffer)).unwrap();

        let mut lines = Vec::new();

        // 遍历每一个工作表
        for (sheet_number, (sheet_name, sheet)) in workbook.worksheets().into_iter().enumerate() {
            for (row_number, row) in sheet.rows().enumerate() {
                let mut line = String::new();
                // Process each row as needed
                // For example, you can iterate over the cells in the row
                for cell in row {
                    line.push_str(&cell.to_string());
                    line.push(' '); // Add a space between cells for readability
                }
                lines.push(line);
            }
        }

        Ok(lines)
    }



    /// 读取ZIP文件中的PDF文件内容
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的pdf文件的路径
    ///
    /// ### 返回值
    /// - `Result<String>`：返回指定pdf文件内容的字符串
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_pdf_as_string("src/test_examples/test_zip.zip","test_rar/test_pdf.pdf");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定pdf文件的内容: \n {}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_pdf_as_string(zip_path: &str, file_name: &str) -> Result<String> {
        // 检查ZIP文件是否存在
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");
    
        // 打开ZIP文件
        let zip_file = File::open(zip_path)?;
    
        // 创建 ZIP 归档实例
        let mut archive = ZipArchive::new(zip_file)?;
    
        // 查找并读取指定的PDF文件
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            if file.name() == file_name {
                assert!(file_name.ends_with(".pdf"), "文件类型错误,请检查文件类型是否为pdf文件");
                // 创建临时文件
                let temp_dir = tempfile::tempdir()?;
                let temp_file_path = temp_dir.path().join("temp.pdf");
    
                // 将PDF文件内容写入临时文件
                let mut temp_file = File::create(&temp_file_path)?;
                let mut buf = Vec::new();
                file.read_to_end(&mut buf)?;
                temp_file.write_all(&buf)?;
    
                // 从临时文件中读取PDF内容
                let pdf_text = extract_text(temp_file_path.to_str().unwrap());
                match pdf_text {
                    Ok(text) => return Ok(text.trim_start_matches('\n').trim_end_matches('\n').to_string()),
                    Err(err) => return Err(Error::new(ErrorKind::Other, format!("Failed to extract PDF text: {}", err))),
                }
            }
        }
    
        Err(Error::new(ErrorKind::NotFound, "PDF文件不存在"))
    }



    /// 读取zip文件中ppt内容,返回一个字符串，便于展示
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的ppt文件的路径
    ///
    /// ### 返回值
    /// - `Result<String>`：返回指定ppt文件内容的字符串
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_pptx_text("src/test_examples/test_zip.zip","test_rar/test_ppt.pptx");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定ppt文件的内容: \n {}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_pptx_text(zip_path: &str, file_name: &str) -> Result<String> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;

        // 找到指定的PPTX文件
        let mut file = archive.by_name(file_name)?;

        // 将PPTX文件内容读取到内存中
        let mut pptx_buffer = Vec::new();
        file.read_to_end(&mut pptx_buffer)?;

        // 使用Cursor包装字节数组，以便支持Seek操作
        let mut cursor = Cursor::new(pptx_buffer);

        // 从Cursor创建一个新的ZipArchive
        let mut pptx_archive = ZipArchive::new(&mut cursor)?;

        let mut slide_texts = String::new();

        // 遍历PPTX文件中的所有文件
        for i in 0..pptx_archive.len() {
            let mut file = pptx_archive.by_index(i)?;
            let file_name = file.name().to_string();

            // 查找幻灯片文件（通常在ppt/slides/目录下，文件名以"slide"开头）
            if file_name.starts_with("ppt/slides/slide") && file_name.ends_with(".xml") {
                let mut xml_content = String::new();
                file.read_to_string(&mut xml_content)?;

                // 解析XML内容，提取文本
                let mut slide_text = String::new();
                let parser = EventReader::from_str(&xml_content);
                for event in parser {
                    match event {
                        Ok(XmlEvent::Characters(text)) => {
                            slide_text.push_str(&text);
                            // slide_text.push('\n');
                        }
                        _ => {}
                    }
                }
                slide_texts.push_str(&slide_text);
                slide_texts.push_str("\n");
            }
        }

        Ok(slide_texts)
    }


    /// 按页读取ZIP文件中的PPTX文件的幻灯片文本内容
    ///
    /// ### 参数
    /// * `zip_path:<&str>` - ZIP文件的路径
    /// * `file_name:<&str>` - ZIP文件内部PPTX文件的路径（相对路径）
    ///
    /// ### 返回值
    /// * `Result<Vec<String>>` - 返回指定pptx文件中提取的所有幻灯片文本内容，每一页一个向量
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_pptx_text_by_slide("src/test_examples/test_zip.zip","test_rar/test_ppt.pptx");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定pptx文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_pptx_text_by_slide(zip_path: &str, file_name: &str) -> Result<Vec<String>> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;

        // 找到指定的PPTX文件
        let mut file = archive.by_name(file_name)?;

        // 将PPTX文件内容读取到内存中
        let mut pptx_buffer = Vec::new();
        file.read_to_end(&mut pptx_buffer)?;

        // 使用Cursor包装字节数组，以便支持Seek操作
        let mut cursor = Cursor::new(pptx_buffer);

        // 从Cursor创建一个新的ZipArchive
        let mut pptx_archive = ZipArchive::new(&mut cursor)?;

        let mut slide_texts = Vec::new();

        // 遍历PPTX文件中的所有文件
        for i in 0..pptx_archive.len() {
            let mut file = pptx_archive.by_index(i)?;
            let file_name = file.name().to_string();

            // 查找幻灯片文件（通常在ppt/slides/目录下，文件名以"slide"开头）
            if file_name.starts_with("ppt/slides/slide") && file_name.ends_with(".xml") {
                let mut xml_content = String::new();
                file.read_to_string(&mut xml_content)?;

                // 解析XML内容，提取文本
                let mut slide_text = String::new();
                let parser = EventReader::from_str(&xml_content);
                for event in parser {
                    match event {
                        Ok(XmlEvent::Characters(text)) => {
                            slide_text.push_str(&text);
                            // slide_text.push('\n');
                        }
                        _ => {}
                    }
                }
                slide_texts.push(slide_text);
            }
        }

        Ok(slide_texts)
    }
    

    /// 读取ZIP文件中ppt备注
    ///
    /// ### 参数
    /// - `zip_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的ppt文件的路径
    ///
    /// ### 返回值
    /// - `Result<Vec<String>>`：返回指定ppt文件备注的字符串数组
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_zip_pptx_notes("src/test_examples/test_zip.zip","test_rar/test_ppt.pptx");
    /// match res {
    ///   Ok(content) => println!("zip文件中指定ppt文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取zip文件失败: {}", e), 
    /// }
    pub fn read_zip_pptx_notes(zip_path: &str, file_name: &str) -> Result<Vec<String>> {
        assert!(fs::metadata(zip_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(zip_path.ends_with(".zip"), "文件类型错误,请检查文件类型是否为zip文件");

        let zip_file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(zip_file)?;
        // 找到指定的PPTX文件
        let mut file = archive.by_name(file_name)?;
        // 将PPTX文件内容读取到内存中
        let mut pptx_buffer = Vec::new();
        file.read_to_end(&mut pptx_buffer)?;
        // 使用Cursor包装字节数组，以便支持Seek操作
        let mut cursor = Cursor::new(pptx_buffer);
        // 从Cursor创建一个新的ZipArchive
        let mut pptx_archive = ZipArchive::new(&mut cursor)?;
    
        let mut notes = Vec::new();
    
        // 遍历PPTX文件中的所有文件
        for i in 0..pptx_archive.len() {
            let mut file = pptx_archive.by_index(i)?;
            if file.name().starts_with("ppt/notesSlides/notesSlide") && file.name().ends_with(".xml") {
                let mut xml_content = String::new();
                file.read_to_string(&mut xml_content)?;
            
                // Parse XML content
                let parser = EventReader::from_str(&xml_content);
                let mut note_content = String::new();
                let mut in_text_run = false;
            
                for event in parser {
                    match event {
                        Ok(XmlEvent::StartElement { name, .. }) => {
                            if name.local_name == "t" {
                                // Start text node
                                in_text_run = true;
                            }
                        },
                        Ok(XmlEvent::Characters(text)) if in_text_run => {
                            note_content.push_str(&text);
                            note_content.push_str("\n");
                        },
                        Ok(XmlEvent::EndElement { name }) => {
                            if name.local_name == "t" {
                                in_text_run = false;
                            }
                        },
                        Err(_) => {
                            // Handle error
                            break;
                        },
                        _ => {}
                    }
                }
    
                if !note_content.is_empty() {
                    // println!("note is {}", &note_content);
                    // 这里主要是去除每条备注之后的序号
                    let note_content = note_content.trim_end_matches(|c: char| c.is_digit(10) || c.is_whitespace()).to_string();
                    notes.push(note_content);
                }
            }
        }
    
        Ok(notes)
    } 






    // 这里写rar文件的操作



    /// 列出 RAR 文件中的所有文件和目录
    ///
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `exclude_directories:<bool>`: 是否包含压缩包里的文件目录
    ///
    /// ### 返回值
    /// - `Result<Vec<String>, Box<dyn Error>>`: 成功时返回文件和目录的名称列表，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::list_rar_filenames("src/test_examples/test_rar.rar",true);
    /// match res {
    ///   Ok(content) => println!("rar文件中所有文件和目录: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn list_rar_filenames(rar_path: &str, exclude_directories: bool) -> Result<Vec<String>> {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");

        let archive = Archive::new(rar_path);
        let entries = archive.open_for_listing().unwrap();
    
        let file_list = entries
            .filter_map(|entry| {
                entry.ok()
                    .and_then(|e| {
                        if !exclude_directories && e.is_directory() {
                            None
                        } else {
                            e.filename.to_str().map(|s| s.to_string())
                        }
                    })
            })
            .collect::<Vec<_>>();
    
        Ok(file_list)
    }
    

    /// 读取rar压缩包内的txt文件内容
    ///
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的文本文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<String, Box<dyn UnrarError>>`: 成功时返回文件文本内容，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_txt("src/test_examples/test_rar.rar","test_rar\\test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定txt文件的内容: \n {}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_txt(rar_path: &str, file_name: &str) -> UnrarResult<String> {
        assert!(std::fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
                return Ok(String::from_utf8_lossy(&file_data).to_string());
            }
            
            archive = header.skip()?;
        }
    }



    /// 按行读取rar文件中txt文件内容
    ///
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的文本文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回包含指定的rar文本文件内容的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_txt_by_line("src/test_examples/test_rar.rar","test_rar\\test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定txt文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_txt_by_line(rar_path: &str, file_name: &str) -> UnrarResult<Vec<String>> {
        assert!(std::fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
    
                let reader = BufReader::new(&file_data[..]);
                let mut lines = Vec::new();
                for line in reader.lines() {
                    lines.push(line.unwrap());
                }
                return Ok(lines);
            }
    
            archive = header.skip()?;
        }
    }


    /// 按字节读取rar文件中txt文件内容
    ///
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的文本文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<u8>, Box<dyn UnrarError>>`: 成功时返回包含指定的rar文本文件内容的字节数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_txt_by_byte("src/test_examples/test_rar.rar","test_rar\\test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定txt文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_txt_by_byte(rar_path: &str, file_name: &str) -> UnrarResult<Vec<u8>> {
        assert!(std::fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
    
                let mut reader = BufReader::new(&file_data[..]);
                let mut bytes = Vec::new();
                let mut buffer = [0u8; 1];
                loop {
                    match reader.read(&mut buffer) {
                        Ok(0) => break, // 文件结束
                        Ok(_) => bytes.push(buffer[0]),
                        Err(e) => return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read)),
                    }
                }
                return Ok(bytes);
            }
    
            archive = header.skip()?;
        }
    }


    /// 按块读取rar文件中txt文件内容
    ///
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的文本文件的路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回一个包含每个块的向量，每个块都是一个字符串，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_txt_by_block("src/test_examples/test_rar.rar","test_rar\\test_txt.txt",4);
    /// match res {
    ///   Ok(content) => println!("rar文件中指定txt文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_txt_by_block(rar_path: &str, file_name: &str, block_size: usize) -> UnrarResult<Vec<String>> {
        assert!(std::fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
    
                let mut reader = BufReader::new(&file_data[..]);
                let mut blocks = Vec::new();
                let mut buffer = vec![0; block_size];
    
                loop {
                    let bytes_read = reader.read(&mut buffer).unwrap();
                    if bytes_read == 0 {
                        break; // 文件末尾
                    }
                    blocks.push(String::from_utf8_lossy(&buffer[..bytes_read]).to_string());
                }
    
                return Ok(blocks);
            }
    
            archive = header.skip()?;
        }
    }


    /// 读取rar压缩包中csv文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的csv文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<Vec<String>>, Box<dyn UnrarError>>`: 成功时返回一个二维字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_csv("src/test_examples/test_rar.rar","test_rar\\test_csv.csv");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定csv文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_csv(rar_path: &str, file_name: &str) -> UnrarResult<Vec<Vec<String>>> {
        assert!(std::fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");

        let mut archive = Archive::new(rar_path).open_for_processing()?;

        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }

            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
                
                let reader = BufReader::new(&file_data[..]);
                let mut rdr = ReaderBuilder::new().from_reader(reader);
                let mut records = Vec::new();
                
                for result in rdr.records() {
                    let record = result.map_err(|_| UnrarError::from(unrar::error::Code::Unknown, unrar::error::When::Read))?;
                    let record_vec: Vec<String> = record.iter().map(|s| s.to_string()).collect();
                    records.push(record_vec);
                }
                return Ok(records);
            }

            archive = header.skip()?;
        }
    }


    /// 按列读取csv文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的csv文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<Vec<String>>, Box<dyn UnrarError>>`: 成功时返回一个二维字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_csv_by_column("src/test_examples/test_rar.rar","test_rar\\test_csv.csv");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定csv文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_csv_by_column(rar_path: &str, file_name: &str) -> UnrarResult<Vec<Vec<String>>> {
        assert!(std::fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
    
                let reader = BufReader::new(&file_data[..]);
                let mut rdr = ReaderBuilder::new().from_reader(reader);
                let mut records:Vec<Vec<String>> = Vec::new();
                
                // 读取所有行
                for result in rdr.records() {
                    let record = result.map_err(|_| UnrarError::from(unrar::error::Code::Unknown, unrar::error::When::Read))?;
                    records.push(record.iter().map(|s| s.to_string()).collect());
                }
    
                // 按列转换
                let mut columns = vec![Vec::new(); records[0].len()];
                for record in records {
                    for (i, field) in record.into_iter().enumerate() {
                        columns[i].push(field);
                    }
                }
    
                return Ok(columns);
            }
    
            archive = header.skip()?;
        }
    }


    /// 按块读取rar中csv文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的csv文件的路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<Vec<Vec<String>>>, Box<dyn UnrarError>>`: 成功时返回一个向量，每个向量元素即为csv文件块的内容，用二维向量存储，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_csv_by_block("src/test_examples/test_rar.rar","test_rar\\test_csv.csv",4);
    /// match res {
    ///   Ok(content) => println!("rar文件中指定csv文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_csv_by_block(rar_path: &str, file_name: &str, block_size: usize) -> UnrarResult<Vec<Vec<Vec<String>>>> {
        assert!(std::fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
    
                let reader = BufReader::new(&file_data[..]);
                let mut rdr = ReaderBuilder::new().from_reader(reader);
                let mut blocks = Vec::new();
                let mut current_block = Vec::new();
    
                for result in rdr.records() {
                    let record = result.map_err(|_| UnrarError::from(unrar::error::Code::Unknown, unrar::error::When::Read))?;
                    let string_vec: Vec<String> = record.iter().map(|s| s.to_string()).collect();
                    current_block.push(string_vec);
    
                    if current_block.len() == block_size {
                        blocks.push(current_block.clone());
                        current_block.clear();
                    }
                }
                // 如果最后一个块不满，仍然保存
                if !current_block.is_empty() {
                    blocks.push(current_block);
                }
    
                return Ok(blocks);
            }
    
            archive = header.skip()?;
        }
    }


    /// 读取rar中json文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的json文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<Value, Box<dyn UnrarError>>`: 成功时返回Value，Value包含整个json文件的内容，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_json("src/test_examples/test_rar.rar","test_rar\\test_json.json");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定json文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_json(rar_path: &str, file_name: &str) -> UnrarResult<Value> {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
    
                let reader = BufReader::new(&file_data[..]);
                let json_data: Value = from_reader(reader)
                    .map_err(|_| UnrarError::from(unrar::error::Code::Unknown, unrar::error::When::Read))?;
                
                return Ok(json_data);
            }
    
            archive = header.skip()?;
        }
    }



    /// 按块读取json文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的json文件的路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<Vec<Value>>, Box<dyn UnrarError>>`: 成功时返回包含每个块的向量，每个块都是一个对象向量，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_json_by_block("src/test_examples/test_rar.rar","test_rar\\test_json.json",4);
    /// match res {
    ///   Ok(content) => println!("rar文件中指定json文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_json_by_block(rar_path: &str, file_name: &str, block_size: usize) -> UnrarResult<Vec<Vec<Value>>> {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
    
                let mut reader = BufReader::new(&file_data[..]);
                let mut content = String::new();
                reader.read_to_string(&mut content)
                    .map_err(|_| UnrarError::from(unrar::error::Code::Unknown, unrar::error::When::Read))?;
                
                let json_array: Value = from_str(&content)
                    .map_err(|_| UnrarError::from(unrar::error::Code::Unknown, unrar::error::When::Read))?;
                let array = json_array.as_array().ok_or_else(|| 
                    UnrarError::from(unrar::error::Code::Unknown, unrar::error::When::Read))?;
                
                let mut blocks = Vec::new();
                let mut current_block = Vec::with_capacity(block_size);
    
                for value in array {
                    current_block.push(value.clone());
                    if current_block.len() == block_size {
                        blocks.push(current_block.clone());
                        current_block.clear();
                    }
                }
                if !current_block.is_empty() {
                    blocks.push(current_block);
                }
    
                return Ok(blocks);
            }
    
            archive = header.skip()?;
        }
    }



    /// 读取rar中jsonl文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的jsonl文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<Value>, Box<dyn UnrarError>>`: 成功时返回Value数组，Value数组包含整个jsonl文件的内容，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_jsonl("src/test_examples/test_rar.rar","test_rar\\test_json.json");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定jsonl文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_jsonl(rar_path: &str, file_name: &str) -> UnrarResult<Vec<Value>> {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
    
                let reader = BufReader::new(&file_data[..]);
                let mut vec_json = Vec::new();
    
                for line in reader.lines() {
                    let line = line.map_err(|_| 
                        UnrarError::from(unrar::error::Code::Unknown, unrar::error::When::Read))?;
                    let json_value = from_str(&line)
                        .map_err(|_| UnrarError::from(unrar::error::Code::Unknown, unrar::error::When::Read))?;
                    vec_json.push(json_value);
                }
    
                return Ok(vec_json);
            }
    
            archive = header.skip()?;
        }
    }


    /// 从rar读取xml文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的xml文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<String, Box<dyn UnrarError>>`: 成功时返回一个字符串，字符串中包含指定xml文件全部内容，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_xml_text("src/test_examples/test_rar.rar","test_rar\\test_json.json");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定xml文件的内容: \n {}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_xml_text(rar_path: &str, file_name: &str) -> UnrarResult<String> {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
                let mut reader = BufReader::new(&file_data[..]);
                let mut content = String::new();
                reader.read_to_string(&mut content).unwrap();
                return Ok(content);
            }
    
            archive = header.skip()?;
        }
    }


    /// 按事件读取xml文件内容
    ///
    /// ### 参数
    /// - `rar_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的xml文件的路径
    /// - `on_start:<FnMut(&str)>`: 处理Start事件的函数
    /// - `on_text:<FnMut(&str)>`: 处理Text事件的函数
    /// - `on_end:<FnMut(&str)>`: 处理End事件的函数
    /// 
    /// 接受三个闭包类型的参数，从而获取对应XML的相关文件内容
    /// ### 返回值
    /// - `Result<()>`：无返回值
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// // 读取xml文件标签
    /// let mut start_tags:Vec<String> = Vec::new();
    /// let mut end_tags:Vec<String> = Vec::new();
    /// let mut text:Vec<String> = Vec::new();
    /// // 处理标签的闭包
    /// let on_start = |tag_name:&str|{
    ///     start_tags.push(tag_name.to_string());
    /// };
    /// let on_end = |tag_name:&str| {
    ///     end_tags.push(tag_name.to_string());
    /// };
    /// let on_text = |text_content:&str| {
    ///     text.push(text_content.to_string());
    /// };
    ///
    /// let res = FileReader::read_rar_xml("src/test_examples/test_rar.rar","test_rar\\test_xml.xml", on_start, on_text, on_end);
    /// match res {
    ///     Ok(()) => {
    ///         println!("Start tags:");
    ///         for start in start_tags {
    ///             println!("  {}", start);
    ///         }
    ///
    ///         println!("Text nodes:");
    ///         for text in text {
    ///             println!("  {}", text);
    ///         }
    ///
    ///         println!("end tags:");
    ///         for end in end_tags {
    ///             println!("  {}", end);
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Error processing XML: {}", e),
    /// }
    pub fn read_rar_xml<S, T, E>(
        rar_path: &str, 
        file_name: &str,
        mut on_start: S,
        mut on_text: T,
        mut on_end: E,
    ) -> UnrarResult<()>
    where
        S: FnMut(&str), 
        T: FnMut(&str),
        E: FnMut(&str),
    {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = unrar::Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
                let reader = BufReader::new(&file_data[..]);
                let mut xml_reader = quick_xml::Reader::from_reader(reader);
                xml_reader.trim_text(true);
                let mut buf = Vec::new();
    
                loop {
                    match xml_reader.read_event_into(&mut buf).unwrap() {
                        Event::Start(ref e) => {
                            let element_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                            on_start(&element_name);
                        }
                        Event::Text(e) => {
                            let text = e.unescape().unwrap().to_string();
                            on_text(&text);
                        }
                        Event::End(ref e) => {
                            let element_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                            on_end(&element_name);
                        }
                        Event::Eof => break,
                        _ => {}
                    }
                    buf.clear();
                }
    
                return Ok(());
            }
    
            archive = header.skip()?;
        }
    }



    /// 从rar中读取md文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的md文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<String, Box<dyn UnrarError>>`: 成功时返回一个字符串，字符串中包含指定md文件全部内容，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_markdown("src/test_examples/test_rar.rar","test_rar\\test_md.md");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定md文件的内容: \n {}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_markdown(rar_path: &str, file_name: &str) -> UnrarResult<String> {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
                let mut reader = BufReader::new(&file_data[..]);
                let mut content = String::new();
                reader.read_to_string(&mut content).unwrap();
                return Ok(content);
            }
    
            archive = header.skip()?;
        }
    }



    /// 按行读取md文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的md文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回一个字符串数组，每个字符串中包含指定md文件的内容，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_md_by_line("src/test_examples/test_rar.rar","test_rar\\test_md.md");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定md文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_md_by_line(rar_path: &str, file_name: &str) -> UnrarResult<Vec<String>> {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
                let reader = BufReader::new(&file_data[..]);
                let mut lines = Vec::new();
    
                for line in reader.lines() {
                    let line = line.unwrap();
                    lines.push(line);
                }
    
                return Ok(lines);
            }
    
            archive = header.skip()?;
        }
    }



    /// 按字节读取md文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的md文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<u8>, Box<dyn UnrarError>>`: 成功时返回一个u8字节数组，包含指定md文件全部内容，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_md_by_byte("src/test_examples/test_rar.rar","test_rar\\test_md.md");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定md文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_md_by_byte(rar_path: &str, file_name: &str) -> UnrarResult<Vec<u8>> {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
                let mut reader = BufReader::new(&file_data[..]);
                let mut buffer = [0u8; 1];
                let mut bytes = Vec::new();
    
                loop {
                    match reader.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(n) => bytes.extend_from_slice(&buffer[..n]),
                        Err(e) => return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read)),
                    }
                }
    
                return Ok(bytes);
            }
    
            archive = header.skip()?;
        }
    }


    /// 按块读取md文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的md文件的路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回指定Markdown文件内容的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_md_by_block("src/test_examples/test_rar.rar","test_rar\\test_md.md",4);
    /// match res {
    ///   Ok(content) => println!("rar文件中指定md文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_md_by_block(rar_path: &str, file_name: &str, block_size: usize) -> UnrarResult<Vec<String>> {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
                let mut reader = BufReader::new(&file_data[..]);
                let mut blocks = Vec::new();
                let mut buffer = vec![0; block_size];
    
                loop {
                    let bytes_read = reader.read(&mut buffer).unwrap();
                    if bytes_read == 0 {
                        break;
                    }
                    blocks.push(String::from_utf8_lossy(&buffer[..bytes_read]).to_string());
                }
    
                return Ok(blocks);
            }
    
            archive = header.skip()?;
        }
    }


    /// 从rar中读取excel文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的excel文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回指定excel文件内容的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_excel("src/test_examples/test_rar.rar","test_rar\\test_excel.xlsx");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定excel文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_excel(rar_path: &str, file_name: &str) -> UnrarResult<Vec<String>> {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
        let mut buffer = Vec::new();
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
                buffer.extend_from_slice(&file_data[..]);
                break;
            }
    
            archive = header.skip()?;
        }
    
        let mut workbook = Xlsx::new(std::io::Cursor::new(buffer)).unwrap();
        let mut lines = Vec::new();
    
        for (_, sheet) in workbook.worksheets().into_iter() {
            for row in sheet.rows() {
                let line = row.iter()
                    .map(|cell| cell.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                lines.push(line);
            }
        }
    
        Ok(lines)
    }



    /// 从rar中读取pdf文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的pdf文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<String, Box<dyn UnrarError>>`: 成功时返回指定pdf文件内容的字符串，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_pdf_as_string("src/test_examples/test_rar.rar","test_rar\\test_pdf.pdf");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定pdf文件的内容: \n {}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_pdf_as_string(rar_path: &str, file_name: &str) -> UnrarResult<String> {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                assert!(file_name.ends_with(".pdf"), "文件类型错误,请检查文件类型是否为pdf文件");
                let temp_dir = tempdir().unwrap();
                let temp_file_path = temp_dir.path().join("temp.pdf");
    
                let (file_data, _) = header.read()?;
                let mut temp_file = File::create(&temp_file_path).unwrap();
                temp_file.write_all(&file_data).unwrap();
    
                let pdf_text = extract_text(temp_file_path.to_str().unwrap()).unwrap();
                return Ok(pdf_text.trim().to_string());
            }
    
            archive = header.skip()?;
        }
    

    }



    /// 从rar中读取ppt文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的ppt文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<String, Box<dyn UnrarError>>`: 成功时返回指定ppt文件内容的字符串，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_pptx_text("src/test_examples/test_rar.rar","test_rar\\test_ppt.pptx");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定ppt文件的内容: \n {}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_pptx_text(rar_path: &str, file_name: &str) -> UnrarResult<String> {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
        let mut pptx_buffer = Vec::new();
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
                pptx_buffer.extend_from_slice(&file_data[..]);
                break;
            }
    
            archive = header.skip()?;
        }
    
        let mut cursor = Cursor::new(pptx_buffer);
        let mut pptx_archive = ZipArchive::new(&mut cursor).unwrap();
    
        let mut slide_texts = String::new();
    
        for i in 0..pptx_archive.len() {
            let mut file = pptx_archive.by_index(i).unwrap();
            let file_name = file.name().to_string();
    
            if file_name.starts_with("ppt/slides/slide") && file_name.ends_with(".xml") {
                let mut xml_content = String::new();
                file.read_to_string(&mut xml_content).unwrap();
    
                let parser = EventReader::from_str(&xml_content);
                for event in parser {
                    if let Ok(XmlEvent::Characters(text)) = event {
                        slide_texts.push_str(&text);
                    }
                }
                slide_texts.push_str("\n");
            }
        }
    
        Ok(slide_texts)
    }



    /// 按行读取rar中ppt文件内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的pptx文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回指定pptx文件中提取的所有幻灯片文本内容，每一页一个向量，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_rar_pptx_text_by_slide("src/test_examples/test_rar.rar","test_rar\\test_ppt.pptx");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定ppt文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_rar_pptx_text_by_slide(rar_path: &str, file_name: &str) -> UnrarResult<Vec<String>> {
        assert!(fs::metadata(rar_path).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(rar_path.ends_with(".rar"), "文件类型错误,请检查文件类型是否为rar文件");
    
        let mut archive = Archive::new(rar_path).open_for_processing()?;
        let mut pptx_buffer = Vec::new();
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == file_name {
                let (file_data, _) = header.read()?;
                pptx_buffer.extend_from_slice(&file_data[..]);
                break;
            }
    
            archive = header.skip()?;
        }
    
        let mut cursor = Cursor::new(pptx_buffer);
        let mut pptx_archive = ZipArchive::new(&mut cursor).unwrap();
    
        let mut slide_texts = Vec::new();
    
        for i in 0..pptx_archive.len() {
            let mut file = pptx_archive.by_index(i).unwrap();
            let file_name = file.name().to_string();
    
            if file_name.starts_with("ppt/slides/slide") && file_name.ends_with(".xml") {
                let mut xml_content = String::new();
                file.read_to_string(&mut xml_content).unwrap();
    
                let parser = EventReader::from_str(&xml_content);
                let mut slide_text = String::new();
    
                for event in parser {
                    if let Ok(XmlEvent::Characters(text)) = event {
                        slide_text.push_str(&text);
                    }
                }
                slide_texts.push(slide_text);
            }
        }
    
        Ok(slide_texts)
    }



    /// 按页读取rar中ppt文件备注内容
    /// ### 参数
    /// - `rar_path:<&str>`: RAR 文件的路径
    /// - `file_name:<&str>`: 要打开的pptx文件的路径
    ///
    /// ### 返回值
    /// - `UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回指定ppt文件备注的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_ppt_notes_from_rar("src/test_examples/test_rar.rar","test_rar\\test_ppt.pptx");
    /// match res {
    ///   Ok(content) => println!("rar文件中指定ppt文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取rar文件失败: {}", e), 
    /// }
    pub fn read_ppt_notes_from_rar(rar_path: &str, ppt_filename: &str) -> UnrarResult<Vec<String>> {
        let mut archive = Archive::new(&rar_path).open_for_processing()?;
    
        loop {
            let header = archive.read_header()?;
            if header.is_none() {
                return Err(unrar::error::UnrarError::from(unrar::error::Code::EndArchive, unrar::error::When::Read));
            }
    
            let header = header.unwrap();
            if header.entry().filename.to_str().unwrap() == ppt_filename {
                let (ppt_data, _rest) = header.read()?;
                
                // 创建临时文件
                let mut temp_file = NamedTempFile::new().unwrap();
                temp_file.write_all(&ppt_data).unwrap();
    
                // 获取临时文件路径
                let temp_path = temp_file.path();
    
                // 打开PPT文件
                let file = File::open(temp_path).unwrap();
                let mut zip = ZipArchive::new(file).unwrap();
                let mut notes_texts = Vec::new();
    
                // 遍历备注幻灯片文件
                for i in 0..zip.len() {
                    let mut file = zip.by_index(i).unwrap();
                    let file_name = file.name().to_string();
    
                    // 查找备注幻灯片文件（通常在ppt/notesSlides/目录下）
                    if file_name.starts_with("ppt/notesSlides/notesSlide") && file_name.ends_with(".xml") {
                        let mut xml_content = String::new();
                        file.read_to_string(&mut xml_content).unwrap();
    
                        // 解析XML内容，提取备注文本
                        let mut note_text = String::new();
                        let parser = EventReader::from_str(&xml_content);
                        for event in parser {
                            if let Ok(XmlEvent::Characters(text)) = event {
                                note_text.push_str(&text);
                            }
                        }
                        // 移除开头的空格
                        let note_text = note_text.trim_start().to_string();
                        // 去除每条备注之后的序号
                        let note_text = note_text.trim_end_matches(|c: char| c.is_digit(10) || c.is_whitespace()).to_string();
                        notes_texts.push(note_text);
                    }
                }
                return Ok(notes_texts);
            }
            
            archive = header.skip()?;
        }
    }



















    // 这里是tar文件的相关操作

    /// 列出文件目录
    ///
    /// ### 参数
    /// - `file_path:<P: AsRef<Path>>`: tar文件路径
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回包含tar所有文件目录的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::list_tar_filenames("src/test_examples/test_tar.tar");
    /// match res {
    ///   Ok(content) => println!("tar文件中所有文件目录: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn list_tar_filenames<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>> {
        assert!(fs::metadata(file_path.as_ref()).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(file_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误，请检查文件类型是否为 tar 文件");
    
        let file = File::open(file_path)?;
        let mut archive = tar::Archive::new(BufReader::new(file));
    
        let mut file_list = Vec::new();
    
        // 遍历所有条目
        for entry in archive.entries()? {
            match entry {
                Ok(entry) => {
                    let entry_path = entry.path()?;
                    let entry_name = entry_path.to_string_lossy().to_string();
    
                    // 根据 exclude_directories 的值来决定是否排除目录
                        file_list.push(entry_name);
                }
                Err(_) => continue, // 错误处理
            }
        }
        Ok(file_list)
    }


    /// 直接读取txt文件内容
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的文本文件的路径
    /// ### 返回值
    /// - `Result<String>`: 成功时返回包含指定的tar文本文件内容的字符串，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_text("src/test_examples/test_tar.tar","test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("tar文件中指定文本文件的内容: \n {}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_text<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<String> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(file_name.ends_with(".txt"), "文件类型错误，请检查文件类型是否为 txt 文件");
        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
    
        for entry in archive.entries()? {
            let mut entry = entry?;
            let entry_path = entry.path()?;
    
            // 检查文件名是否匹配
            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                let mut contents = String::new();
                entry.read_to_string(&mut contents)?;
                return Ok(contents);
            }
        }
    
        Err(io::Error::new(io::ErrorKind::NotFound, format!("未找到指定文件: {}", file_name)))
    }

    /// 按行读取txt文件内容
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的文本文件的路径
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回包含指定的tar文本文件内容的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_text_by_line("src/test_examples/test_tar.tar","test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("tar文件中指定文本文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_text_by_line<P: AsRef<Path>>(tar_path: P, file_name: &str) -> io::Result<Vec<String>> {
        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
    
        for entry in archive.entries()? {
            let entry = entry?;
            let entry_path = entry.path()?;
    
            // 检查文件名是否匹配
            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                let reader = BufReader::new(entry);
                let mut lines = Vec::new();
    
                for line in reader.lines() {
                    lines.push(line?);
                }
    
                return Ok(lines);
            }
        }
        Err(io::Error::new(io::ErrorKind::NotFound, format!("未找到指定文件: {}", file_name)))
    }


    /// 按字节读取文本文件内容
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的文本文件的路径
    /// ### 返回值
    /// - `Result<Vec<u8>>`: 成功时返回包含指定的tar文本文件内容的字节数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_txt_by_byte("src/test_examples/test_tar.tar","test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("tar文件中指定文本文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_txt_by_byte<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<Vec<u8>> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(tar_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误,请检查文件类型是否为 tar 文件");
    
        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
        
        // 遍历所有条目，查找指定文件
        for entry in archive.entries()? {
            let entry = entry?;
            let entry_path = entry.path()?;
            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                let mut reader = BufReader::new(entry);
                let mut buffer = [0u8; 1];
                let mut bytes = Vec::new();
                
                loop {
                    match reader.read(&mut buffer) {
                        Ok(0) => break, // 读到文件末尾
                        Ok(_) => bytes.extend_from_slice(&buffer), // 读取多个字节
                        Err(e) => return Err(e), // 读取错误
                    }
                }
                return Ok(bytes); // 返回读取的字节
            }
        }
    
        Err(io::Error::new(io::ErrorKind::NotFound, "指定文件未找到"))
    }

    /// 按块读取txt文本文件内容
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的文本文件的路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回一个包含每个块的向量，每个块都是一个字符串，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_txt_by_block("src/test_examples/test_tar.tar","test_txt.txt",4);
    /// match res {
    ///   Ok(content) => println!("tar文件中指定文本文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_txt_by_block<P: AsRef<Path>>(tar_path: P, file_name: &str, block_size: usize) -> Result<Vec<String>> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在,请检查路径是否正确");
        assert!(tar_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误,请检查文件类型是否为 tar 文件");
    
        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
    
        // 遍历所有条目，查找指定文件
        for entry in archive.entries()? {
            let  entry = entry?;
            let entry_path = entry.path()?;

            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                let mut reader = BufReader::new(entry);
                let mut blocks = Vec::new();
                let mut buffer = vec![0; block_size]; // 创建一个包含 block_size 个 0 元素的向量
    
                loop {
                    let bytes_read = reader.read(&mut buffer)?; // 一次性读取 buffer 大小字节
                    if bytes_read == 0 {
                        break; // 读取到文件末尾
                    }
                    // 将读取的字节转换为字符串并添加到块中
                    blocks.push(String::from_utf8_lossy(&buffer[..bytes_read]).to_string());
                }
                return Ok(blocks); // 返回读取的块
            }
        }
    
        Err(io::Error::new(io::ErrorKind::NotFound, "指定文件未找到"))
    }



    /// 按行读取csv文件
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的csv文件的路径
    /// ### 返回值
    /// - `Result<Vec<Vec<String>>>`: 成功时返回一个二维字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_csv_by_line("src/test_examples/test_tar.tar","test_csv.csv");
    /// match res {
    ///   Ok(content) => println!("tar文件中csv文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_csv_by_line<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<Vec<Vec<String>>> {
        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
    
        for entry in archive.entries()? {
            let  entry = entry?;
            let entry_path = entry.path()?;
    
            // 检查文件名是否匹配
            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                let reader = BufReader::new(entry);
                let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);
                let mut records = Vec::new();
    
                for result in csv_reader.records() {
                    let record = result?;
                    records.push(record.iter().map(|s| s.to_string()).collect());
                }
    
                return Ok(records);
            }
        }
    
        Err(io::Error::new(io::ErrorKind::NotFound, format!("未找到指定文件: {}", file_name)))
    }


    /// 按列读取csv文件
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的csv文件的路径
    /// ### 返回值
    /// - `Result<Vec<Vec<String>>>`: 成功时返回一个二维字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_csv_by_column("src/test_examples/test_tar.tar","test_csv.csv");
    /// match res {
    ///   Ok(content) => println!("tar文件中csv文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_csv_by_column<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<Vec<Vec<String>>> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(tar_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误，请检查文件类型是否为 tar 文件");
    
        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
        
        // 遍历条目，找到指定的 CSV 文件
        for entry in archive.entries()? {
            let  entry = entry?;
            let entry_path = entry.path()?;
            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                let  reader = BufReader::new(entry);
                let mut rdr = ReaderBuilder::new().from_reader(reader);
                let mut records: Vec<Vec<String>> = Vec::new();
    
                // 读取所有行
                for result in rdr.records() {
                    let record = result?;
                    records.push(record.iter().map(|s| s.to_string()).collect());
                }
    
                // 按列转换
                let mut columns = vec![Vec::new(); records[0].len()];
                for record in records {
                    for (i, field) in record.into_iter().enumerate() {
                        columns[i].push(field);
                    }
                }
    
                return Ok(columns);
            }
        }
    
        Err(io::Error::new(io::ErrorKind::NotFound, "指定的 CSV 文件未找到"))
    }



    /// 按块读取csv文件
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的csv文件的路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    /// ### 返回值
    /// - `Result<Vec<Vec<Vec<String>>>>`: 成功时返回一个向量，每个向量元素即为csv文件块的内容，用二维向量存储，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_csv_by_block("src/test_examples/test_tar.tar","test_csv.csv",4);
    /// match res {
    ///   Ok(content) => println!("tar文件中csv文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_csv_by_block<P: AsRef<Path>>(tar_path: P, file_name: &str, block_size: usize) -> Result<Vec<Vec<Vec<String>>>> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(tar_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误，请检查文件类型是否为 tar 文件");
    
        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
    
        // 遍历条目，找到指定的 CSV 文件
        for entry in archive.entries()? {
            let entry = entry?;
            let entry_path = entry.path()?;

            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                let reader = BufReader::new(entry);
                let mut rdr = ReaderBuilder::new().from_reader(reader);
                let mut blocks = Vec::new();
                let mut current_block = Vec::new();
    
                for result in rdr.records() {
                    let record = result.expect("Failed to read record");
                    let string_vec: Vec<String> = record.iter().map(String::from).collect(); // 一行数据
                    current_block.push(string_vec);
    
                    if current_block.len() == block_size { // 满足指定行数，就添加块到结果中
                        blocks.push(current_block.clone());
                        current_block.clear();
                    }
                }
                // 如果最后一个块不满，仍然保存
                if !current_block.is_empty() {
                    blocks.push(current_block);
                }
    
                return Ok(blocks);
            }
        }
    
        Err(io::Error::new(io::ErrorKind::NotFound, "指定的 CSV 文件未找到"))
    }

    /// 读取json文件内容
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的json文件的路径
    /// ### 返回值
    /// - `Result<Value>`: 成功时返回Value，Value包含整个json文件的内容，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_json("src/test_examples/test_tar.tar","test_json.json");
    /// match res {
    ///   Ok(content) => println!("tar文件中json文件的内容: \n {}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_json<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<Value> {
        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
    
        for entry in archive.entries()? {
            let entry = entry?;
            let entry_path = entry.path()?;
    
            // 检查文件名是否匹配
            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                let reader = BufReader::new(entry);
                let json_value: Value = serde_json::from_reader(reader)?;
                return Ok(json_value);
            }
        }
    
        Err(io::Error::new(io::ErrorKind::NotFound, format!("未找到指定文件: {}", file_name)))
    }

    /// 按行读取jsonl文件内容
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的jsonl文件的路径
    /// ### 返回值
    /// - `Result<Vec<Value>>`: 成功时返回Value数组，Value数组包含整个jsonl文件的内容，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_jsonl_by_line("src/test_examples/test_tar.tar","test_jsonline.jsonl");
    /// match res {
    ///   Ok(content) => println!("tar文件中jsonl文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_jsonl_by_line<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<Vec<Value>> {
        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
    
        for entry in archive.entries()? {
            let entry = entry?;
            let entry_path = entry.path()?;
    
            // 检查文件名是否匹配
            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                let reader = BufReader::new(entry);
                let mut records = Vec::new();
    
                for line in reader.lines() {
                    let line = line?;
                    let json_value: Value = serde_json::from_str(&line)?;
                    records.push(json_value);
                }
    
                return Ok(records);
            }
        }
    
        Err(io::Error::new(io::ErrorKind::NotFound, format!("未找到指定文件: {}", file_name)))
    }


    /// 直接读取xml文件内容
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的xml文件的路径
    /// ### 返回值
    /// - `Result<String>`: 成功时返回一个字符串，字符串中包含指定xml文件全部内容，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_xml_text("src/test_examples/test_tar.tar","test_xml.xml");
    /// match res {
    ///   Ok(content) => println!("tar文件中xml文件的内容: \n {}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_xml_text<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<String> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(tar_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误，请检查文件类型是否为 tar 文件");
    
        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
        
        // 打开指定文件
        for entry in archive.entries()? {
            let entry = entry?;
            let entry_path = entry.path()?;
    
            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                let mut reader = BufReader::new(entry);
                let mut content = String::new();
                reader.read_to_string(&mut content)?;
                return Ok(content);
            }
        }
    
        Err(io::Error::new(io::ErrorKind::NotFound, format!("未找到指定文件: {}", file_name)))
    }


    /// 按照事件读取xml文件内容
    ///
    /// ### 参数
    /// - `tar_path:<&str>`: 文件路径
    /// - `file_name:<&str>`: 要打开的xml文件的路径
    /// - `on_start:<FnMut(&str)>`: 处理Start事件的函数
    /// - `on_text:<FnMut(&str)>`: 处理Text事件的函数
    /// - `on_end:<FnMut(&str)>`: 处理End事件的函数
    /// 
    /// 接受三个闭包类型的参数，从而获取对应XML的相关文件内容
    /// ### 返回值
    /// - `Result<()>`：无返回值
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// // 读取xml文件标签
    /// let mut start_tags:Vec<String> = Vec::new();
    /// let mut end_tags:Vec<String> = Vec::new();
    /// let mut text:Vec<String> = Vec::new();
    /// // 处理标签的闭包
    /// let on_start = |tag_name:&str|{
    ///     start_tags.push(tag_name.to_string());
    /// };
    /// let on_end = |tag_name:&str| {
    ///     end_tags.push(tag_name.to_string());
    /// };
    /// let on_text = |text_content:&str| {
    ///     text.push(text_content.to_string());
    /// };
    ///
    /// let res = FileReader::read_tar_xml_by_listener("src/test_examples/test_tar.tar","test_xml.xml", on_start, on_text, on_end);
    /// match res {
    ///     Ok(()) => {
    ///         println!("Start tags:");
    ///         for start in start_tags {
    ///             println!("  {}", start);
    ///         }
    ///
    ///         println!("Text nodes:");
    ///         for text in text {
    ///             println!("  {}", text);
    ///         }
    ///
    ///         println!("end tags:");
    ///         for end in end_tags {
    ///             println!("  {}", end);
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Error processing XML: {}", e),
    /// }
    pub fn read_tar_xml_by_listener<S, T, E>(
        tar_path: &str,
        file_name: &str,
        mut on_start: S,
        mut on_text: T,
        mut on_end: E,
    ) -> Result<()> 
    where
        S: FnMut(&str),
        T: FnMut(&str),
        E: FnMut(&str),
    {
        assert!(fs::metadata(tar_path).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(tar_path.ends_with(".tar"), "文件类型错误，请检查文件类型是否为 tar 文件");
    
        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
        
        // 打开指定文件
        for entry in archive.entries()? {
            let entry = entry?;
            let entry_path = entry.path()?;
    
            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                let reader = BufReader::new(entry);
                let mut xml_reader = quick_xml::Reader::from_reader(reader);
                xml_reader.trim_text(true);
                let mut buf = Vec::new();
    
                loop {
                    match xml_reader.read_event_into(&mut buf) {
                        Ok(Event::Start(ref e)) => {
                            let element_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                            on_start(&element_name);
                        }
                        Ok(Event::Text(e)) => {
                            let text = e.unescape().unwrap().to_string();
                            on_text(&text);
                        }
                        Ok(Event::End(ref e)) => {
                            let element_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                            on_end(&element_name);
                        }
                        Ok(Event::Eof) => break,
                        Err(e) => {
                            eprintln!("Error at position {}: {:?}", xml_reader.buffer_position(), e);
                            break;
                        }
                        _ => {}
                    }
    
                    buf.clear(); // 清除缓冲区
                }
    
                return Ok(()); // 成功读取完毕
            }
        }
    
        Err(io::Error::new(io::ErrorKind::NotFound, format!("未找到指定文件: {}", file_name)))
    }




    /// 直接读取md文件内容
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的md文件的路径
    /// ### 返回值
    /// - `Result<String>`: 成功时返回一个字符串，字符串中包含指定md文件全部内容，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_markdown("src/test_examples/test_tar.tar","test_md.md");
    /// match res {
    ///   Ok(content) => println!("tar文件中md文件的内容: \n {}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_markdown<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<String> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(tar_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误，请检查文件类型是否为 tar 文件");

        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));

        // 遍历条目，查找指定的 Markdown 文件
        for entry in archive.entries()? {
            let entry = entry?;
            let entry_path = entry.path()?;

            // 只匹配文件名
            if entry_path.file_name().map_or(false, |name| name == file_name) {
                let mut reader = BufReader::new(entry);
                let mut content = String::new();
                reader.read_to_string(&mut content)?;
                return Ok(content);
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, "指定的 Markdown 文件未找到"))
    }



    /// 按行读取md文件内容
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的md文件的路径
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回一个字符串数组，每个字符串中包含指定md文件的内容，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_md_by_line("src/test_examples/test_tar.tar","test_md.md");
    /// match res {
    ///   Ok(content) => println!("tar文件中md文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_md_by_line<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<Vec<String>> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(tar_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误，请检查文件类型是否为 tar 文件");

        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
        
        // 打开指定文件
        for entry in archive.entries()? {
            let entry = entry?;
            let entry_path = entry.path()?;

            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                let mut reader = BufReader::new(entry);
                let mut lines = Vec::new();

                loop {
                    let mut line = String::new();
                    match reader.read_line(&mut line) {
                        Ok(0) => break, // 0 表示读到的字节数，0 表示读到文件末尾
                        Ok(_) => lines.push(line.trim_end_matches('\n').to_string()), // 正常读取
                        Err(e) => return Err(e), // 读取错误
                    }
                }

                return Ok(lines); // 返回读取到的行
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, format!("未找到指定文件: {}", file_name)))
    }



    /// 按字节读取 md文件
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的md文件的路径
    /// ### 返回值
    /// - `Result<Vec<u8>>`: 成功时返回一个u8字节数组，包含指定md文件全部内容，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_md_by_byte("src/test_examples/test_tar.tar","test_md.md");
    /// match res {
    ///   Ok(content) => println!("tar文件中md文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_md_by_byte<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<Vec<u8>> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(tar_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误，请检查文件类型是否为 tar 文件");

        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));

        // 遍历条目，查找指定的 Markdown 文件
        for entry in archive.entries()? {
            let entry = entry?;
            let entry_path = entry.path()?;

            // 只匹配文件名
            if entry_path.file_name().map_or(false, |name| name == file_name) {
                let mut reader = BufReader::new(entry);
                let mut bytes = Vec::new();
                let mut buffer = [0u8; 1];

                loop {
                    match reader.read(&mut buffer) {
                        Ok(0) => break, // 读取到文件末尾
                        Ok(_) => bytes.push(buffer[0]), // 将字节存入 Vec
                        Err(e) => return Err(e),
                    }
                }

                return Ok(bytes);
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, "指定的 Markdown 文件未找到"))
    }



    /// 按块读取md文件内容
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的md文件的路径
    /// - `block_size:<usize>`: 每次读取的块大小，以字节为单位
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回指定Markdown文件内容的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_md_by_block("src/test_examples/test_tar.tar","test_md.md",4);
    /// match res {
    ///   Ok(content) => println!("tar文件中md文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_md_by_block<P: AsRef<Path>>(tar_path: P, file_name: &str, block_size: usize) -> Result<Vec<String>> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(tar_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误，请检查文件类型是否为 tar 文件");

        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
        let mut blocks = Vec::new();
        let mut buffer = vec![0; block_size];

        // 遍历条目，查找指定的 Markdown 文件
        for entry in archive.entries()? {
            let entry = entry?;
            let entry_path = entry.path()?;

            // 只匹配文件名
            if entry_path.file_name().map_or(false, |name| name == file_name) {
                let mut reader = BufReader::new(entry);

                loop {
                    let bytes_read = reader.read(&mut buffer)?; // 一次性读取 buffer 大小字节
                    if bytes_read == 0 {
                        break; // 读取到文件末尾
                    }
                    // 将读取的字节转换为字符串并添加到块中
                    blocks.push(String::from_utf8_lossy(&buffer[..bytes_read]).to_string());
                }

                return Ok(blocks);
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, "指定的 Markdown 文件未找到"))
    }



    /// 读取整个excel文件内容
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的excel文件的路径
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回指定excel文件内容的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_excel("src/test_examples/test_tar.tar","test_excel.xlsx");
    /// match res {
    ///   Ok(content) => println!("tar文件中excel文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_excel<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<Vec<String>> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(tar_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误，请检查文件类型是否为 tar 文件");

        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));

        // 打开指定文件
        for entry in archive.entries()? {
            let mut entry = entry?;
            let entry_path = entry.path()?;

            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                // 创建一个临时缓冲区来存储 Excel 文件内容
                let mut buffer = Vec::new();
                entry.read_to_end(&mut buffer)?; // 读取 Excel 文件内容到缓冲区

                // 使用 calamine 读取 Excel 文件
                let mut workbook = Xlsx::new(std::io::Cursor::new(buffer)).unwrap();

                let mut lines = Vec::new();

                // 遍历每一个工作表
                for (sheet_number, (sheet_name, sheet)) in workbook.worksheets().into_iter().enumerate() {
                    for (row_number, row) in sheet.rows().enumerate() {
                        let mut line = String::new();
                        // 处理每一行
                        for cell in row {
                            line.push_str(&cell.to_string());
                            line.push(' '); // 在单元格之间添加空格
                        }
                        lines.push(line);
                    }
                }

                return Ok(lines); // 返回读取到的行
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, format!("未找到指定文件: {}", file_name)))
    }



    /// 读取pdf文件内容
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的pdf文件的路径
    /// ### 返回值
    /// - `Result<String>`: 成功时返回指定pdf文件内容的字符串，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_pdf_as_string("src/test_examples/test_tar.tar","test_pdf.pdf");
    /// match res {
    ///   Ok(content) => println!("tar文件中pdf文件的内容: \n {}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_pdf_as_string<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<String> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(tar_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误，请检查文件类型是否为 tar 文件");

        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));

        // 查找并读取指定的 PDF 文件
        for entry in archive.entries()? {
            let mut entry = entry?;
            let entry_path = entry.path()?;

            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                assert!(file_name.ends_with(".pdf"), "文件类型错误，请检查文件类型是否为 pdf 文件");

                // 创建临时文件
                let temp_dir = tempdir()?;
                let temp_file_path = temp_dir.path().join("temp.pdf");

                // 将 PDF 文件内容写入临时文件
                let mut temp_file = File::create(&temp_file_path)?;
                let mut buf = Vec::new();
                entry.read_to_end(&mut buf)?;
                temp_file.write_all(&buf)?;

                // 从临时文件中读取 PDF 内容
                let pdf_text = extract_text(temp_file_path.to_str().unwrap());
                match pdf_text {
                    Ok(text) => return Ok(text.trim_start_matches('\n').trim_end_matches('\n').to_string()),
                    Err(err) => return Err(Error::new(ErrorKind::Other, format!("无法提取 PDF 文本: {}", err))),
                }
            }
        }

        Err(Error::new(ErrorKind::NotFound, "PDF 文件不存在"))
    }




    /// 按行读取ppt文件内容
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的ppt文件的路径
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回指定pptx文件中提取的所有幻灯片文本内容，每一页一个向量，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_pptx_text_by_slide("src/test_examples/test_tar.tar","test_ppt.pptx");
    /// match res {
    ///   Ok(content) => println!("tar文件中ppt文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_pptx_text_by_slide<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<Vec<String>> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(tar_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误，请检查文件类型是否为 tar 文件");

        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));

        // 找到指定的 PPTX 文件
        for entry in archive.entries()? {
            let mut entry = entry?;
            let entry_path = entry.path()?;

            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                // 将 PPTX 文件内容读取到内存中
                let mut pptx_buffer = Vec::new();
                entry.read_to_end(&mut pptx_buffer)?;

                // 使用 Cursor 包装字节数组，以便支持 Seek 操作
                let mut cursor = Cursor::new(pptx_buffer);

                // 从 Cursor 创建一个新的 ZipArchive
                let mut pptx_archive = ZipArchive::new(&mut cursor)?;

                let mut slide_texts = Vec::new();

                // 遍历 PPTX 文件中的所有文件
                for i in 0..pptx_archive.len() {
                    let mut file = pptx_archive.by_index(i)?;
                    let file_name = file.name().to_string();

                    // 查找幻灯片文件（通常在 ppt/slides/ 目录下，文件名以 "slide" 开头）
                    if file_name.starts_with("ppt/slides/slide") && file_name.ends_with(".xml") {
                        let mut xml_content = String::new();
                        file.read_to_string(&mut xml_content)?;

                        // 解析 XML 内容，提取文本
                        let mut slide_text = String::new();
                        let parser = EventReader::from_str(&xml_content);
                        for event in parser {
                            match event {
                                Ok(XmlEvent::Characters(text)) => {
                                    slide_text.push_str(&text);
                                    // slide_text.push('\n');
                                }
                                _ => {}
                            }
                        }
                        slide_texts.push(slide_text);
                    }
                }

                return Ok(slide_texts); // 返回幻灯片文本
            }
        }

        Err(Error::new(ErrorKind::NotFound, "PPT 文件不存在"))
    }



    /// 读取ppt备注信息
    ///
    /// ### 参数
    /// - `tar_path:<P: AsRef<Path>>`: tar文件路径
    /// - `file_name:<&str>`: 要打开的ppt文件的路径
    /// ### 返回值
    /// - `Result<Vec<String>>`: 成功时返回指定ppt文件备注的字符串数组，失败时返回错误信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::read_tar_pptx_notes("src/test_examples/test_tar.tar","test_ppt.pptx");
    /// match res {
    ///   Ok(content) => println!("tar文件中ppt文件的内容: \n {:?}", content),
    ///   Err(e) => println!("读取tar文件失败: {}", e), 
    /// }
    pub fn read_tar_pptx_notes<P: AsRef<Path>>(tar_path: P, file_name: &str) -> Result<Vec<String>> {
        assert!(fs::metadata(tar_path.as_ref()).is_ok(), "文件不存在，请检查路径是否正确");
        assert!(tar_path.as_ref().to_string_lossy().ends_with(".tar"), "文件类型错误，请检查文件类型是否为 tar 文件");

        let tar_file = File::open(tar_path)?;
        let mut archive = tar::Archive::new(BufReader::new(tar_file));
        
        // 找到指定的 PPTX 文件
        for entry in archive.entries()? {
            let mut entry = entry?;
            let entry_path = entry.path()?;

            if entry_path.file_name().map(|name| name.to_string_lossy() == file_name).unwrap_or(false) {
                // 将 PPTX 文件内容读取到内存中
                let mut pptx_buffer = Vec::new();
                entry.read_to_end(&mut pptx_buffer)?;

                // 使用 Cursor 包装字节数组，以便支持 Seek 操作
                let mut cursor = Cursor::new(pptx_buffer);
                
                // 从 Cursor 创建一个新的 ZipArchive
                let mut pptx_archive = ZipArchive::new(&mut cursor)?;

                let mut notes = Vec::new();

                // 遍历 PPTX 文件中的所有文件
                for i in 0..pptx_archive.len() {
                    let mut file = pptx_archive.by_index(i)?;
                    if file.name().starts_with("ppt/notesSlides/notesSlide") && file.name().ends_with(".xml") {
                        let mut xml_content = String::new();
                        file.read_to_string(&mut xml_content)?;

                        // 解析 XML 内容
                        let parser = EventReader::from_str(&xml_content);
                        let mut note_content = String::new();
                        let mut in_text_run = false;

                        for event in parser {
                            match event {
                                Ok(XmlEvent::StartElement { name, .. }) => {
                                    if name.local_name == "t" {
                                        // 开始文本节点
                                        in_text_run = true;
                                    }
                                },
                                Ok(XmlEvent::Characters(text)) if in_text_run => {
                                    note_content.push_str(&text);
                                    note_content.push_str("\n");
                                },
                                Ok(XmlEvent::EndElement { name }) => {
                                    if name.local_name == "t" {
                                        in_text_run = false;
                                    }
                                },
                                Err(_) => {
                                    // 处理错误
                                    break;
                                },
                                _ => {}
                            }
                        }

                        if !note_content.is_empty() {
                            // 去除每条备注之后的序号
                            let note_content = note_content.trim_end_matches(|c: char| c.is_digit(10) || c.is_whitespace()).to_string();
                            notes.push(note_content);
                        }
                    }
                }

                return Ok(notes); // 返回备注内容
            }
        }

        Err(Error::new(ErrorKind::NotFound, "PPT 文件不存在"))
    }




    /// 读取文件的基本属性信息
    /// 
    /// ### 参数
    /// - `file_path:<AsRef<Path>>`: 文件的路径
    ///
    /// ### 返回值
    /// - `Result<Metadata>`: 返回文件的基本属性信息
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::get_file_metadata("src/test_examples/test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("文件基本属性信息: \n {:?}", content),
    ///   Err(e) => println!("读取文件失败: {}", e), 
    /// }
    #[inline]
    pub fn get_file_metadata<P: AsRef<Path>>(file_path: P) -> Result<Metadata> {
        fs::metadata(file_path)
    }

    /// 获取文件大小，单位字节
    /// 
    /// ### 参数
    /// - `file_path:<AsRef<Path>>`: 文件的路径
    ///
    /// ### 返回值
    /// - `Result<u64>`: 返回文件的大小
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::get_file_size("src/test_examples/test_txt.txt");
    /// match res {
    ///   Ok(size) => println!("文件大小: \n {}", size),
    ///   Err(e) => println!("读取文件失败: {}", e), 
    /// }
    pub fn get_file_size<P: AsRef<Path>>(file_path: P) -> Result<u64> {
        let metadata = Self::get_file_metadata(file_path)?;
        Ok(metadata.len())
    }

    /// 获取文件创建的时间
    /// 
    /// ### 参数
    /// - `file_path:<AsRef<Path>>`: 文件的路径
    ///
    /// ### 返回值
    /// - `Result<String>`: 返回文件的创建时间的字符串
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::get_file_created_time("src/test_examples/test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("文件创建时间: \n {}", content),
    ///   Err(e) => println!("读取文件失败: {}", e), 
    /// }
    pub fn get_file_created_time<P: AsRef<Path>>(file_path: P) -> Result<String> {
        let metadata = Self::get_file_metadata(file_path)?;
        let created_time = metadata.created()?;
        let datetime: DateTime<Local> = created_time.into();
        Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    /// 获取上一次访问的时间
    /// 
    /// ### 参数
    /// - `file_path:<AsRef<Path>>`: 文件的路径
    ///
    /// ### 返回值
    /// - `Result<String>`: 返回文件的上一次访问时间的字符串
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::get_last_accessed_time("src/test_examples/test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("文件的上一次访问时间: \n {}", content),
    ///   Err(e) => println!("读取文件失败: {}", e), 
    /// }
    pub fn get_last_accessed_time<P: AsRef<Path>>(file_path: P) -> Result<String> {
        let metadata = Self::get_file_metadata(file_path)?;
        let accessed_time = metadata.accessed()?;
        let datetime: DateTime<Local> = accessed_time.into();
        Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
    }


    /// 获取文件最后修改时间
    /// 
    /// ### 参数
    /// - `file_path:<AsRef<Path>>`: 文件的路径
    ///
    /// ### 返回值
    /// - `Result<String>`: 返回文件最后修改时间的字符串
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::get_last_modified("src/test_examples/test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("文件的最后修改时间: \n {}", content),
    ///   Err(e) => println!("读取文件失败: {}", e), 
    /// }
    pub fn get_last_modified<P: AsRef<Path>>(file_path: P) -> Result<String> {
        let metadata = Self::get_file_metadata(file_path)?;
        let modified_time = metadata.modified()?;
        let datetime: DateTime<Local> = modified_time.into();
        Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    /// 获取文件是否为目录
    /// 
    /// ### 参数
    /// - `file_path:<AsRef<Path>>`: 文件的路径
    ///
    /// ### 返回值
    /// - `Result<bool>`: 返回bool值,true表示是目录
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::is_directory("src/test_examples/test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("文件是否为目录: \n {}", content),
    ///   Err(e) => println!("读取文件失败: {}", e), 
    /// }
    pub fn is_directory<P: AsRef<Path>>(file_path: P) -> Result<bool> {
        let metadata = Self::get_file_metadata(file_path)?;
        Ok(metadata.is_dir())
    }

    /// 获取文件是否为普通文件
    /// 
    /// ### 参数
    /// - `file_path:<AsRef<Path>>`: 文件的路径
    ///
    /// ### 返回值
    /// - `Result<bool>`: 返回bool值,true表示是普通文件
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::is_regular_file("src/test_examples/test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("文件是否为普通文件: \n {}", content),
    ///   Err(e) => println!("读取文件失败: {}", e), 
    /// }
    pub fn is_regular_file<P: AsRef<Path>>(file_path: P) -> Result<bool> {
        let metadata = Self::get_file_metadata(file_path)?;
        Ok(metadata.is_file())
    }

    /// 解析文件权限位并返回一个表示权限的字符串。
    ///
    /// ### 参数
    /// - `attrs:<u32>`: 是一个 u32 类型的整数，表示文件的权限位。
    /// ### 返回值
    /// - `String`:返回一个字符串，表示文件的权限，格式为 "rwxrwxrwx"，其中每个字符代表读（r）、写（w）和执行（x）权限
    /// 
    /// 如果某个权限位被设置，则对应的字符为该权限的大写字母；否则为 '-'
    ///
    /// # 示例
    ///
    /// ```
    /// use FileContentReader::FileReader;
    /// let permissions = FileReader::parse_permissions(0o755);
    /// println!("文件权限: \n {}", permissions);
    /// ```
    pub fn parse_permissions(attrs: u32) -> String {
        let owner_read = if attrs & 0o400 != 0 { "r" } else { "-" };  // 0o 表示后面的数是8进制表示
        let owner_write = if attrs & 0o200 != 0 { "w" } else { "-" };
        let owner_exec = if attrs & 0o100 != 0 { "x" } else { "-" };

        let group_read = if attrs & 0o040 != 0 { "r" } else { "-" };
        let group_write = if attrs & 0o020 != 0 { "w" } else { "-" };
        let group_exec = if attrs & 0o010 != 0 { "x" } else { "-" };

        let other_read = if attrs & 0o004 != 0 { "r" } else { "-" };
        let other_write = if attrs & 0o002 != 0 { "w" } else { "-" };
        let other_exec = if attrs & 0o001 != 0 { "x" } else { "-" };

        format!(
            "{}{}{}-{}{}{}-{}{}{}",
            owner_read, owner_write, owner_exec,
            group_read, group_write, group_exec,
            other_read, other_write, other_exec
        )
    }
    /// 获取文件权限
    /// 
    /// ### 参数
    /// - `file_path:<AsRef<Path>>`: 文件的路径
    ///
    /// ### 返回值
    /// - `Result<String>`: 返回表示文件权限的字符串
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::get_file_permission("src/test_examples/test_txt.txt");
    /// match res {
    ///   Ok(content) => println!("文件权限: \n {}", content),
    ///   Err(e) => println!("读取文件失败: {}", e), 
    /// }
    pub fn get_file_permission<P: AsRef<Path>>(file_path: P) -> Result<String> {
        let metadata = Self::get_file_metadata(file_path)?;
        let permissions = metadata.permissions();
        match permissions.readonly() {
            true => Ok(String::from("r--")), // 这是是简化了的，windows获得文件执行权限很复杂
            false => Ok(String::from("rw-")),
        }
    }

    /// 获取文件扩展名
    /// 
    /// ### 参数
    /// - `file_path:<AsRef<Path>>`: 文件的路径
    ///
    /// ### 返回值
    /// - `String`: 返回文件扩展名
    /// # Examples
    /// 
    /// ```
    /// use FileContentReader::FileReader;
    /// let res = FileReader::get_file_extension("src/test_examples/test_txt.txt");
    /// println!("文件扩展名: \n {}", res);
    pub fn get_file_extension<P: AsRef<Path>>(file_path: P) -> String {
        file_path.as_ref().extension().and_then(|ext| ext.to_str()).unwrap_or("unknown").to_string()
    }

}



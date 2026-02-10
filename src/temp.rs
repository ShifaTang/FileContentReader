use FileContentReader::FileReader;
    /// 读取文本文件内容
    /// 
    /// ### 参数
    /// - `file_path:&str` ： 文件路径
    /// 
    /// ### 返回值
    /// -  `Result<String>`：返回字符串，字符串包含整个TXT文件的内容
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

    fn main() {
        pub fn read_txt(file_path: &str) -> Result<String> {
            // 判断文件路径是否存在
            assert!(fs::metadata(file_path).is_ok(), "文件不存在,请检查路径是否正确");
            // 判断文件类型是否正确
            assert!(file_path.ends_with(".txt"), "文件类型错误,请检查文件类型是否为txt文件");
            //打开文件
            let file = File::open(file_path)?;
            // 创建缓冲区
            let mut reader = BufReader::new(file); // 缓冲器
            let mut content = String::new();
            // 将文件内容读入字符串并返回
            reader.read_to_string(&mut content)?;
            Ok(content)
        }
    }

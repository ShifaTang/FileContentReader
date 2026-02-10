use FileContentReader::FileReader;

pub fn test_json() {
    /*
    返回类型：
    read_json_dynamic 函数返回一个 serde_json::Value 类型，这是一种灵活的数据结构，可以表示任何 JSON 数据。
    read_txt 函数返回一个 String 类型，它包含了文件的全部内容。
    解析方式：
    read_json_dynamic 使用 serde_json::from_reader 方法直接从 BufReader 中解析 JSON 数据为 serde_json::Value。
    read_txt 使用 read_to_string 方法将文件内容读取到一个字符串中，而不进行任何解析。
    用途：
    read_json_dynamic 更适合用于读取和解析 JSON 格式的文件。
    read_txt 适用于读取任何纯文本文件，包括 JSON 文件，但不会解析内容。
    处理结果：
    read_json_dynamic 返回的 serde_json::Value 可以方便地访问 JSON 数据的字段。
    read_txt 返回的 String 需要手动解析或处理才能访问 JSON 数据。
     */
    // 直接读取json文件内容,返回字符串
    let json = FileReader::read_json_text("src/test_examples/test_json.json").unwrap();
    println!("json is {}", json);

    // 动态读取json文件，返回Value，更加灵活
    let dynamic_json = FileReader::read_json_dynamic("src/test_examples/test_json.json").unwrap();
    println!("dynamic json is {}", dynamic_json);

    // //按块读取json文件
    let blocks = FileReader::read_json_by_block("src/test_examples/test_json.json", 1).unwrap();
    for (index,block) in blocks.iter().enumerate() {
        println!("Block {} is {}", index, block[0]);
    }

    // 按行读取jsonl文件  一般是日志文件
    let json_lines = FileReader::read_jsonl_by_line("src/test_examples/test_jsonline.jsonl").unwrap();
    for line in json_lines {
        println!("json line is {}", line);
    }

    // 读取文件元数据
    let metadata = FileReader::get_file_metadata("src/test_examples/test_json.json").unwrap();
    println!("metadata is {:?}", metadata);

    // 文件大小
    let file_len = FileReader::get_file_size("src/test_examples/test_json.json").unwrap();
    println!("file size is {}", file_len);

}
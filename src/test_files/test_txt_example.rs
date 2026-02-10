use FileContentReader::FileReader;

// 测试txt文件
pub fn test_txt() {
    // 一次性读取
    //let res = FileReader::read_txt("src/test_examples/test_md.md").unwrap();
    //println!("一次性读取文本文件: \n {}", res);

    // 按行读取
    let lines = FileReader::read_txt_by_line("src/test_examples/test_txt.txt").unwrap();
    println!("按行读取文本文件: \n {:?}", lines);

    // 按字节读取
    let bytes = FileReader::read_txt_by_byte("src/test_examples/test_txt.txt").unwrap();
    println!("按字节读取文本文件: \n {:?}", bytes); // 一个一个的字节

    // 按块读取
    let blocks = FileReader::read_txt_by_block("src/test_examples/test_txt.txt", 4).unwrap();
    println!("按块读取文本文件: \n {:?}", blocks); // 按块读取，每块4个字节
    
    // 文件属性
    let file_metadata = FileReader::get_file_metadata("src/test_examples/test_txt.txt");
    println!("文件元数据: \n {:?}", file_metadata);

    let file_len = FileReader::get_file_size("src/test_examples/test_txt.txt");
    println!("文件大小: {} 字节", file_len.unwrap()); // 如果成功，返回文件大小，否则抛出一个错误

    let file_modified = FileReader::get_last_modified("src/test_examples/test_txt.txt");
    println!("文件最后修改时间: {:?}", file_modified.unwrap()); // 如果成功，返回文件最后修改时间，否则抛出一个错误

    let isDirectory = FileReader::is_directory("src/test_examples/test_txt.txt");
    println!("是否为目录: {}", isDirectory.unwrap()); // 如果成功，返回true或false，否则抛出一个错误

    let isOrdinary = FileReader::is_regular_file("src/test_examples/test_txt.txt");
    println!("是否为普通文件: {}", isOrdinary.unwrap()); // 如果成功，返回true或false，否则抛出一个错误

    let file_permission = FileReader::get_file_permission("src/test_examples/test_txt.txt");
    println!("文件权限: {:?}", file_permission.unwrap()); // 如果成功，返回文件权限，否则抛出一个错误

    let file_extension = FileReader::get_file_extension("src/test_examples/test_txt.txt");
    println!("文件扩展名: {:?}", file_extension); // 如果成功，返回文件扩展名，否则抛出一个错误

    let created_time = FileReader::get_file_created_time("src/test_examples/test_txt.txt");
    println!("文件创建时间: {:?}", created_time.unwrap()); // 如果成功，返回文件创建时间，否则抛出一个错误

    let accessed_time = FileReader::get_last_accessed_time("src/test_examples/test_txt.txt");
    println!("文件最后一次访问时间: {:?}", accessed_time.unwrap()); // 如果成功，返回文件访问时间，否则抛出一个错误

}
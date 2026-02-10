use FileContentReader::FileReader;

pub fn test_csv() {
    // 按行读取csv文件
    let lines = FileReader::read_csv_by_line("src/test_examples/test_csv.csv");
    println!("lines: {:?}", lines);

    // 按列读取csv文件
    let columns = FileReader::read_csv_by_column("src/test_examples/test_csv.csv");
    println!("columns: {:?}", columns);

    // 按块读取csv文件，实际就是读取指定行数
    let blocks = FileReader::read_csv_by_block("src/test_examples/test_csv.csv", 2);
    println!("blocks: {:?}", blocks);

    // 按字节读取就没写了，不适合

    // 读取元数据
    let metadata = FileReader::get_file_metadata("src/test_examples/test_csv.csv");
    println!("metadata: {:?}", metadata);

    // 文件大小
    let file_len = FileReader::get_file_size("src/test_examples/test_csv.csv");
    println!("file_len: {:?}", file_len);
}


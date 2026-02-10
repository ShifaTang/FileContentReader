use FileContentReader::FileReader;

pub fn test_excel() {
    // 读取整个excel文件作为字符串返回，这个函数适合用来显示excel
    let excel_string = FileReader::read_excel_as_string("src/test_examples/test_excel.xlsx");
    println!("Excel xlsx String: {}" , excel_string.unwrap());

    let excel_string = FileReader::read_excel_as_string("src/test_examples/test_excel2.xls");
    println!("Excel xls String: {}" , excel_string.unwrap());

    // 读取excel文件sheet作为字符串返回，这个函数适合用来显示excel的每一个sheet
    let excel_string = FileReader::read_excel_as_string_by_sheet("src/test_examples/test_excel.xlsx");
    for sheet in excel_string.unwrap() {
        println!("sheet xlsx is {}", sheet);
    }

    let excel_string = FileReader::read_excel_as_string_by_sheet("src/test_examples/test_excel2.xls");
    for sheet in excel_string.unwrap() {
        println!("sheet xls is {}", sheet);
    }

    // 按行读取excel文件
    let excel_sheets = FileReader::read_excel_by_row("src/test_examples/test_excel2.xls");
    for sheet in excel_sheets.unwrap() {
        println!("--------------");
        for row in sheet {
            println!("row is {} ", row)
        }
    }

    // 按列读取excel文件
    let excel_sheets = FileReader::read_excel_by_column("src/test_examples/test_excel2.xls");
    for sheet in excel_sheets.unwrap() {
        println!("--------------");
        for col in sheet {
            println!("col is {} ", col)
        }
    }

    // 按块读取excel文件，读取块的大小 (rows, cols)
    let excel_block_string = FileReader::read_excel_by_block_as_string("src/test_examples/test_excel2.xls", (2,2));
    print!("excel block string is\n{}", excel_block_string.unwrap());

    // 按块读取excel文件，读取块的大小 (rows, cols),返回向量
    let excel_block = FileReader::read_excel_by_block("src/test_examples/test_excel2.xls", (2,2));
    for block in excel_block.unwrap() {
        println!("block is\n{}", block);
    }

    // 获取文件属性
    let metadata = FileReader::get_file_metadata("src/test_examples/test_excel2.xls");
    println!("last accessed time is {}", FileReader::get_last_accessed_time("src/test_examples/test_excel2.xls").unwrap());
}
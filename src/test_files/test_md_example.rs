use FileContentReader::FileReader;

pub fn test_md() {
    // 一次性读取markdown文件
    let md_content = FileReader::read_markdown("src/test_examples/test_md.md");
    println!("content is \n{}", md_content.unwrap());

    // 按行读取markdown文件
    let lines = FileReader::read_md_by_line("src/test_examples/test_md.md");
    println!("按行读取数据");
    match lines {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }

    // 字节读取数据
    let bytes = FileReader::read_md_by_byte("src/test_examples/test_md.md");
    println!("字节读取数据");
    match bytes {
        Ok(bytes) => {
            for byte in bytes {
                println!("{}", byte);
            }
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }

    // 按块读取
    let blocks = FileReader::read_md_by_block("src/test_examples/test_md.md", 3);
    println!("按块读取数据");
    match blocks {
        Ok(blocks) => {
            for block in blocks {
                print!("{}", block);
            }
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }

    // 获取文件元数据
    let metadata = FileReader::get_file_metadata("src/test_examples/test_md.md");
    println!("文件元数据");
    match metadata {
        Ok(metadata) => {
            println!("文件大小: {}", metadata.len());
        },
        Err(e) => {
            println!("error: {}", e);
        }
    }

    // 将markdown转换成html,将对应的标题转成对应标签
    let html_content = FileReader::read_markdown_and_convert_to_html("src/test_examples/test_md.md");
    println!("html content is \n{}", html_content.unwrap());


}
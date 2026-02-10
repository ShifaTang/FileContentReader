use FileContentReader::FileReader;

pub fn test_xml() {
    // 直接读取xml文本
    let xml_text = FileReader::read_xml_text("src/test_examples/test_xml.xml");
    println!("XML Text: {}", xml_text.unwrap());

    // 读取xml文件标签
    let mut start_tags:Vec<String> = Vec::new();
    let mut end_tags:Vec<String> = Vec::new();
    let mut text:Vec<String> = Vec::new();
    // 处理标签的闭包
    let on_start = |tag_name:&str|{
        start_tags.push(tag_name.to_string());
    };
    let on_end = |tag_name:&str| {
        end_tags.push(tag_name.to_string());
    };
    let on_text = |text_content:&str| {
        text.push(text_content.to_string());
    };

    let res = FileReader::read_xml_by_listener("src/test_examples/test_xml.xml", on_start, on_text, on_end);
    match res {
        Ok(()) => {
            println!("Start tags:");
            for start in start_tags {
                println!("  {}", start);
            }

            println!("Text nodes:");
            for text in text {
                println!("  {}", text);
            }

            println!("end tags:");
            for end in end_tags {
                println!("  {}", end);
            }
        },
        Err(e) => eprintln!("Error processing XML: {}", e),
    }

    // 读取xml文件元数据
    let metadata = FileReader::get_file_metadata("src/test_examples/test_xml.xml");
    println!("File metadata: {:?}", metadata);


    // 读取xml文件大小
    let file_size = FileReader::get_file_size("src/test_examples/test_xml.xml");
    println!("File size: {:?}", file_size);

} 
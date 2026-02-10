use FileContentReader::FileReader; // Ensure this is included



pub fn test_rar() {
    // 读取第一个文件内容
    // let data = FileReader::read_rar_first_file("src/test_examples/test_rar.rar", "test_txt.txt").unwrap();



    // 读取rar压缩包目录结构
    let files = FileReader::list_rar_filenames("src/test_examples/test_rar.rar", true);
    println!("----------------读取rar压缩包目录结构-------------------");
    for file in files.unwrap() {
        println!("File name is : {}", file);
    }



    let data = FileReader::read_rar_txt("src/test_examples/test_rar.rar", "test_rar\\test_txt.txt").unwrap();
    println!("----------------读取rar压缩包内的txt文件内容-------------------");
    println!("TXT content: \n{}", data);




    let lines = FileReader::read_rar_txt_by_line("src/test_examples/test_rar.rar", "test_rar\\test_txt.txt").unwrap();
    println!("----------------按行读取rar文件中txt文件内容-");
    for line in lines {
        println!("row -- > {}", line);
    }



    let bytes = FileReader::read_rar_txt_by_byte("src/test_examples/test_rar.rar", "test_rar\\test_txt.txt").unwrap();
    println!("----------------按字节读取rar文件中txt文件内容-");
    for byte in bytes {
        print!("{:02x} ", byte);
    }
    println!("");

    

    let blocks = FileReader::read_rar_txt_by_block("src/test_examples/test_rar.rar", "test_rar\\test_txt.txt", 1024).unwrap();
    println!("----------------按块读取rar文件中txt文件内容-");
    for block in blocks {
        println!("block -- > {}", block);
    }


    

    let data = FileReader::read_rar_csv("src/test_examples/test_rar.rar", "test_rar\\test_csv.csv").unwrap();
    println!("-------------读取rar压缩包中csv文件内容-------------");
    for row in data {
        println!("{:?}", row);
    }

    

    let data = FileReader::read_rar_csv_by_column("src/test_examples/test_rar.rar", "test_rar\\test_csv.csv").unwrap();
    println!("-------------按列读取csv文件内容-");
    for column in data {
        println!("{:?}", column);
    }


    

    let data = FileReader::read_rar_csv_by_block("src/test_examples/test_rar.rar", "test_rar\\test_csv.csv", 10).unwrap();
    println!("-------------按块读取rar中csv文件内容-");
    for block in data {
        for row in block {
            println!("{:?}", row);
        }
    }


    

    let data = FileReader::read_rar_json("src/test_examples/test_rar.rar", "test_rar\\test_json.json").unwrap();
    println!("-------------读取rar中json文件内容-");
    println!("JSON content: \n{}", data);


    
    let data = FileReader::read_rar_json_by_block("src/test_examples/test_rar.rar", "test_rar\\test_json.json", 10).unwrap();
    println!("-------------按块读取json文件内容-");
    for block in data {
        for json in block {
            println!("JSON content: \n{}", json);
        }
    }

    

    let data = FileReader::read_rar_jsonl("src/test_examples/test_rar.rar", "test_rar\\test_jsonline.jsonl").unwrap();
    println!("-------------读取rar中jsonl文件内容-");
    for json in data {
        println!("JSON content: \n{}", json);
    }

    
    let data = FileReader::read_rar_xml_text("src/test_examples/test_rar.rar", "test_rar\\test_xml.xml").unwrap();
    println!("-------------读取rar中xml文件内容-");
    println!("XML content: \n{}", data);


    

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

    let res = FileReader::read_rar_xml("src/test_examples/test_rar.rar","test_rar\\test_xml.xml", on_start, on_text, on_end);
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



    

    let data = FileReader::read_rar_markdown("src/test_examples/test_rar.rar", "test_rar\\test_md.md").unwrap();
    println!("-------------读取rar中markdown文件内容-");
    println!("Markdown content: \n{}", data);



    

    let data = FileReader::read_rar_md_by_line("src/test_examples/test_rar.rar", "test_rar\\test_md.md").unwrap();
    println!("-------------按行读取rar中markdown文件内容-");
    for line in data {
        println!("{}", line);
    }




    let data = FileReader::read_rar_md_by_byte("src/test_examples/test_rar.rar", "test_rar\\test_md.md").unwrap();
    println!("-------------按字节读取rar中markdown文件内容-");
    for byte in data {
        print!("{}", byte as char);
    }


    

    let data = FileReader::read_rar_md_by_block("src/test_examples/test_rar.rar", "test_rar\\test_md.md", 10).unwrap();
    println!("-------------按块读取rar中markdown文件内容-");
    for block in data {
        println!("{}", block);
    }


    

    let data = FileReader::read_rar_excel("src/test_examples/test_rar.rar", "test_rar\\test_excel.xlsx").unwrap();
    for row in data {
        println!("{:?}", row);
    }

    

    let data = FileReader::read_rar_pdf_as_string("src/test_examples/test_rar.rar", "test_rar\\test_pdf.pdf").unwrap();
    println!("-------------读取rar中pdf文件内容-");
    println!("PDF content: \n{}", data);


    
    let data = FileReader::read_rar_pptx_text("src/test_examples/test_rar.rar", "test_rar\\test_ppt.pptx").unwrap();
    println!("-------------读取rar中pptx文件内容-");
    println!("PPTX content: \n{}", data);



    

    let data = FileReader::read_rar_pptx_text_by_slide("src/test_examples/test_rar.rar", "test_rar\\test_ppt.pptx").unwrap();
    for slide in data {
        println!("{:?}", slide);
    }


    

    let data = FileReader::read_ppt_notes_from_rar("src/test_examples/test_rar.rar", "test_rar\\test_ppt.pptx").unwrap();
    for note in data {
        println!("Note content: {}", note);
    }

}

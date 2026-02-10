use FileContentReader::FileReader;


pub fn test_tar() {


    let file_list = FileReader::list_tar_filenames("src/test_examples/test_tar.tar").unwrap();
    println!("---------列出文件目录---------");
    for file_name in file_list {
        println!("{}", file_name);
    }




    let line = FileReader::read_tar_text("src/test_examples/test_tar.tar", "test_txt.txt");
    println!("---------读取成字符串---------");
    println!("\n{}",line.unwrap());



    let line = FileReader::read_tar_text_by_line("src/test_examples/test_tar.tar", "test_txt.txt");
    println!("---------按行读取字符串---------");
    for line in line.unwrap() {
        println!("by line {}", line);
    }




    let bytes = FileReader::read_tar_txt_by_byte("src/test_examples/test_tar.tar", "test_txt.txt").unwrap();
    println!("---------按字节读取文本文件内容---------");
    println!("{:?}", bytes);



    let blocks = FileReader::read_tar_txt_by_block("src/test_examples/test_tar.tar", "test_txt.txt", 1024).unwrap();
    println!("---------按块读取文本文件内容---------");
    for block in blocks {
        println!("-- {}", block);
    }




    let records = FileReader::read_tar_csv_by_line("src/test_examples/test_tar.tar", "test_csv.csv");
    println!("---------按行读取csv文件---------");
    for record in records.unwrap() {
        println!("{:?}", record);
    }




    let columns = FileReader::read_tar_csv_by_column("src/test_examples/test_tar.tar", "test_csv.csv").unwrap();
    println!("---------按列读取csv文件---------");
    for column in columns {
        println!("{:?}", column);
    }



    let blocks = FileReader::read_tar_csv_by_block("src/test_examples/test_tar.tar", "test_csv.csv", 2).unwrap();
    println!("---------按块读取csv文件---------");
    for block in blocks {
        for record in block {
            println!("{:?}", record);
        }
    }




    let json_value = FileReader::read_tar_json("src/test_examples/test_tar.tar", "test_json.json");
    println!("---------读取json文件内容---------");
    println!("{:?}", json_value.unwrap());



    let json_values = FileReader::read_tar_jsonl_by_line("src/test_examples/test_tar.tar", "test_jsonline.jsonl");
    println!("---------按行读取jsonl文件内容---------");
    for json_value in json_values.unwrap() {
        println!("{:?}", json_value);
    }




    let xml_text = FileReader::read_tar_xml_text("src/test_examples/test_tar.tar", "test_xml.xml").unwrap();
    println!("---------读取xml文件内容---------");
    println!("{}", xml_text);


    
    

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

    let res = FileReader::read_tar_xml_by_listener("src/test_examples/test_tar.tar","test_xml.xml" ,on_start, on_text, on_end);
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

    

    let md_text = FileReader::read_tar_markdown("src/test_examples/test_tar.tar", "test_md.md").unwrap();
    println!("---------读取md文件内容---------");
    println!("{}", md_text);



    
    let lines = FileReader::read_tar_md_by_line("src/test_examples/test_tar.tar", "test_md.md").unwrap();
    println!("---------按行读取md文件内容---------");
    for line in lines {
        println!("{}", line);
    }

    

    let bytes = FileReader::read_tar_md_by_byte("src/test_examples/test_tar.tar", "test_md.md").unwrap();
    println!("---------按字节读取md文件内容---------");
    for byte in bytes { 
        print!("{} ", byte); 
    }
    println!();

    

    let blocks = FileReader::read_tar_md_by_block("src/test_examples/test_tar.tar", "test_md.md", 1024).unwrap();
    println!("---------按块读取md文件内容---------");
    for block in blocks {
        println!("{}", block);
    }



   

    let lines = FileReader::read_tar_excel("src/test_examples/test_tar.tar", "test_excel.xlsx").unwrap();
    println!("---------读取整个excel文件内容---------");
    for line in lines {
        println!("{}", line);
    }

    
    let pdf_text = FileReader::read_tar_pdf_as_string("src/test_examples/test_tar.tar", "test_pdf.pdf").unwrap();
    println!("---------读取pdf文件内容---------");
    println!("{}", pdf_text);


    
    
    let slide_texts = FileReader::read_tar_pptx_text_by_slide("src/test_examples/test_tar.tar", "test_ppt.pptx").unwrap();
    println!("---------按行读取ppt文件内容---------");
    for slide_text in slide_texts {
        println!("{}", slide_text);
    }


    
    let notes = FileReader::read_tar_pptx_notes("src/test_examples/test_tar.tar", "test_ppt.pptx").unwrap();
    println!("---------读取ppt备注信息---------");
    for note in notes {
        println!("{}", note);
    }


}

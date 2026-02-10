use FileContentReader::FileReader;


pub fn test_zip() {

    // 读取zip文件目录
    let rar_file_names = FileReader::list_zip_filenames("src/test_examples/test_zip.zip", false);
    for file_name in rar_file_names.unwrap() {
        println!("File name: {}", file_name);
    }

    // // 读取zip文件指定路径文件
    // let zip_file = FileReader::read_zip_txt("src/test_examples/test_zip.zip", "test_rar/test_txt.txt");
    // println!("File content: \n{}", zip_file.unwrap());

    // // 读取zip文件csv文件
    // let zip_csv = FileReader::read_zip_csv("src/test_examples/test_zip.zip", "test_rar/test_csv.csv");
    // for lines in zip_csv.unwrap() {
    //     println!("line is {:?}", lines);
    // }
    
    // // 读取zip文件内jsonl文件
    // let zip_jsonl = FileReader::read_zip_jsonl("src/test_examples/test_zip.zip", "test_rar/test_jsonline.jsonl");
    // println!("zip json content: \n{:?}", zip_jsonl.unwrap());

    // // 读取zip文件内json文件
    // let zip_json = FileReader::read_zip_json("src/test_examples/test_zip.zip", "test_rar/test_json.json");
    // println!("zip json content: \n{:?}", zip_json.unwrap());

    // // 读取zip文件内xml文件
    // // 读取xml文件标签
    // let mut start_tags:Vec<String> = Vec::new();
    // let mut end_tags:Vec<String> = Vec::new();
    // let mut text:Vec<String> = Vec::new();
    // // 处理标签的闭包
    // let on_start = |tag_name:&str|{
    //     start_tags.push(tag_name.to_string());
    // };
    // let on_end = |tag_name:&str| {
    //     end_tags.push(tag_name.to_string());
    // };
    // let on_text = |text_content:&str| {
    //     text.push(text_content.to_string());
    // };

    // let res = FileReader::read_zip_xml("src/test_examples/test_zip.zip", "test_rar/test_xml.xml", on_start, on_text, on_end);
    // match res {
    //     Ok(()) => {
    //         println!("Start tags:");
    //         for start in start_tags {
    //             println!("  {}", start);
    //         }

    //         println!("Text nodes:");
    //         for text in text {
    //             println!("  {}", text);
    //         }

    //         println!("end tags:");
    //         for end in end_tags {
    //             println!("  {}", end);
    //         }
    //     },
    //     Err(e) => eprintln!("Error processing XML: {}", e),
    // }


    // // 读取zip文件内文件内容
    // let zip_md = FileReader::read_zip_md("src/test_examples/test_zip.zip", "test_rar/test_md.md");
    // println!("zip md content: \n{:?}", zip_md.unwrap());

    // // 读取zip文件中excel文件内容
    // let zip_excel = FileReader::read_zip_excel("src/test_examples/test_zip.zip", "test_rar/test_excel.xlsx");
    // println!("zip excel content: \n{:?}", zip_excel.unwrap());

    // // 读取zip文件中ppt文件内容
    // let zip_ppt = FileReader::read_zip_pptx_text("src/test_examples/test_zip.zip", "test_rar/test_ppt.pptx");
    // println!("zip ppt content: \n{:?}", zip_ppt.unwrap());



    // // 按行读取zip文件txt文件内容
    // let lines = FileReader::read_zip_txt_by_line("src/test_examples/test_zip.zip", "test_rar/test_txt.txt");
    // println!("zip txt content: \n{:?}", lines.unwrap());

    // // 按字节读取zip文件txt文件内容
    // let bytes = FileReader::read_zip_txt_by_byte("src/test_examples/test_zip.zip", "test_rar/test_txt.txt");
    // println!("zip txt content: \n{:?}", bytes.unwrap());

    // // 按块读取zip文件txt文件内容
    // let blocks = FileReader::read_zip_txt_by_block("src/test_examples/test_zip.zip", "test_rar/test_txt.txt",4);
    // println!("zip txt content: \n{:?}", blocks.unwrap());

    // // 按列读取zipcsv文件内容
    // let columns = FileReader::read_zip_csv_by_column("src/test_examples/test_zip.zip", "test_rar/test_csv.csv");
    // println!("zip csv content: \n{:?}", columns.unwrap());

    // // 按行读取zipcsv文件内容
    // let rows = FileReader::read_zip_csv("src/test_examples/test_zip.zip", "test_rar/test_csv.csv");
    // println!("zip csv content: \n{:?}", rows.unwrap());

    // // 按块读取zipcsv文件内容
    // let blocks = FileReader::read_zip_csv_by_block("src/test_examples/test_zip.zip", "test_rar/test_csv.csv",4);
    // println!("zip csv content: \n{:?}", blocks.unwrap());

    // // 按块读取json文件内容
    // let blocks = FileReader::read_zip_json_by_block("src/test_examples/test_zip.zip", "test_rar/test_json.json",4);
    // println!("zip json content: \n{:?}", blocks.unwrap());

    // // 直接读取zip xml文件内容
    // let xml = FileReader::read_zip_xml_text("src/test_examples/test_zip.zip", "test_rar/test_xml.xml");
    // println!("zip xml content: \n{:?}", xml.unwrap());

    // // 直接读取zip md文件内容
    // let md = FileReader::read_zip_markdown("src/test_examples/test_zip.zip", "test_rar/test_md.md");
    // println!("zip md content: \n{:?}", md.unwrap());

    // // 按字节读取zip md文件内容
    // let bytes = FileReader::read_zip_md_by_byte("src/test_examples/test_zip.zip", "test_rar/test_md.md");
    // println!("zip md content: \n{:?}", bytes.unwrap());

    // // 按块读取zip md文件内容
    // let blocks = FileReader::read_zip_md_by_block("src/test_examples/test_zip.zip", "test_rar/test_md.md",4);
    // println!("zip md content: \n{:?}", blocks.unwrap());

    // 读取zip中pdf文件内容
    let pdf = FileReader::read_zip_pdf_as_string("src/test_examples/test_zip.zip", "test_rar/test_pdf.pdf");
    println!("zip pdf context {}", pdf.unwrap());

    // 读取zip中ppt文件内容
    let ppt = FileReader::read_zip_pptx_text("src/test_examples/test_zip.zip", "test_rar/test_ppt.pptx");
    println!("zip ppt context {}", ppt.unwrap());

    // 读取zip中ppt文件内容，返回向量
    let ppt_vec = FileReader::read_zip_pptx_text_by_slide("src/test_examples/test_zip.zip", "test_rar/test_ppt.pptx");
    println!("ppt vec: \n{:?}", ppt_vec.unwrap());

    // 读取zip中ppt备注内容
    let ppt_notes = FileReader::read_zip_pptx_notes("src/test_examples/test_zip.zip", "test_rar/test_ppt.pptx");
    println!("ppt notes: \n{:?}", ppt_notes.unwrap());


}
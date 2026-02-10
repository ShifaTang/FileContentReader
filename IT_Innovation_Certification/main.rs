use attribute::{get_last_accessed_time, get_file_created_time, get_file_extension, get_file_permission, get_file_size, get_last_modified, is_directory, is_regular_file, read_file_metadata};
use FileContentReader::FileReader;
mod attribute;
mod test_files;
use test_files::test_txt_example::test_txt;
use test_files::test_csv_example::test_csv;
use test_files::test_json_example::test_json;
use test_files::test_xml_example::test_xml;
use test_files::test_md_example::test_md;
use test_files::test_pdf_example::test_pdf;
use test_files::test_excel_example::test_excel;
use test_files::test_ppt_example::test_ppt;
use test_files::test_zip_example::test_zip;
use test_files::test_rar_example::test_rar;
use test_files::test_tar_example::test_tar;
use std::io::{self, Write}; 
use std::thread;
use std::time::Duration;

fn main(){

    // 创建一个包含标准输出的句柄
    let stdout = io::stdout();
    // 获取一个可以用来写入的锁，因为标准输出是可缓冲的
    let mut handle = stdout.lock();
    // 打印提示信息，并刷新标准输出以确保提示信息显示出来
    handle.flush().unwrap();
    // 创建一个字符串缓冲区来存储输入的数据
    let mut input = String::new();
    // 使用标准输入读取一行数据到input变量中
    println!("请输入要调用的函数:");
    io::stdin().read_line(&mut input).expect("无法读取输入");
    input = input.trim().to_string();

    println!("请输入文件路径:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("无法读取输入");
    input1 = input1.trim().to_string();


//D:/examples/test_txt.txt

    match input.as_str(){
        "read_txt"=>{
            let res = FileReader::read_txt(&input1).unwrap();
            println!("一次性读取文本文件: \n {}", res);
        }
        "read_txt_by_line"=>{
            let lines = FileReader::read_txt_by_line(&input1).unwrap();
            println!("按行读取文本文件: \n {:?}", lines);
        }
        "read_txt_by_byte"=>{
            let bytes = FileReader::read_txt_by_byte(&input1).unwrap();
            println!("按字节读取文本文件: \n {:?}", bytes);
        }
        "read_txt_by_block"=>{
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let blocks = FileReader::read_txt_by_block(&input1, number as usize).unwrap();
                    println!("按块读取文本文件: \n {:?}", blocks);
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_csv_by_line"=>{
            let lines = FileReader::read_csv_by_line(&input1);
            println!("lines: {:?}", lines);
        }
        "read_csv_by_column"=>{
            let columns = FileReader::read_csv_by_column(&input1);
            println!("columns: {:?}", columns);
        }
        "read_csv_by_block"=>{
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let blocks = FileReader::read_csv_by_block(&input1, number as usize);
                    println!("blocks: {:?}", blocks);
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_json_text"=>{
            let json = FileReader::read_json_text(&input1).unwrap();
            println!("json is {}", json);
        }
        "read_json_dynamic"=>{
            let dynamic_json = FileReader::read_json_dynamic(&input1).unwrap();
            println!("dynamic json is {}", dynamic_json);
        }
        "read_jsonl_by_line"=>{
            let json_lines = FileReader::read_jsonl_by_line(&input1).unwrap();
            for line in json_lines {
                println!("json line is {}", line);
            }
        }
        "read_json_by_block"=>{
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let blocks = FileReader::read_json_by_block(&input1, number as usize).unwrap();
                    for (index,block) in blocks.iter().enumerate() {
                        println!("Block {} is {}", index, block[0]);
                    }
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_xml_text"=>{
            let xml_text = FileReader::read_xml_text(&input1);
            println!("XML Text: {}", xml_text.unwrap());
        }
        "read_xml_by_listener"=>{
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

            let res = FileReader::read_xml_by_listener(&input1, on_start, on_text, on_end);
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
        }
        "read_markdown"=>{
            let md_content = FileReader::read_markdown(&input1);
            println!("content is \n{}", md_content.unwrap());
        }
        "read_md_by_line"=>{
            let lines = FileReader::read_md_by_line(&input1);
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
        }
        "read_md_by_byte"=>{
            let bytes = FileReader::read_md_by_byte(&input1);
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
        }
        "read_md_by_block"=>{
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let blocks = FileReader::read_md_by_block(&input1, number as usize);
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
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_markdown_and_convert_to_html"=>{
            let html_content = FileReader::read_markdown_and_convert_to_html(&input1);
            println!("html content is \n{}", html_content.unwrap());
        }
        "read_pdf_as_string"=>{
            let pdf_content = FileReader::read_pdf_as_string(&input1).unwrap();
            println!("{}", pdf_content);
        }
        "read_excel_as_string"=>{
            let excel_string = FileReader::read_excel_as_string(&input1);
            println!("Excel xlsx String: {}" , excel_string.unwrap());
        }
        "read_excel_as_string_by_sheet"=>{
            let excel_string = FileReader::read_excel_as_string_by_sheet(&input1);
            for sheet in excel_string.unwrap() {
                println!("sheet xlsx is {}", sheet);
            }
        }
        "read_excel_by_column_as_string"=>{
            let excel_block_string = FileReader::read_excel_by_column_as_string(&input1);
            print!("excel column string is\n{}", excel_block_string.unwrap());
        }
        "read_excel_by_row"=>{
            let excel_sheets = FileReader::read_excel_by_row(&input1);
            for sheet in excel_sheets.unwrap() {
                println!("--------------");
                for row in sheet {
                    println!("row is {} ", row)
                }
            }
        }
        "read_excel_by_column"=>{
            let excel_sheets = FileReader::read_excel_by_column(&input1);
            for sheet in excel_sheets.unwrap() {
                println!("--------------");
                for col in sheet {
                    println!("col is {} ", col)
                }
            }
        }
        "read_excel_by_block_as_string"=>{
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let excel_block_string = FileReader::read_excel_by_block_as_string(&input1, (number as usize,number as usize));
                    print!("excel block string is\n{}", excel_block_string.unwrap());
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_excel_by_block"=>{
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let excel_block = FileReader::read_excel_by_block(&input1, (number as usize,number as usize));
                    for block in excel_block.unwrap() {
                        println!("block is\n{}", block);
                    }
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
            
        }
        "read_pptx_text"=>{
            let ppt_text = FileReader::read_pptx_text(&input1);
            println!("ppt text is\n{}", ppt_text.unwrap());
        }
        "read_pptx_text_by_slide"=>{
            let ppt_text = FileReader::read_pptx_text_by_slide(&input1);
            for ppt in ppt_text.unwrap() {
                println!("ppt slide text is\n{}", ppt);
            }
        }
        "read_pptx_notes"=>{
            let ppt_notes = FileReader::read_pptx_notes(&input1);
            for note in ppt_notes.unwrap() {
                println!("ppt note is \n{}", note);
            }
        }
        "list_zip_filenames"=>{
            println!("请输入是否包含压缩包里的文件目录:true or false");
            let mut flag = String::new();
            io::stdin().read_line(&mut flag).expect("无法读取输入");
            flag = flag.trim().to_string();
            match flag.parse::<bool>() {
                Ok(boolean_value) =>{
                    let rar_file_names = FileReader::list_zip_filenames(&input1, boolean_value);
                    for file_name in rar_file_names.unwrap() {
                        println!("File name: {}", file_name);
                    }
                }
                Err(_) => println!("输入无效，请输入 'true' 或 'false'"),
            }
        }
        "read_zip_txt"=>{
            //test_rar/test_txt.txt
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let zip_file = FileReader::read_zip_txt(&input1, &input2);
            println!("File content: \n{}", zip_file.unwrap());
        }
        "read_zip_txt_by_line"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let lines = FileReader::read_zip_txt_by_line(&input1, &input2);
            println!("zip txt content: \n{:?}", lines.unwrap());
        }
        "read_zip_txt_by_byte"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let bytes = FileReader::read_zip_txt_by_byte(&input1, &input2);
            println!("zip txt content: \n{:?}", bytes.unwrap());
        }
        "read_zip_txt_by_block"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let blocks = FileReader::read_zip_txt_by_block(&input1, &input2,number as usize);
                    println!("zip txt content: \n{:?}", blocks.unwrap());
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }

        }
        "read_zip_csv"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let zip_csv = FileReader::read_zip_csv(&input1, &input2);
            for lines in zip_csv.unwrap() {
                println!("line is {:?}", lines);
            }
        }
        "read_zip_csv_by_column"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let columns = FileReader::read_zip_csv_by_column(&input1, &input2);
            println!("zip csv content: \n{:?}", columns.unwrap());
        }
        "read_zip_csv_by_block"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let blocks = FileReader::read_zip_csv_by_block(&input1, &input2,number as usize);
                    println!("zip csv content: \n{:?}", blocks.unwrap());
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_zip_json"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let zip_json = FileReader::read_zip_json(&input1, &input2);
            println!("zip json content: \n{:?}", zip_json.unwrap());
        }
        "read_zip_json_by_block"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let blocks = FileReader::read_zip_json_by_block(&input1, &input2,number as usize);
                    println!("zip json content: \n{:?}", blocks.unwrap());
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_zip_jsonl"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let zip_jsonl = FileReader::read_zip_jsonl(&input1, &input2);
            println!("zip json content: \n{:?}", zip_jsonl.unwrap());
        }
        "read_zip_xml_text"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let xml = FileReader::read_zip_xml_text(&input1, &input2);
            println!("zip xml content: \n{:?}", xml.unwrap());
        }
        "read_zip_xml"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
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
        
            let res = FileReader::read_zip_xml(&input1, &input2, on_start, on_text, on_end);
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
        }
        "read_zip_markdown"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let zip_md = FileReader::read_zip_markdown(&input1, &input2);
            println!("zip md content: \n{:?}", zip_md.unwrap());
        }
        "read_zip_md_by_line"=>{
            
        }
        "read_zip_md_by_byte"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let bytes = FileReader::read_zip_md_by_byte(&input1, &input2);
            println!("zip md content: \n{:?}", bytes.unwrap());
        }
        "read_zip_md_by_block"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let blocks = FileReader::read_zip_md_by_block(&input1, &input2,number as usize);
                    println!("zip md content: \n{:?}", blocks.unwrap());
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_zip_excel"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let zip_excel = FileReader::read_zip_excel(&input1, &input2);
            println!("zip excel content: \n{:?}", zip_excel.unwrap());
        }
        "read_zip_pdf_as_string"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let pdf = FileReader::read_zip_pdf_as_string(&input1, &input2);
            println!("zip pdf context {}", pdf.unwrap());
        }
        "read_zip_pptx_text"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let zip_ppt = FileReader::read_zip_pptx_text(&input1, &input2);
            println!("zip ppt content: \n{:?}", zip_ppt.unwrap());
        }
        "read_zip_pptx_text_by_slide"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let ppt_vec = FileReader::read_zip_pptx_text_by_slide(&input1, &input2);
            println!("ppt vec: \n{:?}", ppt_vec.unwrap());
        }
        "read_zip_pptx_notes"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let ppt_notes = FileReader::read_zip_pptx_notes(&input1, &input2);
            println!("ppt notes: \n{:?}", ppt_notes.unwrap());
        }
        "list_rar_filenames"=>{
            println!("请输入是否包含压缩包里的文件目录:true or false");
            let mut flag = String::new();
            io::stdin().read_line(&mut flag).expect("无法读取输入");
            flag = flag.trim().to_string();
            match flag.parse::<bool>() {
                Ok(boolean_value) =>{
                    let files = FileReader::list_rar_filenames(&input1, boolean_value);
                    println!("----------------读取rar压缩包目录结构-------------------");
                    for file in files.unwrap() {
                        println!("File name is : {}", file);
                    }
                }
                Err(_) => println!("输入无效，请输入 'true' 或 'false'"),
            }
        }
        "read_rar_txt"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_rar_txt(&input1, &input2).unwrap();
            println!("----------------读取rar压缩包内的txt文件内容-------------------");
            println!("TXT content: \n{}", data);
        }
        "read_rar_txt_by_line"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let lines = FileReader::read_rar_txt_by_line(&input1, &input2).unwrap();
            println!("----------------按行读取rar文件中txt文件内容-");
            for line in lines {
                println!("row -- > {}", line);
            }
        }
        "read_rar_txt_by_byte"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let bytes = FileReader::read_rar_txt_by_byte(&input1, &input2).unwrap();
            println!("----------------按字节读取rar文件中txt文件内容-");
            for byte in bytes {
                print!("{:02x} ", byte);
            }
            println!("");
        }
        "read_rar_txt_by_block"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let blocks = FileReader::read_rar_txt_by_block(&input1, &input2, number as usize).unwrap();
                    println!("----------------按块读取rar文件中txt文件内容-");
                    for block in blocks {
                        println!("block -- > {}", block);
                    }
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_rar_csv"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_rar_csv(&input1, &input2).unwrap();
            println!("-------------读取rar压缩包中csv文件内容-------------");
            for row in data {
                println!("{:?}", row);
            }
        }
        "read_rar_csv_by_column"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_rar_csv_by_column(&input1, &input2).unwrap();
            println!("-------------按列读取csv文件内容-");
            for column in data {
                println!("{:?}", column);
            }
        }
        "read_rar_csv_by_block"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let data = FileReader::read_rar_csv_by_block(&input1, &input2, number as usize).unwrap();
                    println!("-------------按块读取rar中csv文件内容-");
                    for block in data {
                        for row in block {
                            println!("{:?}", row);
                        }
                    }
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_rar_json"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_rar_json(&input1, &input2).unwrap();
            println!("-------------读取rar中json文件内容-");
            println!("JSON content: \n{}", data);
        }
        "read_rar_json_by_block"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let data = FileReader::read_rar_json_by_block(&input1, &input2, number as usize).unwrap();
                    println!("-------------按块读取json文件内容-");
                    for block in data {
                        for json in block {
                            println!("JSON content: \n{}", json);
                        }
                    }
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_rar_jsonl"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_rar_jsonl(&input1, &input2).unwrap();
            println!("-------------读取rar中jsonl文件内容-");
            for json in data {
                println!("JSON content: \n{}", json);
            }        
        }
        "read_rar_xml_text"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_rar_xml_text(&input1, &input2).unwrap();
            println!("-------------读取rar中xml文件内容-");
            println!("XML content: \n{}", data);
        }
        "read_rar_xml"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
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
        
            let res = FileReader::read_rar_xml(&input1,&input2, on_start, on_text, on_end);
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
        }
        "read_rar_markdown"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_rar_markdown(&input1, &input2).unwrap();
            println!("-------------读取rar中markdown文件内容-");
            println!("Markdown content: \n{}", data);
        }
        "read_rar_md_by_line"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_rar_md_by_line(&input1, &input2).unwrap();
            println!("-------------按行读取rar中markdown文件内容-");
            for line in data {
                println!("{}", line);
            }
        }
        "read_rar_md_by_byte"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_rar_md_by_byte(&input1, &input2).unwrap();
            println!("-------------按字节读取rar中markdown文件内容-");
            for byte in data {
                print!("{}", byte as char);
            }
        }
        "read_rar_md_by_block"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let data = FileReader::read_rar_md_by_block(&input1, &input2, number as usize).unwrap();
                    println!("-------------按块读取rar中markdown文件内容-");
                    for block in data {
                        println!("{}", block);
                    }
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_rar_excel"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_rar_excel(&input1, &input2).unwrap();
            for row in data {
                println!("{:?}", row);
            }
        }
        "read_rar_pdf_as_string"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_rar_pdf_as_string(&input1, &input2).unwrap();
            println!("-------------读取rar中pdf文件内容-");
            println!("PDF content: \n{}", data);
        }
        "read_rar_pptx_text"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_rar_pptx_text(&input1, &input2).unwrap();
            println!("-------------读取rar中pptx文件内容-");
            println!("PPTX content: \n{}", data);
        }
        "read_rar_pptx_text_by_slide"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_rar_pptx_text_by_slide(&input1, &input2).unwrap();
            for slide in data {
                println!("{:?}", slide);
            }
        }
        "read_ppt_notes_from_rar"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let data = FileReader::read_ppt_notes_from_rar(&input1, &input2).unwrap();
            for note in data {
                println!("Note content: {}", note);
            }
        }
        "list_tar_filenames"=>{
            let file_list = FileReader::list_tar_filenames(&input1).unwrap();
            println!("---------列出文件目录---------");
            for file_name in file_list {
                println!("{}", file_name);
            }
        }
        "read_tar_text"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let line = FileReader::read_tar_text(&input1, &input2);
            println!("---------读取成字符串---------");
            println!("\n{}",line.unwrap());
        }
        "read_tar_text_by_line"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let line = FileReader::read_tar_text_by_line(&input1, &input2);
            println!("---------按行读取字符串---------");
            for line in line.unwrap() {
                println!("by line {}", line);
            }
        }
        "read_tar_txt_by_byte"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let bytes = FileReader::read_tar_txt_by_byte(&input1, &input2).unwrap();
            println!("---------按字节读取文本文件内容---------");
            println!("{:?}", bytes);
        }
        "read_tar_txt_by_block"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let blocks = FileReader::read_tar_txt_by_block(&input1, &input2, number as usize).unwrap();
                    println!("---------按块读取文本文件内容---------");
                    for block in blocks {
                        println!("-- {}", block);
                    }
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_tar_csv_by_line"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let records = FileReader::read_tar_csv_by_line(&input1, &input2);
            println!("---------按行读取csv文件---------");
            for record in records.unwrap() {
                println!("{:?}", record);
            }
        }
        "read_tar_csv_by_column"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let columns = FileReader::read_tar_csv_by_column(&input1, &input2).unwrap();
            println!("---------按列读取csv文件---------");
            for column in columns {
                println!("{:?}", column);
            }
        }
        "read_tar_csv_by_block"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let blocks = FileReader::read_tar_csv_by_block(&input1, &input2, number as usize).unwrap();
                    println!("---------按块读取csv文件---------");
                    for block in blocks {
                        for record in block {
                            println!("{:?}", record);
                        }
                    }
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_tar_json"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let json_value = FileReader::read_tar_json(&input1, &input2);
            println!("---------读取json文件内容---------");
            println!("{:?}", json_value.unwrap());
        }
        "read_tar_jsonl_by_line"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let json_values = FileReader::read_tar_jsonl_by_line(&input1, &input2);
            println!("---------按行读取jsonl文件内容---------");
            for json_value in json_values.unwrap() {
                println!("{:?}", json_value);
            }
        }
        "read_tar_xml_text"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let xml_text = FileReader::read_tar_xml_text(&input1, &input2).unwrap();
            println!("---------读取xml文件内容---------");
            println!("{}", xml_text);            
        }
        "read_tar_xml_by_listener"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
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
        
            let res = FileReader::read_tar_xml_by_listener(&input1,&input2 ,on_start, on_text, on_end);
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
        }
        "read_tar_markdown"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let md_text = FileReader::read_tar_markdown(&input1, &input2).unwrap();
            println!("---------读取md文件内容---------");
            println!("{}", md_text);
        }
        "read_tar_md_by_line"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let lines = FileReader::read_tar_md_by_line(&input1, &input2).unwrap();
            println!("---------按行读取md文件内容---------");
            for line in lines {
                println!("{}", line);
            }
        }
        "read_tar_md_by_byte"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let bytes = FileReader::read_tar_md_by_byte(&input1, &input2).unwrap();
            println!("---------按字节读取md文件内容---------");
            for byte in bytes { 
                print!("{} ", byte); 
            }
            println!();
        }
        "read_tar_md_by_block"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let mut num = String::new();
            println!("请输入读取的block的大小:");
            io::stdin().read_line(&mut num).expect("无法读取输入");
            num = num.trim().to_string();
        
            match num.parse::<f64>() {
                Ok(number) => {
                    let blocks = FileReader::read_tar_md_by_block(&input1, &input2, number as usize).unwrap();
                    println!("---------按块读取md文件内容---------");
                    for block in blocks {
                        println!("{}", block);
                    }
            }
                Err(_) => println!("输入无效，请输入一个有效的数"),
            }
        }
        "read_tar_excel"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let lines = FileReader::read_tar_excel(&input1, &input2).unwrap();
            println!("---------读取整个excel文件内容---------");
            for line in lines {
                println!("{}", line);
            }
        }
        "read_tar_pdf_as_string"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let pdf_text = FileReader::read_tar_pdf_as_string(&input1, &input2).unwrap();
            println!("---------读取pdf文件内容---------");
            println!("{}", pdf_text);
        }
        "read_tar_pptx_text_by_slide"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let slide_texts = FileReader::read_tar_pptx_text_by_slide(&input1, &input2).unwrap();
            println!("---------按行读取ppt文件内容---------");
            for slide_text in slide_texts {
                println!("{}", slide_text);
            }
        }
        "read_tar_pptx_notes"=>{
            let mut input2 = String::new();
            println!("要打开的文件的路径:");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("无法读取输入");
            input2 = input2.trim().to_string();
            let notes = FileReader::read_tar_pptx_notes(&input1, &input2).unwrap();
            println!("---------读取ppt备注信息---------");
            for note in notes {
                println!("{}", note);
            }
        }
        "get_file_metadata"=>{
            let file_metadata = FileReader::get_file_metadata(&input1);
            println!("文件元数据: \n {:?}", file_metadata);
        }
        "get_file_size"=>{
            let file_len = get_file_size(&input1);
            println!("文件大小: {} 字节", file_len.unwrap()); // 如果成功，返回文件大小，否则抛出一个错误        
        }
        "get_file_created_time"=>{
            let created_time = get_file_created_time(&input1);
            println!("文件创建时间: {:?}", created_time.unwrap()); // 如果成功，返回文件创建时间，否则抛出一个错误
        
        }
        "get_last_accessed_time"=>{
            let accessed_time = get_last_accessed_time(&input1);
            println!("文件最后一次访问时间: {:?}", accessed_time.unwrap()); // 如果成功，返回文件访问时间，否则抛出一个错误
        
        }
        "get_last_modified"=>{
            let file_modified = get_last_modified(&input1);
            println!("文件最后修改时间: {:?}", file_modified.unwrap()); // 如果成功，返回文件最后修改时间，否则抛出一个错误
        
        }
        "is_directory"=>{
            let isDirectory = is_directory(&input1);
            println!("是否为目录: {}", isDirectory.unwrap()); // 如果成功，返回true或false，否则抛出一个错误
        
        }
        "is_regular_file"=>{
            let isOrdinary = is_regular_file(&input1);
            println!("是否为普通文件: {}", isOrdinary.unwrap()); // 如果成功，返回true或false，否则抛出一个错误
        
        }
        "parse_permissions"=>{
            
        }
        "get_file_permission"=>{
            let file_permission = get_file_permission(&input1);
            println!("文件权限: {:?}", file_permission.unwrap()); // 如果成功，返回文件权限，否则抛出一个错误
        
        }
        "get_file_extension"=>{
            let file_extension = get_file_extension(&input1);
            println!("文件扩展名: {:?}", file_extension); // 如果成功，返回文件扩展名，否则抛出一个错误
        }
        _ => println!("未知函数: {}", input),
    }
    thread::sleep(Duration::from_secs(3));
}








use FileContentReader::FileReader;



pub fn test_pdf() {
    // 得到pdf所有页内容，pretty 表示是否变成json格式
    // let pdf_content = FileReader::read_pdf_as_string("src/test_examples/test_pdf.pdf", false, "").unwrap();
    // println!("PDF Content: {}", pdf_content);

    // // 每个字符串对应一个页面的内容
    // let pdf_content_by_page = FileReader::read_pdf_as_string_by_page("src/test_examples/test_pdf.pdf", false, "").unwrap();
    // for page_content in pdf_content_by_page {
    //     println!("PDF Content: {}", page_content);
    // }

    // 测试
    let pdf_content = FileReader::read_pdf_as_string("src/test_examples/test_pdf.pdf").unwrap();
    println!("{}", pdf_content);

    let metadata = FileReader::get_file_metadata("src/test_examples/test_pdf.pdf");
    println!("metadata is {:?}", metadata.unwrap());

    

}

use FileContentReader::FileReader;


pub fn test_ppt() {
    // 返回ppt所有文本内容
    // let ppt_text = FileReader::read_pptx_text("src/test_examples/test_ppt.pptx");
    // println!("ppt text is\n{}", ppt_text.unwrap());

    // // 按页返回ppt文本内容
    // let ppt_text = FileReader::read_pptx_text_by_slide("src/test_examples/test_ppt.pptx");
    // for ppt in ppt_text.unwrap() {
    //     println!("ppt slide text is\n{}", ppt);
    // }

    // 读取ppt备注
    let ppt_notes = FileReader::read_pptx_notes("src/test_examples/test_ppt.pptx");
    for note in ppt_notes.unwrap() {
        println!("ppt note is \n{}", note);
    }




}


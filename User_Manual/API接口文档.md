#                                              API接口文档

## 1.读取txt文本文件

### 1.1 函数名: `read_txt`

#### 接口说明

**描述 **:  一次性读取文本文件内容,适合小文件

**参数**:  `file_path:<&str>` : 文件路径

**返回值**:  

成功：`Result<String>` : 返回字符串,字符串包含整个txt文件的内容

失败： 文件路径不存在， 文件类型错误，无法打开文件

#### 调用示例

  ```rust
use FileContentReader::FileReader;
let res = 												 FileReader::read_txt("src/test_examples/test_txt.txt");
match res {
    Ok(content) => println!("文本文件内容: \n {}", content),
    Err(e) => println!("读取文本文件失败: {}", e), 
}
  ```


### 1.2 函数名: `read_txt_by_line`

#### 接口说明

**描述**:  按行读取txt文本文件内容

**参数：**`file_path:AsRef<Path>`:文件路径

**返回值**:  

成功：`Result<Vec<String>>` : 返回一个字符串可变数组,一个字符串一个内容

失败：文件路径错误，文件类型错误，无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_txt_by_line("src/test_examples/test_txt.txt");
match res {
	Ok(content) => println!("文本文件内容: \n {:?}",content),
	Err(e) => println!("读取文本文件失败: {}", e), 
}
 ```



### 1.3 函数名: `read_txt_by_byte`

#### 接口说明

**描述**:  按字节读取txt文本文件内容

**参数**:  `file_path:AsRef<Path>` : 文件路径

**返回值**: 

成功： `Result<Vec<u8>>` : 返回一个u8字节数组，数组包含文本内容

失败：文件路径错误， 文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_txt_by_byte("src/test_examples/test_txt.txt");
match res {
    Ok(content) => println!("文本文件内容: \n {:?}", content),
    Err(e) => println!("读取文本文件失败: {}", e), 
}
 ```



### 1.4函数名: `read_txt_by_block`

#### 接口说明

**描述**:  按块读取txt文件

**参数**:  
	`file_path:<&str>` : 文件路径
	`block_size:<usize>` : 每次读取的块大小，以字节为单位

**返回值**: 

 成功：`Result<Vec<String>>` : 返回一个包含每个块的向量，每个块都是一个字符串

 失败： 文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_txt_by_block("src/test_examples/test_txt.txt",4);
match res {
	Ok(content) => println!("文本文件内容: \n {:?}", content),
	Err(e) => println!("读取文本文件失败: {}", e), 
}
 ```



## 2.读取csv文件

### 2.1函数名: `read_csv_by_line`

#### 接口说明

**描述**:  按行读取csv文件,默认不读取列名

**参数**:  `file_path:<&str>` : 文件路径

**返回值**:  

成功：`Result<Vec<Vec<String>>>` : 返回一个二维String数组，一个字符串为一个数据

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_csv_by_line("src/test_examples/test_csv.csv");
match res {
	Ok(content) => println!("csv文件内容: \n {:?}", content),
	Err(e) => println!("读取csv文件失败: {}", e), 
}
 ```


### 2.2函数名: `read_csv_by_column`

#### 接口说明

**描述**:  按列读取csv文件

**参数**:  `file_path:<&str>` : 文件路径

**返回值**:  

成功：`Result<Vec<Vec<String>>>` : 返回一个二维String数组，一个字符串为一个数据

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_csv_by_column("src/test_examples/test_csv.csv");
match res {
	Ok(content) => println!("csv文件内容: \n {:?}", content),
	Err(e) => println!("读取csv文件失败: {}", e), 
}
 ```



### 2.3函数名: `read_csv_by_block`

#### 接口说明

**描述**:  按块读取csv文件

**参数**:  
	`file_path:<&str>` : 文件路径
	`block_size:<usize>` : 每次读取的块大小，以字节为单位

**返回值**: 

成功： `Result<Vec<Vec<Vec<String>>>` : 返回一个向量，每个向量元素即为csv文件块的内容，用二维向量存储

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_csv_by_block("src/test_examples/test_csv.csv",4);
match res {
	Ok(content) => println!("csv文件内容: \n {:?}", content),
	Err(e) => println!("读取csv文件失败: {}", e), 
}
 ```



## 3.读取json文件

### 3.1函数名: `read_json_text`

#### 接口说明

**描述**:  读取json文件内容

**参数**:  `file_path:<&str>` : 文件路径

**返回值**:  

成功：`Result<String>` : 返回字符串,字符串包含整个json文件的内容

失败： 文件路径错误，文件类型错误， 无法打开文件 

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_json_text("src/test_examples/test_json.json");
match res {
	Ok(content) => println!("json文件内容: \n {}", content),
	Err(e) => println!("读取json文件失败: {}", e), 
}
 ```



### 3.2函数名: `read_json_dynamic`

#### 接口说明

**描述**:  动态读取json文件

**参数**:  `file_path:<&str>` : 文件路径

**返回值**: 

成功： `serde_json::Result<Value>` : 返回Value，Value包含整个json文件的内容

失败： 文件路径错误，文件类型错误， 无法打开文件 

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_json_dynamic("src/test_examples/test_json.json");
match res {
	Ok(content) => println!("json文件内容: \n {}", content),
	Err(e) => println!("读取json文件失败: {}", e), 
}
 ```



### 3.3函数名: `read_jsonl_by_line`

#### 接口说明

**描述**:  按行读取jsonl文件

**参数**:  `file_path:<&str>` : 文件路径

**返回值**:  

成功：`Result<Vec<Value>>` : 返回Value数组，Value数组包含整个jsonl文件的内容

失败： 文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
 use FileContentReader::FileReader;
 let res = FileReader::read_jsonl_by_line("src/test_examples/test_jsonline.jsonl");
match res {
	Ok(content) => println!("jsonline文件内容: \n {:?}", content),
	Err(e) => println!("读取jsonline文件失败: {}", e), 
}
 ```



### 3.4函数名: `read_json_by_block`

#### 接口说明

**描述**:  按块读取json文件

**参数**:
	`file_path:<&str>` : 文件路径
	`block_size:<usize>` : 每次读取的块大小，以对象的数量为单位

**返回值**:  

成功：`Result<Vec<Vec<Value>>>` : 返回包含每个块的向量，每个块都是一个对象向量

失败： 文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_json_by_block("src/test_examples/test_json.json",4);
match res {
	Ok(content) => println!("json文件内容: \n {:?}", content),
	Err(e) => println!("读取json文件失败: {}", e), 
}
 ```


## 4.读取xml文件

### 4.1函数名: `read_xml_text`

#### 接口说明

**描述**:  读取xml文件内容

**参数**:  `file_path:<&str>` : 文件路径

**返回值**:  

成功：`Result<String>` : 返回字符串,字符串包含整个xml文件的内容

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_xml_text("src/test_examples/test_xml.xml");
match res {
	Ok(content) => println!("xml文件内容: \n {}", content),
	Err(e) => println!("读取xml文件失败: {}", e), 
}
 ```



### 4.2函数名: `read_xml_by_listener`

#### 接口说明

**描述**:  读取xml文件内容

**参数**:
	`file_path:<&str>` : 文件路径
	`on_start:<FnMut(&str)>` : 处理start事件的函数
	`on_text:<FnMut(&str)>` :  处理text事件的函数
	`on_end:<FnMut(&str)>` :  处理end事件的函数

**返回值**:  

成功：`Result<()>` : 无返回值

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
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
 ```



## 5.读取markdown文件

### 5.1函数名: `read_markdown`

#### 接口说明

**描述**:  读取Markdown文件内容

**参数**:  `file_path:<&str>` : Markdown文件路径

**返回值**:  

成功：`Result<String>`:返回Markdown文件内容的字符串

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_markdown("src/test_examples/test_md.md");
match res {
    Ok(content) => println!("md文件内容: \n {}", content),
    Err(e) => println!("读取md文件失败: {}", e), 
}
 ```


### 5.2函数名: `read_md_by_line`

#### 接口说明

**描述**:  按行读取mardown文件

**参数**:  `file_path:<&str>` : markdown文件路径

**返回值**:  

成功：`Result<Vec<String>>` : 返回markdown文件内容的字符串数组

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_md_by_line("src/test_examples/test_md.md");
match res {
    Ok(content) => println!("md文件内容: \n {:?}", content),
    Err(e) => println!("读取md文件失败: {}", e), 
}
 ```



### 5.3函数名: `read_md_by_byte`

#### 接口说明

**描述**:  按字节读取markdown文件

**参数**:  `file_path:<&str>`: markdown文件路径

**返回值**:  

成功：`Result<Vec<u8>>`:返回markdown文件内容的u8字节数组

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_md_by_byte("src/test_examples/test_md.md");
match res {
    Ok(content) => println!("md文件内容: \n {:?}", content),
    Err(e) => println!("读取md文件失败: {}", e), 
}
 ```



### 5.4函数名: `read_md_by_block`

#### 接口说明

**描述**:  按块读取markdown文件

**参数**:
	`file_path:<&str>` : markdown文件路径
	`block_size:<usize>` : 每次读取的块大小，以字节为单位

**返回值**:  

成功：`Result<Vec<String>>` : 返回markdown文件内容的字符串数组

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_md_by_block("src/test_examples/test_md.md",4);
match res {
    Ok(content) => println!("md文件内容: \n {:?}", content),
    Err(e) => println!("读取md文件失败: {}", e), 
}
 ```



### 5.5函数名: `read_markdown_and_convert_to_html`

#### 接口说明

**描述**:  读取markdown文件并转换为HTML

**参数**:  `file_path:<&str>` : Markdown文件路径

**返回值**:  

成功：`Result<String>` : 返回转换后的HTML内容

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_markdown_and_convert_to_html("src/test_examples/test_md.md");
match res {
    Ok(content) => println!("md文件内容: \n {}", content),
    Err(e) => println!("读取md文件失败: {}", e), 
}
 ```


## 6.读取PDF文件

### 6.1函数名: `read_pdf_as_string`

#### 接口说明

**描述**:  读取PDF文件内容

**参数**:  `file_path:<&str>` : pdf文件路径

**返回值**:  

成功：`Result<String>` : 返回pdf所有页内容的字符串

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_pdf_as_string("src/test_examples/test_pdf.pdf");
match res {
	Ok(content) => println!("pdf文件内容: \n {}", content),
	Err(e) => println!("读取pdf文件失败: {}", e), 
}
 ```



## 7.读取excel文件

### 7.1函数名: `read_excel_as_string`

#### 接口说明

**描述**:  读取excel文件中的所有内容，并以字符串形式返回

**参数**:  `file_path:<&str>` : excel文件路径

**返回值**:  

成功：`Result<String>` : 成功时返回包含excel内容的字符串，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_excel_as_string("src/test_examples/test_excel.xlsx");
match res {
    Ok(content) => println!("excel文件内容: \n {}", content),
    Err(e) => println!("读取excel文件失败: {}", e), 
}
 ```



### 7.2函数名: `read_excel_as_string_by_sheet`

#### 接口说明

**描述**:  按照Sheet读取excel文件中的所有内容，并以字符串形式返回

**参数**:  `file_path:<&str>` : excel文件路径

**返回值**:  

成功：`Result<Vec<String>>` : 成功时返回包含excel内容的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_excel_as_string_by_sheet("src/test_examples/test_excel.xlsx");
match res {
    Ok(content) => println!("excel文件内容: \n {:?}", content),
    Err(e) => println!("读取excel文件失败: {}", e), 
}
 ```



### 7.3函数名: `read_excel_by_column_as_string`

#### 接口说明

**描述**:  读取excel文件中的所有列的内容，并按列格式化为字符串，感觉这个函数意义不大

**参数**:  `file_path` : excel文件路径

**返回值**: 

成功： `Result<String>` : 成功时返回包含excel内容的字符串，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_excel_by_column_as_string("src/test_examples/test_excel.xlsx");
match res {
    Ok(content) => println!("excel文件内容: \n {:?}", content),
    Err(e) => println!("读取excel文件失败: {}", e), 
}
 ```



### 7.4函数名: `read_excel_by_row`

#### 接口说明

**描述**:  按行读取excel文件内容，每一行一个向量，每个元素用制表符分隔

**参数**:  `file_path` : Excel文件路径

**返回值**:  

成功：`Result<Vec<Vec<String>>>`: 成功时返回包含Excel内容的字符串向量，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_excel_by_row("src/test_examples/test_excel.xlsx");
match res {
    Ok(content) => println!("excel文件内容: \n {:?}", content),
    Err(e) => println!("读取excel文件失败: {}", e), 
}
 ```



### 7.5函数名: `read_excel_by_column`

#### 接口说明

**描述**:  按列读取excel文件内容，每一列一个向量，每个元素用制表符分隔

**参数**:  `file_path:<&str>` : Excel文件路径

**返回值**:  

成功：`Result<Vec<Vec<String>>>` : 成功时返回包含Excel内容的字符串向量，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_excel_by_column("src/test_examples/test_excel.xlsx");
match res {
    Ok(content) => println!("excel文件内容: \n {:?}", content),
    Err(e) => println!("读取excel文件失败: {}", e), 
}
 ```



### 7.6函数名: `read_excel_by_block_as_string`

#### 接口说明

**描述**:  读取Excel文件中的内容并按块格式化为字符串，返回就是一个字符串，用来展示的

**参数**:  
	`file_path:<&str>` : Excel文件路径
	`block_size:(usize,usize)` : 读取块的大小 (rows, cols)

**返回值**:  

成功：`Result<String>` : 成功时返回包含Excel内容的字符串，失败时返回错误信息,只是返回一个字符串,块都是按行展开

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_excel_by_block_as_string("src/test_examples/test_excel.xlsx",(4,4));
match res {
    Ok(content) => println!("excel文件内容: \n {}", content),
    Err(e) => println!("读取excel文件失败: {}", e), 
}
 ```



### 7.7函数名: `read_excel_by_block`

#### 接口说明

**描述**:  读取Excel文件中的内容并按块格式化为字符串

**参数**:  
	`file_path:<&str>` : excel文件路径
	`block_size:(usize,usize)` : 读取块的大小 (rows, cols)

**返回值**:  

成功：`Result<String>` : 成功时返回包含excel内容的字符串，失败时返回错误信息,每一个块是一个字符串

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_excel_by_block("src/test_examples/test_excel.xlsx",(4,4));
match res {
    Ok(content) => println!("excel文件内容: \n {:?}", content),
    Err(e) => println!("读取excel文件失败: {}", e), 
}
 ```



## 8.读取pptx文件

### 8.1函数名: `read_pptx_text`

#### 接口说明

**描述**:  从pptx文件中提取文本内容

**参数**:  `file_path:<&str>` : pptx文件路径

**返回值**:  

成功：`Result<String>`: 成功时返回包含PPTX内容的字符串，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_pptx_text("src/test_examples/test_ppt.pptx");
match res {
    Ok(content) => println!("pptx文件内容: \n {}", content),
    Err(e) => println!("读取pptx文件失败: {}", e), 
}
 ```



### 8.2函数名: `read_pptx_text_by_slide`

#### 接口说明

**描述**:  按页读取pptx文件内容，每一页一个向量

**参数**:  `file_path:<&str>` : PPTX文件路径

**返回值**:  

成功：`Result<Vec<String>>` : 成功时返回包含PPTX内容的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_pptx_text_by_slide("src/test_examples/test_ppt.pptx");
match res {
    Ok(content) => println!("pptx文件内容: \n {:?}", content),
    Err(e) => println!("读取pptx文件失败: {}", e), 
}
 ```



### 8.3函数名: `read_pptx_notes`

#### 接口说明

**描述**:  读取ppt备注

**参数**:  `file_path:<&str>` : PPTX文件路径

**返回值**:  

成功：`Result<Vec<String>>` : 成功时返回包含PPTX备注的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_pptx_text_by_notes("src/test_examples/test_ppt.pptx");
match res {
    Ok(content) => println!("pptx文件备注: \n {:?}", content),
    Err(e) => println!("读取pptx文件失败: {}", e), 
}
```



## 9.读取zip文件

### 9.1函数名: `list_zip_filenames`

#### 接口说明

**描述**:  列出zip文件中所有文件目录

**参数**:  
	`file_path:<&str>`: zip文件路径
	`exclude_directories:<bool>` : 是否包含压缩包里的文件目录

**返回值**:  

成功：`Result<Vec<String>>` : 成功时返回包含zip所有文件目录的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::list_zip_filenames("src/test_examples/test_zip.zip",true);
match res {
    Ok(content) => println!("zip文件中所以文件目录: \n {:?}", content),
    Err(e) => println!("读取zip文件失败: {}", e), 
}
 ```



### 9.2函数名: `read_zip_txt`

#### 接口说明

**描述**:  一次性读取zip内文本文件内容

**参数**:  
	`file_path:<&str>` : zip文件路径
	`file_name:<&str>`: 要打开的文本文件的路径

**返回值**:  

成功：`Result<String>` : 成功时返回包含指定的zip文本文件内容的字符串，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_txt("src/test_examples/test_zip.zip","test_rar/test_txt.txt");
match res {
	Ok(content) => println!("zip文件中指定文本文件的内容: \n {}", content),
	Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.3函数名: `read_zip_txt_by_line`

#### 接口说明

**描述**:  按行读取文本文件内容

**参数**:  
	`zip_path:<&str>` : zip文件路径
	`file_name:<&str>` : 要打开的文本文件的路径

**返回值**:  

成功：`Result<Vec<String>>` : 成功时返回包含指定的zip文本文件内容的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_txt_by_line("src/test_examples/test_zip.zip","test_rar/test_txt.txt");
match res {
	Ok(content) => println!("zip文件中指定文本文件的内容: \n {:?}", content),
	Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.4函数名: `read_zip_txt_by_byte`

#### 接口说明

**描述**:  按字节读取zip内文本文件内容

**参数**:  
	`zip_path:<&str>` : zip文件路径
	`file_name:<&str>` : 要打开的文本文件的路径

**返回值**:  

成功：`Result<Vec<u8>>`: 成功时返回包含指定的zip文本文件内容的字节数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_txt_by_byte("src/test_examples/test_zip.zip","test_rar/test_txt.txt");
match res {
	Ok(content) => println!("zip文件中指定文本文件的内容: \n {:?}", content),
	Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.5函数名: `read_zip_txt_by_block`

#### 接口说明

**描述**:  按块读取zip文件内文本文件

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的文本文件的路径
	`block_size:<usize>` : 每次读取的块大小，以字节为单位

**返回值**:  

成功：`Result<Vec<String>>` : 返回一个包含每个块的向量，每个块都是一个字符串

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_txt_by_byte("src/test_examples/test_zip.zip","test_rar/test_txt.txt",4);
match res {
	Ok(content) => println!("zip文件中指定文本文件的内容: \n {:?}", content),
	Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.6函数名: `read_zip_csv`

#### 接口说明

**描述**:  按行读取zip内csv文件内容

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的csv文件的路径

**返回值**: 

成功： `Result<Vec<Vec<String>>>`：返回一个二维字符串数组

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_csv("src/test_examples/test_zip.zip","test_rar/test_csv.csv");
match res {
    Ok(content) => println!("zip文件中指定csv文件的内容: \n {:?}", content),
    Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.7函数名: `read_zip_csv_by_column`

#### 接口说明

**描述**:  按列读取zip中csv文件内容，默认是没有读取列名的

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的csv文件的路径

**返回值**: 

成功： `Result<Vec<Vec<String>>>`：返回一个二维字符串数组

失败： 文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_csv_by_column("src/test_examples/test_zip.zip","test_rar/test_csv.csv");
match res {
    Ok(content) => println!("zip文件中指定csv文件的内容: \n {:?}", content),
    Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.8函数名: `read_zip_csv_by_block`

#### 接口说明

**描述**:  按块读取zip中csv文件内容

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的csv文件的路径
	`block_size:<usize>` : 每次读取的块大小，以字节为单位

**返回值**: 

成功： `Result<Vec<Vec<Vec<String>>>>`：返回一个向量，每个向量元素即为csv文件块的内容，用二维向量存储

失败： 文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_csv_by_block("src/test_examples/test_zip.zip","test_rar/test_csv.csv",4);
match res {
	Ok(content) => println!("zip文件中指定csv文件的内容:\n {:?}", content),
	Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.9函数名: `read_zip_json`

#### 接口说明

**描述**:  动态读取zip中json文件

**参数**:  
	`zip_path:<&str>`  : 文件路径
	`file_name:<&str>`  : 要打开的json文件的路径

**返回值**:  

成功：`serde_json::Result<Value>` ：返回Value，Value包含整个json文件的内容

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_json("src/test_examples/test_zip.zip","test_rar/test_json.json");
match res {
    Ok(content) => println!("zip文件中指定json文件的内容:\n {:?}", content),
    Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.10函数名: `read_zip_json_by_block`

#### 接口说明

**描述**:  按块读取json文件

**参数**:  
	`zip_path:<&str>` : 文件路径
	`file_name:<&str>`  : 要打开的json文件的路径
	`block_size:<usize>` : 每次读取的块大小，以字节为单位

**返回值**:  

成功：`Result<Vec<Vec<Value>>>`：返回包含每个块的向量，每个块都是一个对象向量

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_json_by_block("src/test_examples/test_zip.zip","test_rar/test_json.json",4);
match res {
 	Ok(content) => println!("zip文件中指定json文件的内容: \n {:?}", content),
 	Err(e) => println!("读取zip文件失败: {}", e), 
 }
```



### 9.11函数名: `read_zip_jsonl`

#### 接口说明

**描述**:  按行读取zip内jsonl文件

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的jsonl文件的路径

**返回值**: 

成功： `Result<Vec<Value>>`：返回Value数组，Value数组包含整个jsonl文件的内容

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_jsonl("src/test_examples/test_zip.zip","test_rar/test_jsonline.jsonl");
match res {
    Ok(content) => println!("zip文件中指定jsonl文件的内容: \n {:?}", content),
    Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.12函数名: `read_zip_xml_text`

#### 接口说明

**描述**:  直接读取zip文件中xml文件内容

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的xml文件的路径

**返回值**:  

成功：`Result<String>`：返回一个字符串，字符串中包含指定xml文件全部内容

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_xml_text("src/test_examples/test_zip.zip","test_rar/test_xml.xml");
match res {
	Ok(content) => println!("zip文件中指定xml文件的内容:\n {}", content),
	Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.13函数名: `read_zip_xml`

#### 接口说明

**描述**:  按事件处理读取zip文件中xml文件内容

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的xml文件的路径
	`on_start:<FnMut(&str)>`: 处理Start事件的函数
	`on_text:<FnMut(&str)>`: 处理Text事件的函数
	`on_end:<FnMut(&str)>`: 处理End事件的函数

**返回值**:  

成功：`Result<()>`：无返回值

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
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

let res = FileReader::read_zip_xml("src/test_examples/test_zip.zip", "test_rar/test_xml.xml", on_start, on_text, on_end);
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
```



### 9.14函数名: `read_zip_markdown`

#### 接口说明

**描述**:  一次性读取zip文件中md文件内容

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的md文件的路径

**返回值**:  

成功：`Result<String>`：返回一个字符串，字符串中包含指定md文件全部内容

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_markdown("src/test_examples/test_zip.zip","test_rar/test_md.md");
match res {
    Ok(content) => println!("zip文件中指定md文件的内容: \n {}", content),
    Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.15函数名: `read_zip_md_by_line`

#### 接口说明

**描述**:  按行读取zip文件中md文件内容

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的md文件的路径

**返回值**:  

成功：`Result<Vec<String>>`：返回一个字符串数组，每个字符串中包含指定md文件的内容

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_md_by_line("src/test_examples/test_zip.zip","test_rar/test_md.md");
match res {
    Ok(content) => println!("zip文件中指定md文件的内容: \n {:?}", content),
    Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.16函数名: `read_zip_md_by_byte`

#### 接口说明

**描述**:  按字节读取zip文件中md文件内容

**参数**:  
	`zip_path:<&str>` : 文件路径
	`file_name:<&str>` : 要打开的md文件的路径

**返回值**:  

成功：`Result<Vec<u8>>`：返回一个u8字节数组，包含指定md文件全部内容

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_md_by_byte("src/test_examples/test_zip.zip","test_rar/test_md.md");
match res {
       Ok(content) => println!("zip文件中指定md文件的内容: \n {:?}", content),
       Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.17函数名: `read_zip_md_by_block`

#### 接口说明

**描述**:  按块读取zip文件中markdown文件内容

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的md文件的路径
	`block_size:<usize>`: 每次读取的块大小，以字节为单位

**返回值**:  

成功：`Result<Vec<String>>`：返回指定markdown文件内容的字符串数组

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_md_by_block("src/test_examples/test_zip.zip","test_rar/test_md.md",4);
match res {
    Ok(content) => println!("zip文件中指定md文件的内容: \n {:?}", content),
    Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.18函数名: `read_zip_pdf_as_string`

#### 接口说明

**描述**:  按块读取zip文件中md文件内容

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的pdf文件的路径

**返回值**:  

成功：`Result<String>`：返回指定pdf文件内容的字符串

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_pdf_as_string("src/test_examples/test_zip.zip","test_rar/test_pdf.pdf");
match res {
    Ok(content) => println!("zip文件中指定pdf文件的内容: \n {}", content),
    Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.19函数名: `read_zip_excel`

#### 接口说明

**描述**:  按块读取zip文件中excel文件内容

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的excel文件的路径

**返回值**:  

成功：`Result<Vec<String>>`：返回指定excel文件内容的字符串数组

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_excel("src/test_examples/test_zip.zip","test_rar/test_excel.xlsx");
match res {
    Ok(content) => println!("zip文件中指定excel文件的内容: \n {:?}", content),
    Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.20函数名: `read_zip_pptx_text`

#### 接口说明

**描述**:  读取zip文件中ppt内容,返回一个字符串，便于展示

**参数**:  
	`zip_path:<&str>` : 文件路径
	`file_name:<&str>` : 要打开的ppt文件的路径

**返回值**:  

成功：`Result<String>` ：返回指定ppt文件内容的字符串

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_pptx_text("src/test_examples/test_zip.zip","test_rar/test_ppt.pptx");
match res {
    Ok(content) => println!("zip文件中指定ppt文件的内容: \n {}", content),
    Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.21函数名: `read_zip_pptx_text_by_slide`

#### 接口说明

**描述**:  读取zip文件中ppt内容,返回一个字符串，便于展示

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>` - ZIP文件内部PPTX文件的路径（相对路径）

**返回值**:  

成功：`Result<Vec<String>>` - 返回指定pptx文件中提取的所有幻灯片文本内容，每一页一个向量

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_pptx_text_by_slide("src/test_examples/test_zip.zip","test_rar/test_ppt.pptx");
match res {
    Ok(content) => println!("zip文件中指定pptx文件的内容: \n {:?}", content),
    Err(e) => println!("读取zip文件失败: {}", e), 
}
```



### 9.22函数名: `read_zip_pptx_notes`

#### 接口说明

**描述**:  读取ZIP文件中ppt备注

**参数**:  
	`zip_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的pptx文件的路径

**返回值**:  

成功：`Result<Vec<String>>`：返回指定pptx文件备注的字符串数组

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_zip_pptx_notes("src/test_examples/test_zip.zip","test_rar/test_ppt.pptx");
match res {
    Ok(content) => println!("zip文件中指定ppt文件的内容: \n {:?}", content),
    Err(e) => println!("读取zip文件失败: {}", e), 
}
```





## 10.读取rar文件

### 10.1函数名: `list_rar_filenames`

#### 接口说明

**描述**:  列出 RAR 文件中的所有文件和目录

**参数**:  
	`file_path:<&str>`: RAR 文件的路径
	`exclude_directories:<bool>`: 是否包含压缩包里的文件目录

**返回值**:  

成功：`Result<Vec<String>, Box<dyn Error>>`: 成功时返回文件和目录的名称列表，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::list_rar_filenames("src/test_examples/test_rar.rar",true);
match res {
    Ok(content) => println!("rar文件中所有文件和目录: \n {:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
 ```

### 10.2函数名: `read_rar_txt`

#### 接口说明

**描述**:  读取rar压缩包内的txt文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的文本文件的路径

**返回值**:  

成功：`UnrarResult<String, Box<dyn UnrarError>>`: 成功时返回文件文本内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_txt("src/test_examples/test_rar.rar","test_rar\\test_txt.txt");
match res {
    Ok(content) => println!("rar文件中指定txt文件的内容: \n {}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.3函数名: `read_rar_txt_by_line`

#### 接口说明

**描述**:  按行读取rar文件中txt文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的文本文件的路径

**返回值**:  

成功：`UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回包含指定的rar文本文件内容的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_txt_by_line("src/test_examples/test_rar.rar","test_rar\\test_txt.txt");
match res {
    Ok(content) => println!("rar文件中指定txt文件的内容: \n {:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.4函数名: `read_rar_txt_by_byte`

#### 接口说明

**描述**:  按字节读取rar文件中txt文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的文本文件的路径

**返回值**:  

成功： `UnrarResult<Vec<u8>, Box<dyn UnrarError>>`: 成功时返回包含指定的rar文本文件内容的字节数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_txt_by_byte("src/test_examples/test_rar.rar","test_rar\\test_txt.txt");
match res {
    Ok(content) => println!("rar文件中指定txt文件的内容: \n {:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.5函数名: `read_rar_txt_by_block`

#### 接口说明

**描述**:  按块读取rar文件中txt文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的文本文件的路径
	`block_size:<usize>`: 每次读取的块大小，以字节为单位

**返回值**:  

成功：`UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回一个包含每个块的向量，每个块都是一个字符串，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_txt_by_block("src/test_examples/test_rar.rar","test_rar\\test_txt.txt",4);
match res {
    Ok(content) => println!("rar文件中指定txt文件的内容: \n {:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.6函数名: `read_rar_csv`

#### 接口说明

**描述**:  读取rar压缩包中csv文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的csv文件的路径

**返回值**:  

成功：`UnrarResult<Vec<Vec<String>>, Box<dyn UnrarError>>`: 成功时返回一个二维字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_csv("src/test_examples/test_rar.rar","test_rar\\test_csv.csv");
match res {
    Ok(content) => println!("rar文件中指定csv文件的内容: \n {:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.7函数名: `read_rar_csv_by_column`

#### 接口说明

**描述**:  按列读取csv文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的csv文件的路径

**返回值**:  

成功：`UnrarResult<Vec<Vec<String>>, Box<dyn UnrarError>>`: 成功时返回一个二维字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_csv_by_column("src/test_examples/test_rar.rar","test_rar\\test_csv.csv");
match res {
    Ok(content) => println!("rar文件中指定csv文件的内容: \n {:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.8函数名: `read_rar_csv_by_block`

#### 接口说明

**描述**:  按块读取rar中csv文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的csv文件的路径
	`block_size:<usize>`: 每次读取的块大小，以字节为单位

**返回值**:  

成功：`UnrarResult<Vec<Vec<Vec<String>>>, Box<dyn UnrarError>>`: 成功时返回一个向量，每个向量元素即为csv文件块的内容，用二维向量存储，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_csv_by_block("src/test_examples/test_rar.rar","test_rar\\test_csv.csv",4);
match res {
    Ok(content) => println!("rar文件中指定csv文件的内容: \n {:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.9函数名: `read_rar_json`

#### 接口说明

**描述**:  读取rar中json文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的json文件的路径

**返回值**:  

成功：`UnrarResult<Value, Box<dyn UnrarError>>`: 成功时返回Value，Value包含整个json文件的内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_json("src/test_examples/test_rar.rar","test_rar\\test_json.json");
match res {
    Ok(content) => println!("rar文件中指定json文件的内容: \n {:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.10函数名: `read_rar_json_by_block`

#### 接口说明

**描述**:  按块读取json文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的json文件的路径
	`block_size:<usize>`: 每次读取的块大小，以字节为单位

**返回值**:  

成功：`UnrarResult<Vec<Vec<Value>>, Box<dyn UnrarError>>`: 成功时返回包含每个块的向量，每个块都是一个对象向量，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_json_by_block("src/test_examples/test_rar.rar","test_rar\\test_json.json",4);
match res {
    Ok(content) => println!("rar文件中指定json文件的内容: \n {:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.11函数名: `read_rar_jsonl`

#### 接口说明

**描述**:  列出 RAR 文件中的所有文件和目录

**参数**:  
	`file_path:<&str>`: RAR 文件的路径
	`exclude_directories:<bool>`: 是否包含压缩包里的文件目录

**返回值**:  

成功：`UnrarResult<Vec<Value>, Box<dyn UnrarError>>`: 成功时返回Value数组，Value数组包含整个jsonl文件的内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_jsonl("src/test_examples/test_rar.rar","test_rar\\test_json.json");
match res {
    Ok(content) => println!("rar文件中指定jsonl文件的内容: \n {:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.12函数名: `read_rar_xml_text`

#### 接口说明

**描述**:  从rar读取xml文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的xml文件的路径

**返回值**:  

成功：`UnrarResult<String, Box<dyn UnrarError>>`: 成功时返回一个字符串，字符串中包含指定xml文件全部内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_xml_text("src/test_examples/test_rar.rar","test_rar\\test_json.json");
match res {
    Ok(content) => println!("rar文件中指定xml文件的内容: \n {}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.13函数名: `read_rar_xml`

#### 接口说明

**描述**:  按事件读取xml文件内容

**参数**:  
	`rar_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的xml文件的路径
	`on_start:<FnMut(&str)>`: 处理Start事件的函数
	`on_text:<FnMut(&str)>`: 处理Text事件的函数
	`on_end:<FnMut(&str)>`: 处理End事件的函数

**返回值**:  

成功：`Result<()>`：无返回值

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
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
```

### 10.14函数名: `read_rar_markdown`

#### 接口说明

**描述**:  从rar中读取md文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的md文件的路径

**返回值**:  

成功：`UnrarResult<String, Box<dyn UnrarError>>`: 成功时返回一个字符串，字符串中包含指定md文件全部内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_markdown("src/test_examples/test_rar.rar","test_rar\\test_md.md");
match res {
    Ok(content) => println!("rar文件中指定md文件的内容: \n {}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.15函数名: `read_rar_md_by_line`

#### 接口说明

**描述**:  按行读取md文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的md文件的路径

**返回值**:  

成功： `UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回一个字符串数组，每个字符串中包含指定md文件的内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_md_by_line("src/test_examples/test_rar.rar","test_rar\\test_md.md");
match res {
    Ok(content) => println!("rar文件中指定md文件的内容: \n 		{:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.16函数名: `read_rar_md_by_byte`

#### 接口说明

**描述**:  按字节读取md文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的md文件的路径

**返回值**:  

成功：`UnrarResult<Vec<u8>, Box<dyn UnrarError>>`: 成功时返回一个u8字节数组，包含指定md文件全部内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_md_by_byte("src/test_examples/test_rar.rar","test_rar\\test_md.md");
match res {
    Ok(content) => println!("rar文件中指定md文件的内容: \n 		{:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.17函数名: `read_rar_md_by_block`

#### 接口说明

**描述**:  按块读取md文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的md文件的路径
	`block_size:<usize>`: 每次读取的块大小，以字节为单位

**返回值**:  

成功：`UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回指定Markdown文件内容的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_md_by_block("src/test_examples/test_rar.rar","test_rar\\test_md.md",4);
match res {
    Ok(content) => println!("rar文件中指定md文件的内容: \n {:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.18函数名: `read_rar_excel`

#### 接口说明

**描述**:  从rar中读取excel文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的excel文件的路径

**返回值**:  

成功：`UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回指定excel文件内容的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_excel("src/test_examples/test_rar.rar","test_rar\\test_excel.xlsx");
match res {
    Ok(content) => println!("rar文件中指定excel文件的内容: \n 	{:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.19函数名: `read_rar_pdf_as_string`

#### 接口说明

**描述**:  从rar中读取pdf文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的pdf文件的路径

**返回值**:  

成功：`UnrarResult<String, Box<dyn UnrarError>>`: 成功时返回指定pdf文件内容的字符串，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_pdf_as_string("src/test_examples/test_rar.rar","test_rar\\test_pdf.pdf");
match res {
    Ok(content) => println!("rar文件中指定pdf文件的内容: \n 		{}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.20函数名: `read_rar_pptx_text`

#### 接口说明

**描述**:  从rar中读取ppt文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的ppt文件的路径

**返回值**:  

成功： `UnrarResult<String, Box<dyn UnrarError>>`: 成功时返回指定ppt文件内容的字符串，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_pptx_text("src/test_examples/test_rar.rar","test_rar\\test_ppt.pptx");
match res {
    Ok(content) => println!("rar文件中指定ppt文件的内容: \n {}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.21函数名: `read_rar_pptx_text_by_slide`

#### 接口说明

**描述**:  按行读取rar中ppt文件内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的pptx文件的路径

**返回值**:  

成功：`UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回指定pptx文件中提取的所有幻灯片文本内容，每一页一个向量，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_rar_pptx_text_by_slide("src/test_examples/test_rar.rar","test_rar\\test_ppt.pptx");
match res {
    Ok(content) => println!("rar文件中指定ppt文件的内容: \n 		{:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

### 10.22函数名: `read_ppt_notes_from_rar`

#### 接口说明

**描述**:  按页读取rar中ppt文件备注内容

**参数**:  
	`rar_path:<&str>`: RAR 文件的路径
	`file_name:<&str>`: 要打开的pptx文件的路径

**返回值**:  

成功：`UnrarResult<Vec<String>, Box<dyn UnrarError>>`: 成功时返回指定ppt文件备注的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_ppt_notes_from_rar("src/test_examples/test_rar.rar","test_rar\\test_ppt.pptx");
match res {
    Ok(content) => println!("rar文件中指定ppt文件的内容: \n {:?}", content),
    Err(e) => println!("读取rar文件失败: {}", e), 
}
```

## 11.读取tar文件

### 11.1函数名: `list_tar_filenames`

#### 接口说明

**描述**:  列出文件目录

**参数**:  
	`file_path:<P: AsRef<Path>>`: tar文件路径
	`exclude_directories:<bool>`: 是否包含压缩包里的文件目录

**返回值**:  

成功：`Result<Vec<String>>`: 成功时返回包含tar所有文件目录的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::list_tar_filenames("src/test_examples/test_tar.tar");
match res {
    Ok(content) => println!("tar文件中所有文件目录: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.2函数名: `read_tar_text`

#### 接口说明

**描述**:  直接读取txt文件内容

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的文本文件的路径

**返回值**:  

成功：`Result<String>`: 成功时返回包含指定的tar文本文件内容的字符串，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_text("src/test_examples/test_tar.tar","test_txt.txt");
match res {
    Ok(content) => println!("tar文件中指定文本文件的内容: \n {}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.3函数名: `read_tar_text_by_line`

#### 接口说明

**描述**:  按行读取txt文件内容

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的文本文件的路径

**返回值**:  

成功：`Result<Vec<String>>`: 成功时返回包含指定的tar文本文件内容的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_text_by_line("src/test_examples/test_tar.tar","test_txt.txt");
match res {
    Ok(content) => println!("tar文件中指定文本文件的内容: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.4函数名: `read_tar_txt_by_byte`

#### 接口说明

**描述**:  按字节读取文本文件内容

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的文本文件的路径

**返回值**:  

成功：`Result<Vec<u8>>`: 成功时返回包含指定的tar文本文件内容的字节数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_txt_by_byte("src/test_examples/test_tar.tar","test_txt.txt");
match res {
    Ok(content) => println!("tar文件中指定文本文件的内容: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.5函数名: `read_tar_txt_by_block`

#### 接口说明

**描述**:  按块读取txt文本文件内容

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的文本文件的路径
	`block_size:<usize>`: 每次读取的块大小，以字节为单位

**返回值**:  

成功：`Result<Vec<String>>`: 成功时返回一个包含每个块的向量，每个块都是一个字符串，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_txt_by_block("src/test_examples/test_tar.tar","test_txt.txt",4);
match res {
    Ok(content) => println!("tar文件中指定文本文件的内容: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.6函数名: `read_tar_csv_by_line`

#### 接口说明

**描述**:  按行读取csv文件

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的csv文件的路径

**返回值**:  

成功：`Result<Vec<Vec<String>>>`: 成功时返回一个二维字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_csv_by_line("src/test_examples/test_tar.tar","test_csv.csv");
match res {
    Ok(content) => println!("tar文件中csv文件的内容: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.7函数名: `read_tar_csv_by_column`

#### 接口说明

**描述**:  按列读取csv文件

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的csv文件的路径

**返回值**:  

成功：`Result<Vec<Vec<String>>>`: 成功时返回一个二维字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_csv_by_column("src/test_examples/test_tar.tar","test_csv.csv");
match res {
    Ok(content) => println!("tar文件中csv文件的内容: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.8函数名: `read_tar_csv_by_block`

#### 接口说明

**描述**:  按块读取csv文件

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的csv文件的路径
	`block_size:<usize>`: 每次读取的块大小，以字节为单位

**返回值**:  

成功：`Result<Vec<Vec<Vec<String>>>>`: 成功时返回一个向量，每个向量元素即为csv文件块的内容，用二维向量存储，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_csv_by_block("src/test_examples/test_tar.tar","test_csv.csv",4);
match res {
    Ok(content) => println!("tar文件中csv文件的内容: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.9函数名: `read_tar_json`

#### 接口说明

**描述**:  读取json文件内容

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的json文件的路径

**返回值**:  

成功：`Result<Value>`: 成功时返回Value，Value包含整个json文件的内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_json("src/test_examples/test_tar.tar","test_json.json");
match res {
    Ok(content) => println!("tar文件中json文件的内容: \n {}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.10函数名: `read_tar_jsonl_by_line`

#### 接口说明

**描述**:  按行读取jsonl文件内容

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的jsonl文件的路径

**返回值**:  

成功：`Result<Vec<Value>>`: 成功时返回Value数组，Value数组包含整个jsonl文件的内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_jsonl_by_line("src/test_examples/test_tar.tar","test_jsonline.jsonl");
match res {
    Ok(content) => println!("tar文件中jsonl文件的内容: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.11函数名: `read_tar_xml_text`

#### 接口说明

**描述**:  直接读取xml文件内容

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的xml文件的路径

**返回值**:  

成功：`Result<String>`: 成功时返回一个字符串，字符串中包含指定xml文件全部内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_xml_text("src/test_examples/test_tar.tar","test_xml.xml");
match res {
    Ok(content) => println!("tar文件中xml文件的内容: \n {}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.12函数名: `list_tar_filenames`

#### 接口说明

**描述**:  按照事件读取xml文件内容

**参数**:  
	`tar_path:<&str>`: 文件路径
	`file_name:<&str>`: 要打开的xml文件的路径
	`on_start:<FnMut(&str)>`: 处理Start事件的函数
	`on_text:<FnMut(&str)>`: 处理Text事件的函数
	`on_end:<FnMut(&str)>`: 处理End事件的函数

**返回值**:  

成功：`Result<()>`：无返回值

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
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
let res = FileReader::read_tar_xml_by_listener("src/test_examples/test_tar.tar","test_xml.xml", on_start, on_text, on_end);
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
```

### 11.13函数名: `read_tar_markdown`

#### 接口说明

**描述**:  直接读取md文件内容

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的md文件的路径

**返回值**:  

成功： `Result<String>`: 成功时返回一个字符串，字符串中包含指定md文件全部内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_markdown("src/test_examples/test_tar.tar","test_md.md");
match res {
    Ok(content) => println!("tar文件中md文件的内容: \n {}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.14函数名: `read_tar_md_by_line`

#### 接口说明

**描述**:  按行读取md文件内容

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的md文件的路径

**返回值**:  

成功：`Result<Vec<String>>`: 成功时返回一个字符串数组，每个字符串中包含指定md文件的内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_md_by_line("src/test_examples/test_tar.tar","test_md.md");
match res {
    Ok(content) => println!("tar文件中md文件的内容: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.15函数名: `read_tar_md_by_byte`

#### 接口说明

**描述**:  按字节读取 md文件

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的md文件的路径

**返回值**:  

成功：`Result<Vec<u8>>`: 成功时返回一个u8字节数组，包含指定md文件全部内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_md_by_byte("src/test_examples/test_tar.tar","test_md.md");
match res {
    Ok(content) => println!("tar文件中md文件的内容: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.16函数名: `read_tar_md_by_block`

#### 接口说明

**描述**:  按块读取md文件内容

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的md文件的路径
	`block_size:<usize>`: 每次读取的块大小，以字节为单位

**返回值**:  

成功：`Result<Vec<String>>`: 成功时返回指定Markdown文件内容的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_md_by_block("src/test_examples/test_tar.tar","test_md.md",4);
match res {
    Ok(content) => println!("tar文件中md文件的内容: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```



### 11.17函数名: `read_tar_excel`

#### 接口说明

**描述**:  读取整个excel文件内容

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的excel文件的路径

**返回值**:  

成功：`Result<Vec<String>>`: 成功时返回指定excel文件内容的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_excel("src/test_examples/test_tar.tar","test_excel.xlsx");
match res {
    Ok(content) => println!("tar文件中excel文件的内容: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.18函数名: `read_tar_pdf_as_string`

#### 接口说明

**描述**:  读取pdf文件内容

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的pdf文件的路径

**返回值**:  

成功：`Result<String>`: 成功时返回指定pdf文件内容的字符串，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_pdf_as_string("src/test_examples/test_tar.tar","test_pdf.pdf");
match res {
    Ok(content) => println!("tar文件中pdf文件的内容: \n {}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.19函数名: `read_tar_pptx_text_by_slide`

#### 接口说明

**描述**:  按行读取ppt文件内容

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的ppt文件的路径

**返回值**:  

成功：`Result<Vec<String>>`: 成功时返回指定pptx文件中提取的所有幻灯片文本内容，每一页一个向量，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_pptx_text_by_slide("src/test_examples/test_tar.tar","test_ppt.pptx");
match res {
    Ok(content) => println!("tar文件中ppt文件的内容: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

### 11.20函数名: `read_tar_pptx_notes`

#### 接口说明

**描述**:  读取ppt备注信息

**参数**:  
	`tar_path:<P: AsRef<Path>>`: tar文件路径
	`file_name:<&str>`: 要打开的ppt文件的路径

**返回值**:  

成功：`Result<Vec<String>>`: 成功时返回指定ppt文件备注的字符串数组，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

}

#### 调用示例

```rust
use FileContentReader::FileReader;
let res = FileReader::read_tar_pptx_notes("src/test_examples/test_tar.tar","test_ppt.pptx");
match res {
    Ok(content) => println!("tar文件中ppt文件的内容: \n {:?}", content),
    Err(e) => println!("读取tar文件失败: {}", e), 
}
```

## 12.读取文件信息

### 12.1函数名: `read_file_default`

#### 接口说明

**描述**:  会自动判断文件格式来读取内容

**参数**:  `file_path:<&str>` : 文件的路径

**返回值**: 

成功： `Result<Vec<Vec<String>>>` : 成功时返回文件的内容，失败时返回错误信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::read_file_default("src/test_examples/test_txt.txt");
match res {
    Ok(content) => println!("文件内容: \n {:?}", content),
    Err(e) => println!("读取文件失败: {}", e), 
}
 ```



### 12.2函数名: `get_file_size`

#### 接口说明

**描述**:  获取文件大小，单位字节

**参数**:  `file_path:<AsRef<Path>>`: 文件的路径

**返回值**:  

成功：`Result<u64>`: 返回文件的大小

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::get_file_size("src/test_examples/test_txt.txt");
match res {
    Ok(size) => println!("文件大小: \n {}", size),
    Err(e) => println!("读取文件失败: {}", e), 
}
 ```



### 12.3函数名: `get_file_created_time`

#### 接口说明

**描述**:  获取文件创建的时间

**参数**:  `file_path:<AsRef<Path>>`: 文件的路径

**返回值**:  

成功：`Result<String>`: 返回文件的创建时间的字符串

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::get_file_created_time("src/test_examples/test_txt.txt");
match res {
    Ok(content) => println!("文件创建时间: \n {}", content),
    Err(e) => println!("读取文件失败: {}", e), 
}
 ```



### 12.4函数名: `get_last_accessed_time`

#### 接口说明

**描述**:  获取上一次访问的时间

**参数**:  `file_path:<AsRef<Path>>`: 文件的路径

**返回值**:  

成功：`Result<String>`: 返回文件的上一次访问时间的字符串

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::get_last_accessed_time("src/test_examples/test_txt.txt");
match res {
    Ok(content) => println!("文件的上一次访问时间: \n {}", content),
    Err(e) => println!("读取文件失败: {}", e), 
}
 ```



### 12.5函数名: `get_last_modified`

#### 接口说明

**描述**:  获取文件最后修改时间

**参数**:  `file_path:<AsRef<Path>>`: 文件的路径

**返回值**:  

成功：`Result<String>`: 返回文件最后修改时间的字符串

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::get_last_modified("src/test_examples/test_txt.txt");
match res {
    Ok(content) => println!("文件的最后修改时间: \n {}", content),
    Err(e) => println!("读取文件失败: {}", e), 
}
 ```



### 12.6函数名: `is_directory`

#### 接口说明

**描述**:  获取文件是否为目录

**参数**:  `file_path:<AsRef<Path>>`: 文件的路径

**返回值**:  

成功：`Result<bool>`: 返回bool值,true表示是目录

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::is_directory("src/test_examples/test_txt.txt");
match res {
    Ok(content) => println!("文件是否为目录: \n {}", content),
    Err(e) => println!("读取文件失败: {}", e), 
}
 ```



### 12.7函数名: `is_regular_file`

#### 接口说明

**描述**:  获取文件是否为普通文件

**参数**:  `file_path:<AsRef<Path>>`: 文件的路径

**返回值**:  

成功：`Result<bool>`: 返回bool值,true表示是普通文件

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::is_regular_file("src/test_examples/test_txt.txt");
match res {
    Ok(content) => println!("文件是否为普通文件: \n {}", content),
    Err(e) => println!("读取文件失败: {}", e), 
}
 ```



### 12.8函数名: `parse_permissions`

#### 接口说明

**描述**:  解析文件权限位并返回一个表示权限的字符串

**参数**:  `attrs:<u32>`: 是一个 u32 类型的整数，表示文件的权限位

**返回值**:  

成功：`String`:返回一个字符串，表示文件的权限，格式为 "rwxrwxrwx"，其中每个字符代表读（r）、写（w）和执行（x）权限

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let permissions = FileReader::parse_permissions(0o755);
println!("文件权限: \n {}", permissions);
 ```



### 12.9函数名: `get_file_permission`

#### 接口说明

**描述**:  获取文件权限

**参数**:  `file_path:<AsRef<Path>>`: 文件的路径

**返回值**:  

成功：`Result<String>`: 返回表示文件权限的字符串

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::get_file_permission("src/test_examples/test_txt.txt");
match res {
    Ok(content) => println!("文件权限: \n {}", content),
    Err(e) => println!("读取文件失败: {}", e), 
}
 ```



### 12.10函数名: `get_file_extension`

#### 接口说明

**描述**:  获取文件扩展名

**参数**:  `file_path:<AsRef<Path>>`: 文件的路径

**返回值**:  

成功：`String`: 返回文件扩展名

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::get_file_extension("src/test_examples/test_txt.txt");
println!("文件扩展名: \n {}", res);
 ```



### 12.11函数名: `get_file_metadata`

#### 接口说明

**描述**:  读取文件的基本属性信息

**参数**:   `file_path:<AsRef<Path>>`: 文件的路径

**返回值**:  

成功：`Result<Metadata>`: 返回文件的基本属性信息

失败：文件路径错误，文件类型错误， 无法打开文件

#### 调用示例

 ```rust
use FileContentReader::FileReader;
let res = FileReader::get_file_metadata("src/test_examples/test_txt.txt");
match res {
    Ok(content) => println!("文件基本属性信息: \n {:?}", content),
    Err(e) => println!("读取文件失败: {}", e), 
}
 ```


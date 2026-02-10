pub mod test_txt_example; // 用于声明模块
pub mod test_csv_example;
pub mod test_json_example;
pub mod test_xml_example;
pub mod test_md_example;
pub mod test_pdf_example;
pub mod test_excel_example;
pub mod test_ppt_example;
pub mod test_zip_example;
pub mod test_rar_example;
pub mod test_tar_example;
/*
同一目录下使用mod引入文件:
    在同一目录下使用mod引入文件时，可以直接使用文件名。这是因为Rust编译器会在当前目录下查找以.rs结尾的文件，并将其视为模块。

引入另一个目录下的文件:  
    声明模块：在父目录的mod.rs文件中声明子目录作为一个模块。
    子目录中的mod.rs：在子目录中创建一个mod.rs文件，并声明您想要引入的文件作为模块的一部分。
    对于引入另一个目录下的文件，您需要先声明目录作为模块，并在该目录中的mod.rs文件中声明具体的文件。

为什么不能直接使用mod加上目录名和文件名:
    Rust的模块系统是基于目录结构的。当您使用mod引入一个目录时，您实际上是在声明一个模块，并告诉编译器去查找该目录下的mod.rs文件。
    因此，您需要在目录中有一个mod.rs文件来声明其他文件作为模块的一部分。
    例如，您不能在main.rs中直接写mod test_files::test_txt;，因为Rust编译器不知道如何解析这样的语法。
    正确的做法是先声明目录作为模块，然后在目录中的mod.rs文件中声明具体的文件。


*/

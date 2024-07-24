use digest::Digest;
use md5::Md5;
use std::{fs::File, io::Read};
use hex::encode;

#[allow(dead_code)]
/// 计算文件MD5值
pub fn calculate_file_md5(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let _f = match File::open(file_path) {
        Ok(_) => {
            //let _r = read_file(filename.to_string(), key.to_string(), begin, end, lineonly);
        }
        Err(_why) => {
            println!("文件({ })打开失败.", file_path);
            return Err(_why.into());
        }
    };
    // 打开文件
    let mut file = std::fs::File::open(file_path)?;
    // 创建MD5摘要器
    let mut md5_ctx = Md5::new();

    // 读取文件内容并更新MD5摘要
    let mut buffer = [0u8; 4096];
    
    loop {  
        match file.read(&mut buffer) { 
                Ok(size) if size > 0 => {  
                    md5_ctx.update(&buffer[..size]);  
                },  
                Ok(_) => break,  
                Err(err) => return Err(err.into()),  
            }  
    }
    // 获取最终的MD5哈希值
    let md5_hash = md5_ctx.finalize();
    // 将哈希值转换为十六进制字符串
    let md5_string = encode(&md5_hash);
    Ok(md5_string)
}

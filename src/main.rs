// read_file is  读取日志
extern crate ansi_term;
use ansi_term::Colour;
use std::fs::File;
//use std::io::prelude::*;
use std::io::{BufRead, BufReader};
// use std::fs::File;
#[macro_use]
extern crate clap;
extern crate libc;
use clap::{App, Arg, SubCommand};
use shadow_rs::shadow;
#[allow(dead_code)]
fn gen_value<T>(_: &T) {
    println!("类型： {}\n", std::any::type_name::<T>());
    Default::default()
}

//i32转[i8]
// fn i32Toi8(v: i32) -> [i8; 4] {
//     unsafe {
//         let i32Ptr: *const i32 = &v as *const i32;
//         let i8Ptr: *const i8 = i32Ptr as *const i8;
//         return [*i8Ptr.offset(0), *i8Ptr.offset(1), *i8Ptr.offset(2), *i8Ptr.offset(3)];
//     }
// }

/// 查询的关键词高亮显示
fn find_and_print(s: &str,key:&str) {
    use crossterm::style::Stylize;
    let mut current_index = 0;

    while let Some(next_index) = s[current_index..].find(&key) {
        let before = &s[current_index..current_index + next_index];
        let target = &s[current_index + next_index..current_index + next_index + key.len()];

        print!("{}", before);
        print!("{}", target.yellow().on_blue());

        current_index += next_index + key.len();
    }


    if current_index < s.len() {
        println!("{}", &s[current_index..]);
    }
}

#[allow(dead_code)]
/// 查询的关键词高亮显示
fn color_log(s: String, kyw: String) -> () {
    let line = s;
    use ansi_term::Colour::{Blue, Yellow};
    use ansi_term::Style;
    let l = kyw.len();
    match line.find(&kyw) {
        Some(start_bytes) => {
            let result = &line[start_bytes..start_bytes + l];
            print!("{}", &line[..start_bytes]);
            print!("{}", Style::new().on(Blue).fg(Yellow).paint(result));
            let subline = &line[start_bytes + l..];
            println!("{subline}")
            // color_log(subline.to_string(), kyw.to_string(), lineonly);
        }
        None => {
            println!("");
        }
    }
}
fn read_file(
    filename: String,
    kyw: String,
    begin: i32,
    end: i32,
    lineonly: bool,
) -> std::io::Result<()> {
    //let file = File::open(filename).unwrap();
    //let f = BufReader::new(file);

    use encoding::all::UTF_8;
    use encodingbufreader::BufReaderEncoding;
    let file = File::open(filename)?;
    let mut i = 1;
    // println!("开始:{}\t;结束:{}\n",begin,end);
    for line in BufReaderEncoding::new(file, UTF_8).lines() {
        if (begin == 0 && end == 0) || (i >= begin && i <= end) {
            if lineonly{
                let str_line=line.unwrap();
                match str_line.find(&kyw) {
                    Some(_) => {
                        let s = format!("{:08}", i);
                        print!("{}  ", Colour::Yellow.paint(s));
                        if kyw.len() == 0 {
                            println!("{}", str_line);
                        } else {
                            // color_log(str_line, kyw.to_string());
                            find_and_print(&str_line, &kyw);
                        }
                    }
                    None => {
                        println!("");
                    }
                }
            }else{
                let s = format!("{:08}", i);
                print!("{}  ", Colour::Yellow.paint(s));
                if kyw.len() == 0 {
                    println!("{}", line?);
                } else {
                    // color_log(line?, kyw.to_string());
                    find_and_print(&line.unwrap(), &kyw);
                }
            }
            
        }
        i = i + 1;
        if i > end && end != 0 {
            break;
        }
    }
    Ok(())
}

#[allow(dead_code)]
// Returns the path to the user's template directory.
//这个方法只能用于string类型，容易出现【stream did not contain valid UTF-8】
//works, but it keeps allocation a string for each line. Besides, if there is no line break on the input file, the whole file would be load to the memory.
fn read_file_old(filename: String, kyw: String, begin: i32, end: i32) -> std::io::Result<()> {
    let file = File::open(filename).unwrap();
    let fin = BufReader::new(file);
    let mut i = 1;
    for line in fin.lines() {
        if begin == 0 && end == 0 {
            let s = format!("{:07}", i);
            print!("{}  \t", Colour::Yellow.paint(s));
            if kyw.len() == 0 {
                gen_value(&line);
                println!(" {}", line.unwrap().to_string());
            } else {
                color_log(line.unwrap().to_string(), kyw.to_string());
            }
        } else {
            if i >= begin && i <= end {
                let s = format!("{:07}", i);
                print!("{}  \t", Colour::Yellow.paint(s));
                if kyw.len() == 0 {
                    println!(" {}", line.unwrap().to_string());
                } else {
                    color_log(line.unwrap().to_string(), kyw.to_string());
                }
            }
        }
        i = i + 1;
    }
    Ok(())
}

/// flag用于参数
fn flag() -> () {
    let s = "Version:".to_owned()
        + &crate_version!().to_owned()
        + "  git:"
        + &crate_description!().to_owned();
    // println!("{:?}",s);
    let matches = App::new("Sunnycat")
        .version(&*s)
        .author(crate_authors!())
        .about(
            "日志检索，增强版cat.
        例子:sunnycat --keyword example --file log.txt
                 ./sunnycat lines -r 5,10
        ",
        )
        .arg(
            Arg::with_name("detail")
                .short("d")
                .long("detail")
                // .multiple(false)
                // .takes_value(false)
                .help("Get detail program's info"),
        )
        .arg(
            Arg::with_name("lineonly")
                .short("l")
                .long("lineonly")
                // .multiple(false)
                // .takes_value(false)
                .help("只显示有关键字存在的行号"),
        )
        .args_from_usage("-k, --keyword=[KEYWORD] '搜索关键字'")
        .args_from_usage("-b, --bytekeyword=[BYTEKEYWORD] '搜索byte关键字'")
        .args_from_usage("-s,--str=[STRING]'转成中文'")
        .args_from_usage("-f ,--file=[FILE] 'Sets the input file to use'")
        // .subcommand(
        //     App::new("lineonly")
        //         .about("只显示有关键字存在的行号")
        //         .version(crate_version!())
        //         .args_from_usage("-l, --list '只显示能够查找到关键字的行号。'"),
        // )
        .subcommand(
            SubCommand::with_name("lines")
                .about("选择哪些行显示")
                .version(crate_version!())
                .author("Sunny Region. <jinheking@gmail.com>")
                .args_from_usage(
                    "-r --rows '输入行数，例如：-r 1,10,表示从第一行到第10行。'
                                    [LINES]  ' 1,10,表示从第一行到第10行。'",
                ),
        )
        .get_matches();

    let filename = matches.value_of("file").unwrap_or("log.txt");
    let key = matches.value_of("keyword").unwrap_or("");
    let bytekey = matches.value_of("bytekeyword").unwrap_or("");

    if bytekey.len() > 0 {
        let bytes = bytekey.as_bytes();
        for i in bytes {
            print!("\\\\{:o}", i);
        }
        println!("------{}", bytekey);
        use std::process;
        process::exit(0x0100);
    }
    /*
    \\345\\214\\227\\344\\272\\254\\357\\274\\214\\347\\256\\200\\347\\247\\260\\342\\200\\234\\344\\272\\254\\342\\200\\235\\357\\274\\214
      */
    let s_byte = matches.value_of("str").unwrap_or("");
    if s_byte.len() > 0 {
        let tokens: Vec<&str> = s_byte.split("\\").collect();
        let mut vec = Vec::new();
        for value in tokens.iter() {
            if value.len() > 0 {
                //let ss=value.parse::<i32>().unwrap();
                let ss = u8::from_str_radix(value, 8).unwrap();
                vec.push(ss);
            }
        }
        let sparkle_heart = String::from_utf8(vec).unwrap();
        println!("vec :{:?}", sparkle_heart);
        use std::process;
        process::exit(0x0100);
    }

    let detail = matches.is_present("detail");
    if detail {
        get_shadow();
        use std::process;
        process::exit(0x0100);
    }

    //  println!("{}",bLine);
    let mut lines: String = "0,0".to_string();
    if let Some(matches) = matches.subcommand_matches("lines") {
        if matches.is_present("rows") {
            lines = matches.value_of("LINES").unwrap_or("0,0").to_string();
        } else {
            println!("Printing normally...");
        }
    }
    let lineonly = matches.is_present("lineonly");
    
    // let mut lineonly: bool = false;
    // if let Some(matches) = matches.subcommand_matches("lineonly") {
    //     if matches.is_present("list") {
    //         lineonly = true;
    //         //     gen_value(&lineonly);
    //         //    print!(" _|￣|○ -----🎉🎉🎉👍💁👌 RUST{}  ⚽🎍😍🎉🎉🎉------○|￣|_  \n",lineonly);
    //         //    use std::process;
    //         //    process::exit(0x0100);
    //     }
    // }
    let l: Vec<&str> = lines.split(",").collect();
    let begin: i32 = l[0].parse().unwrap();
    let end: i32 = l[1].parse().unwrap();
    let _f = match File::open(filename) {
        Ok(_) => {
            let _r = read_file(filename.to_string(), key.to_string(), begin, end, lineonly);
        }
        Err(_why) => {
            println!("文件({ })打开失败.", filename);
        }
    };
}

// pub mod shadow {
//     include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
// }

pub fn get_shadow() {
    shadow!(build);
    print!("Name:{}\t", build::PROJECT_NAME); //shadow-rs
    print!("Author:Sunnyregion\t"); //
    println!("Email:jinheking@gmail.com"); //

    print!("Git branch:{}\t", build::BRANCH); //master
    print!("Git Version:{}\t", build::COMMIT_HASH); //
                                                    //println!("{}", build::SHORT_COMMIT); //
    println!("Git commit date:{}", build::COMMIT_DATE); //

    print!("OS:{}\t", build::BUILD_OS); //macos-x86_64
    print!("Rust version:{}\t", build::RUST_VERSION); //rustc 1.45.0 (5c1f21c3b 2020-07-13)
    println!("Channel:{}", build::RUST_CHANNEL); //stable-x86_64-apple-darwin (default)
    print!("Cargo Version:{}\t", build::CARGO_VERSION); //cargo 1.45.0 (744bd1fbb 2020-06-15)
    println!("PKG Version:{}", build::PKG_VERSION); //0.3.13
                                                    //    println!("{}",build::CARGO_LOCK);

    print!("Build Time:{}\t", build::BUILD_TIME); //2020-08-16 14:50:25
    println!("Build Rust Channel:{}", build::BUILD_RUST_CHANNEL); //debug
}
/// 主程序
fn main() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    flag();
}

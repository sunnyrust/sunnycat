# SunnyCat

SunnyCat is search log file tools

## help
    Sunnycat Version:0.3.6  git:A tool to search log files
    Sunny <jinheking@gmail.com>
    日志检索，增强版cat.
            例子:sunnycat --keyword example --file log.txt
                    ./sunnycat lines -r 5,10
            

    USAGE:
        sunnycat [FLAGS] [OPTIONS] [SUBCOMMAND]

    FLAGS:
        -d, --detail      Get detail program's info
        -h, --help        Prints help information
        -l, --lineonly    只显示有关键字存在的行号
        -m, --md5         Get the value of md5.
        -V, --version     Prints version information

    OPTIONS:
        -b, --bytekeyword <BYTEKEYWORD>    搜索byte关键字
        -f, --file <FILE>                  Sets the input file to use
        -k, --keyword <KEYWORD>            搜索关键字
        -s, --str <STRING>                 转成中文

    SUBCOMMANDS:
        help     Prints this message or the help of the given subcommand(s)
        lines    选择哪些行显示


# relesae note:
- 2024-07-24 暴雨 开发读取文件的md5方法。

# SunnyCat

SunnyCat is search log file tools

## help

Sunny <jinheking@gmail.com>
日志检索，增强版cat.
        例子:sunnycat --keyword example --file log.txt
                 ./sunnycat lines -r 5,10


USAGE:
    sunnycat [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --bytekeyword <BYTEKEYWORD>    搜索byte关键字
    -f, --file <FILE>                  Sets the input file to use
    -k, --keyword <KEYWORD>            搜索关键字
    -s, --str <STRING>                 转成中文

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    lineonly    只显示有关键字存在的行号
    lines       选择哪些行显示

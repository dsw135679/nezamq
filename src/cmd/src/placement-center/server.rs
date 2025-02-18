use clap::command;
use clap::Parser;


// 定义默认的配置路径，即当命令行没配置路径时，默认的配置文件路径
pub const DEFAULT_PLACEMENT_CENTER_CONFIG: &str= "config/placement-center.toml";

#[derive(Parser,Debug)]
#[command(author="nezamq",version="0.0.1",about=" NezaMQ: study rust project. ",long_about= None)]
#[command(next_line_help = true)]
struct ArgsParams{
    #[arg(short,long,default_value_t=String::from(DEFAULT_PLACEMENT_CENTER_CONFIG))]
    conf:String,
}


fn main(){
    let args=ArgsParams::parse();
    println!("conf path: {:?}",args.conf);
}

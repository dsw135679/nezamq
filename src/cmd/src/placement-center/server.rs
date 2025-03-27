use clap::Parser;
use clap::command;
use common_base::config::placement_center::init_placement_center_conf_by_path;
use common_base::config::placement_center::placement_center_conf;
use common_base::error::CommonError;
use common_base::log::placement_center::init_placement_center_log;
use log::info;
use tokio::select;
use tokio::sync::oneshot;

// 定义默认的配置路径，即当命令行没配置路径时，默认的配置文件路径
pub const DEFAULT_PLACEMENT_CENTER_CONFIG: &str = "config/placement-center.toml";
pub const DEFAULT_LOGGING_CONFIG: &str = "config/log4rs.yaml";

#[derive(Parser, Debug)]
#[command(author="nezamq",version="0.0.1",about=" NezaMQ: study rust project. ",long_about= None)]
#[command(next_line_help = true)]
struct ArgsParams {
  // 配置文件
  #[arg(short,long,default_value_t=String::from(DEFAULT_PLACEMENT_CENTER_CONFIG))]
  conf: String,
}

#[tokio::main]
async fn main() -> Result<(), CommonError> {
  let args = ArgsParams::parse();
  println!("conf path: {:?}", args.conf);
  // 1. 初始化配置文件
  init_placement_center_conf_by_path(&args.conf);

  // 2. 初始化日志
  init_placement_center_log();

  // 3. 获取配置
  let config = placement_center_conf();

  info!("{:?}", config);

  //start_server();
  let (tx1, rx1) = oneshot::channel();
  let (tx2, rx2) = oneshot::channel();

  tokio::spawn(async {
    let _ = tx1.send("one");
  });

  tokio::spawn(async {
    let _ = tx2.send("two");
  });

  select! {
    val= rx1=>{
      println!("rx1 completed first with {:?}",val);
    }
    val= rx2=>{
      println!("rx2 completed first with {:?}",val);
    }
  }

  Ok(())
}

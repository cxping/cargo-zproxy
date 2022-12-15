use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(name = "cargo-zproxy")]
#[command(bin_name = "cargo")]
#[command(author = "Cxping. <chping@iowk.com>")]
#[command(version = "0.1.0")]
#[command(about = "Switch Change image", long_about = None)]
#[command(propagate_version = true)]
pub enum Commands {
    /// list 获取当前可用镜像
    List {
        #[arg(short, long)]
        list: bool,
    },
    /// 自动选择一个镜像地址
    Auto {
        #[arg(short, long)]
        auto: bool,
    },
    /// 手动设置一个地址
    /// set -name xxx  
    ///     -id  xxx
    Use {
        #[arg(short, long)]
        name: String,
    },
    /// 手动添加一个地址
    /// add -name xxx -url xxxx
    Add{
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        source:String,
        #[arg(short, long)]
        url:String,
    },
    Init,
    //设置同步备份奖项地址
    /// Sync 
    /// -url xxx 设置仓库地址
    /// -repo  同步备份
    Sync{
        #[arg(short, long)]
        sync: String,
        #[arg(short, long)]
        repo:bool,
    }
}

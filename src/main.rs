use std::{env, io, ffi::OsString};
use std::path::PathBuf;
use args::Commands;
use clap::Parser;
use log::Level;
use log::debug;
pub mod args;

struct  CargoConfig{
    cargo_dir:PathBuf,
}

#[tokio::main]
async fn main()->io::Result<()> {
    let  args = std::env::args_os();
    let  mut iter:Vec<OsString> = args.into_iter().collect();
    if iter.len() > 1 && iter[1] == "zproxy" {
		iter.remove(1);
	}
    //解析指令
    let app_args= Commands::parse_from(iter);
    match app_args {
        // list  获取前配置项中镜像仓库
        Commands::List { list } => {
            println!("-- list ");
        },
        // auto  自动切换一个镜像仓库
        Commands::Auto { auto } => {
            println!("-- Auto ");
        },
        // Use --name  xxx  切换指定名称得镜像仓库
        Commands::Use { name } => {
            println!("Add -->Set {}",name);
        }
        // --name : 镜像名称（非必须）
        // --source: 仓库标识--唯一标识
        // --url : 仓库地址
        Commands::Add { name,url,source} => {
            println!("Add -->{},{},{} ",name,url,source);
        }
        // sync 同步镜像配置
        // --url xxx 设置镜像源地址
        // --repo   下载镜像配置
        Commands::Sync { sync,repo } => {
            println!("sync ");
        }

        _=>{
            println!("无法指令参数")
        }
    };
    Ok(())
}
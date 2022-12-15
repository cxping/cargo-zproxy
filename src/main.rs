use args::Commands;
use clap::Parser;
use dirs::home_dir;
use std::{error, fs};
use std::path::PathBuf;
use std::{env, ffi::OsString, io};
use std::fs::File;
use std::io::{BufWriter, Error, ErrorKind, Write};
use crate::config::ZProxy;

pub mod args;
pub mod config;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = std::env::args_os();
    let mut iter: Vec<OsString> = args.into_iter().collect();
    if iter.len() > 1 && iter[1] == "zproxy" {
        iter.remove(1);
    }
    let app_args = Commands::parse_from(iter);
    let mut cargo_dir = cargo_dir();
    match app_args {
        // list  获取前配置项中镜像仓库
        Commands::List { list: _ } => {
            zproxy_list(&mut cargo_dir);
        }
        // auto  自动切换一个镜像仓库
        Commands::Auto { auto: _ } => {
            println!("auto 命令暂未实现");
        }
        // Use --name  xxx  切换指定名称得镜像仓库
        Commands::Use { name } => {
            println!("use 命令暂未实现,");
        }
        // --name : 镜像名称（非必须）
        // --source: 仓库标识--唯一标识
        // --url : 仓库地址
        Commands::Add { name, url, source } => {
            println!("add:命令暂未实现,");
        }
        // sync 同步镜像配置
        // --url xxx 设置镜像源地址
        // --repo   下载镜像配置
        Commands::Sync { sync: _, repo: _ } => {
            println!("sync:命令暂未实现");
        }
        Commands::Init=>{
            zproxy_init(&mut cargo_dir,false);
        }
        _ => {
            println!("未知指令，无法执行")
        }
    };
    Ok(())
}

fn print_list(morris: &Vec<config::Mirror>) {
    for (i, k) in morris.iter().enumerate() {
        output_string(i, k.name.as_str(), k.source.as_str());
    }
}
/// 打印镜像配置内容
fn output_string(index: usize, name: &str, source: &str) {
    if index == 0 {
        println!("\n");
        println!("以下镜像不分排名先后");
        println!("指定镜像地址use --name  源");
        println!("索引\t\t\t\t源\t\t\t\t名称");
    }
    println!("{}\t\t\t\t{}\t\t\t\t{}", index, source, name);
}

//copy  cargo-update
fn cargo_dir() -> PathBuf {
    match env::var("CARGO_INSTALL_ROOT")
        .map_err(|_| ())
        .and_then(|ch| fs::canonicalize(ch).map_err(|_| ()))
    {
        Ok(ch) => ch,
        Err(()) => match env::var("CARGO_HOME")
            .map_err(|_| ())
            .and_then(|ch| fs::canonicalize(ch).map_err(|_| ()))
        {
            Ok(ch) => ch,
            Err(()) => match home_dir().and_then(|hd| hd.canonicalize().ok()) {
                Some(mut hd) => {
                    hd.push(".cargo");
                    fs::create_dir_all(&hd).unwrap();
                    hd
                }
                None => {
                    panic!("cargo-dir无法获取")
                }
            },
        },
    }
}
/// 初始化指令
/// zproxy_init
fn zproxy_init(cargo_dir:&mut PathBuf,force:bool){
    let  zproxy_conf_path = cargo_dir;
    zproxy_conf_path.push(".zproxy.json");
    let mut file =match  fs::File::open(&zproxy_conf_path) {
        Ok(f) => {
            if !force{
                panic!("无需重复初始化配置项，如继续请添加--f进行强制初始化");
            }
            f
        }
        Err(e) => {
        let f=      match e.kind() {
                ErrorKind::NotFound => {
                    File::create(&zproxy_conf_path).unwrap()
                }
                _=>{
                    panic!("初始化失败")
                }
            };
            f
        }
    };
    //检查是否已经存在config配置文件，如果存在则需要加载或者覆盖
    let zproxy_conf = ZProxy::default();
    if   let Some(conf) = serde_json::to_string(&zproxy_conf).ok(){
        file.write_all(conf.as_bytes()).expect("配置文件写入失败");
    }else{
        println!("配置文件写入失败")
    }
}

/// list 指令
/// 查看所有镜像源配置
fn zproxy_list(cargo_dir:&mut PathBuf){
    let mut  zproxy_conf_path = cargo_dir;
    zproxy_conf_path.push(".zproxy.json");
    let zproxy_conf = match fs::canonicalize(zproxy_conf_path) {
        Ok(ch) => ch,
        Err(_) => {
            //cargo zproxy sync 同步远程镜像配置文件,或者
            panic!("请先执行cargo zproxy init 初始化内置镜像配置")
        }
    };
    let zproxy_json = config::ZProxy::form_file(&zproxy_conf);
    let zeporxy = match zproxy_json {
        Some(zp) => zp,
        None => config::ZProxy::default(),
    };
    let mirros = zeporxy.mirrors;
    print_list(&mirros);
}

//用户切换镜像地址
fn zproxy_use(){

}
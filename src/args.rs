use crate::config::{self, ZProxy, Mirror};
use clap::Parser;
use dirs::home_dir;
use std::{
    env,
    fs::{self, File},
    io::{ErrorKind, Write},
    path::{PathBuf, Path},
};

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
        source: String,
    },
    /// 手动添加一个地址
    /// add -name xxx -url xxxx
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        source: String,
        #[arg(short, long)]
        url: String,
    },
    Init,
    //设置同步备份奖项地址
    /// Sync
    /// -url xxx 设置仓库地址
    /// -repo  同步备份
    Sync {
        #[arg(short, long)]
        sync: String,
        #[arg(short, long)]
        repo: bool,
    },
    // def 切换为使用官方默认镜像crates.io
    Def,
}

pub fn process(cmd: Commands) {
    let mut cargo_dir = cargo_dir();
    let zproxy = read_zproxy(cargo_dir.clone());
    
    match cmd {
        // list  获取前配置项中镜像仓库
        Commands::List { list: _ } => {
            if let Some(z) = zproxy {
                zproxy_list(z);
            }
        }
        // auto  自动切换一个镜像仓库
        Commands::Auto { auto: _ } => {
            println!("auto 命令暂未实现");
        }
        // Use --name  xxx  切换指定名称得镜像仓库
        Commands::Use { source } => {
            if let Some(z) = zproxy {
                zproxy_use(source, z, &mut cargo_dir);
            }
        }
        // --name : 镜像名称（非必须）
        // --source: 仓库标识--唯一标识
        // --url : 仓库地址
        Commands::Add { name, url, source } => {
            if url.is_empty() || source.is_empty(){
                panic!("请添使用cargo zproxy add --help 查看命令帮助信息")
            }

            if let Some(z) = zproxy {
                let mirror = Mirror::new(name, source, url);
                zproxy_add(z,mirror, &mut cargo_dir);
            }
    
        }
        // sync 同步镜像配置
        // --url xxx 设置镜像源地址
        // --repo   下载镜像配置
        Commands::Sync { sync: _, repo: _ } => {
            println!("sync:命令暂未实现");
        }
        Commands::Init => {
            zproxy_init(&mut cargo_dir, false);
        }

        Commands::Def=>{
            if let Some(z) = zproxy {
                zproxy_use("default".to_owned(), z, &mut cargo_dir)
            } 
        },
    
    };
}

fn print_list(morris: &Vec<config::Mirror>) {
    for (i, k) in morris.iter().enumerate() {
        output_string(i, k.name.as_str(), k.source.as_str());
    }
}
/// 打印镜像配置内容
fn output_string(index: usize, name: &str, source: &str) {
    if index == 0 {
        println!("以下镜像不分排名先后");
        println!("指定镜像地址use --name  源");
        println!("索引(index)\t\t\t\t源(source)\t\t\t\t名称(name)");
    }
    println!("{}\t\t\t\t{}\t\t\t\t{}", index, source, name);
}

/// 获取cargo安装目录
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
fn zproxy_init(cargo_dir: &mut PathBuf, force: bool) {
    let zproxy_conf_path = cargo_dir;
    zproxy_conf_path.push(".zproxy.json");
    let mut file = match fs::File::open(&zproxy_conf_path) {
        Ok(f) => {
            if !force {
                panic!("无需重复初始化配置项，如继续请添加--f进行强制初始化");
            }
            f
        }
        Err(e) => {
            let f = match e.kind() {
                ErrorKind::NotFound => File::create(&zproxy_conf_path).unwrap(),
                _ => {
                    panic!("初始化失败")
                }
            };
            f
        }
    };
    //检查是否已经存在config配置文件，如果存在则需要加载或者覆盖
    let zproxy_conf = ZProxy::default();
    if let Some(conf) = serde_json::to_string(&zproxy_conf).ok() {
        file.write_all(conf.as_bytes()).expect("配置文件写入失败");
    } else {
        println!("配置文件写入失败")
    }
}

/// list 指令
/// 查看所有镜像源配置
fn zproxy_list(zproxy: ZProxy) {
    print_list(&zproxy.mirrors);
}

//[source.crates-io]
// registry = "https://github.com/rust-lang/crates.io-index"
// # 指定镜像
// replace-with = 'sjtu' # 如：tuna、sjtu、ustc，或者 rustcc
//用户切换镜像地址
fn zproxy_use(name: String, zproxy: ZProxy, cargo_dir: &mut PathBuf) {
    let config_file_path = &mut *cargo_dir;
    config_file_path.push("config");
    if name.eq("default") {
        let conf = format!(
            r#"[source.crates-io]
registry  = 'https://github.com/rust-lang/crates.io-index'
"#);
match fs::write(&mut *config_file_path, conf) {
    Ok(_) => {
        println!("切换成功==>default:https://github.com/rust-lang/crates.io-index");
        return;
    }
    Err(e) => {
        println!("{:?}", e)
    }
}
  
 };

    let mirrors = zproxy.mirrors;
    for mirror in mirrors {
        if mirror.source.eq(&name) {
            //找到对应的镜像内容

            //合成toml文件
            //1：装配数据
            let conf = format!(
                r#"[source.crates-io]
replace-with = '{}'

[registries.{}]
index = '{}'

git-fetch-with-cli = true

check-revoke =  false
"#,
                mirror.source, mirror.source, mirror.registry
            );
            match fs::write(&mut *config_file_path, conf) {
                Ok(_) => {
                    println!("切换成功==>{}", mirror.source);
                    break;
                }
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
    }
}

fn _help() {
    let help = r#"
  cargo zproxy  init              执行本地初始化
  cargo zproxy  auto              评估网络延迟并自动切换到最优的镜像
  cargo zproxy  use <source>        切换为要使用的镜像
  cargo zproxy  list              查看当前所有配置的镜像资源地址
  cargo zproxy  add  -source<XX> --name<XX> -url xxxx
                                  添加一个镜像进本地配置文件
  cargo zproxy  sync              同步镜像配置文件
                  --repo          设置私有配置仓库
                  --push          同步镜像配置文件
  cargo zproxy  version           查看当前版本
  cargo zproxy  def               设置默认官方镜像
"#;
    println!("{}", help);
}

// 读取配置文件
fn read_zproxy(cargo_dir: PathBuf) -> Option<ZProxy> {
    let mut zproxy_conf_path = cargo_dir;
    zproxy_conf_path.push(".zproxy.json");
    let zproxy_conf = match fs::canonicalize(zproxy_conf_path) {
        Ok(ch) => ch,
        Err(_) => {
            //cargo zproxy sync 同步远程镜像配置文件,或者
            return None;
        }
    };
    config::ZProxy::form_file(&zproxy_conf)
}

//添加源进入到本地配置文件
fn zproxy_add(mut zproxy: ZProxy,mirror: Mirror, cargo_dir: &mut PathBuf){
    let mirrors =  zproxy.mirrors.iter().filter(|s|{
        s.registry.eq(&mirror.registry)
    }).collect::<Vec<&Mirror>>();
    if mirrors.len() >=1 {
        println!("当前源已添加,无需重复添加");
       return;
    }

    let zproxy_conf_path = cargo_dir;
    zproxy_conf_path.push(".zproxy.json");
    let  zp_clone  = zproxy_conf_path.clone();
    let mut file = if Path::new(zproxy_conf_path).exists(){
        fs::remove_file(zp_clone).unwrap();
        File::create(&zproxy_conf_path).unwrap()
    }else{
         File::create(&zproxy_conf_path).unwrap()
    };
    zproxy.mirrors.push(mirror);
    print_list(&zproxy.mirrors);
    //检查是否已经存在config配置文件，如果存在则需要加载或者覆盖
    if let Some(conf) = serde_json::to_string(&zproxy).ok() {
        match file.write_all(conf.as_bytes()) {
            Ok(()) => {},
            Err(e) => {
               println!("文件写入失败：{:?}",e);
            },
        }
    } else {
        println!("配置文件写入失败")
    }
}
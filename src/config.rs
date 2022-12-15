use std::{io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ZProxy {
    pub version: String,
    pub default: String,
    pub mirrors: Vec<Mirror>,
    //其他配置想
    pub git_fetch_with_cli: bool,
    pub check_revoke: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mirror {
    pub name: String,
    pub source: String,
    pub registry: String,
}

impl ZProxy {
    //加载配置文件中的数据
    pub fn form_file(cnf: &PathBuf) -> Option<Self> {
        let file = std::fs::File::open(cnf).unwrap();
        let reader = BufReader::new(file);
        let conf_data: Result<ZProxy, serde_json::Error> = serde_json::from_reader(reader);
        return match conf_data {
            Ok(conf) => Some(conf),
            Err(e) => panic!("{}",e),
        };
    }
}

impl Default for ZProxy {
      fn default() -> Self {
        ZProxy {
            version: "0".to_string(),
            default: "create-io".to_string(),
            mirrors: vec![
                Mirror {
                    name: "crates".to_string(),
                    source: "crates-io".to_string(),
                    registry: "https://github.com/rust-lang/crates.io-index.git".to_string(),
                },
                Mirror {
                    name: "清华大学开源软件镜像站".to_string(),
                    source: "tuna".to_string(),
                    registry: "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
                        .to_string(),
                },
                Mirror {
                    name: "北京外国语大学开源软件镜像站".to_string(),
                    source: "bfsu".to_string(),
                    registry: "https://mirrors.bfsu.edu.cn/git/crates.io-index.git"
                        .to_string(),
                },
                Mirror {
                    name: "浙江大学开源软件镜像站".to_string(),
                    source: "zju".to_string(),
                    registry: "https://mirrors.zju.edu.cn/git/crates.io-index.git"
                        .to_string(),
                },
                Mirror {
                    name: "哈尔滨工业大学开源软件镜像站".to_string(),
                    source: "hit.edu".to_string(),
                    registry: "https://mirrors.hit.edu.cn/crates.io-index.git"
                        .to_string(),
                },
                Mirror {
                    name: "中科大USTC".to_string(),
                    source: "ustc".to_string(),
                    registry: "https://mirrors.ustc.edu.cn/crates.io-index"
                        .to_string(),
                }
            ],
            git_fetch_with_cli: true,
            check_revoke: false,
        }
    }
}

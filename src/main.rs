use std::{env, io, ffi::OsString};
use args::AppArgs;
use clap::Parser;

pub mod args;

#[tokio::main]
async fn main()->io::Result<()> {
    let  args = std::env::args_os();
    //ArgsOs { inner: ["D:\\chping\\.cargo\\bin\\cargo-zproxy.exe", "zproxy", "-l"] }
    //解析指令
    let  mut iter:Vec<OsString> = args.into_iter().collect();
    if iter.len() > 1 && iter[1] == "zproxy" {
		iter.remove(1);
	}
    //解析指令
    let app_args= AppArgs::parse_from(iter);
    app_args.parse_args();
    Ok(())
}
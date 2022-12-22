use args::{process,Commands};
use clap::Parser;
use dirs::home_dir;
use std::{env, ffi::OsString, io::{self,Write,ErrorKind},path::PathBuf,fs::{self,File}};
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
    process(app_args);
    Ok(())
}


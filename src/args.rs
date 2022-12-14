use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(name = "cargo-zproxy")]
#[command(author = "Cxping. <chping@iowk.com>")]
#[command(version = "0.1.0")]
#[command(about = "Switch Change image", long_about = None)]
#[command(propagate_version = true)]
pub struct  AppArgs{
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand,Debug,Clone)]
pub enum Commands {
    /// does testing things
    List{
        #[arg(short, long)]
        list: bool,
    }
}

impl  AppArgs{
    pub fn parse_args(&self){
        match  &self.command {
            Some(command) => {
                println!("这是一个参数{:?}",command);
            },
            None =>{
                println!("未知参数");
            },
        }
    }
}
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "Pyschedelic image generator", about = "Generate random psychedelic images")]
pub struct Opt {
    #[structopt(short = "s", long = "size", default_value = "300")]
    pub size: u32,

    #[structopt(short = "n", long = "num", default_value = "5")]
    pub num: usize,

    #[structopt(short = "p", long = "path", default_value = "images")]
    pub path: PathBuf
}
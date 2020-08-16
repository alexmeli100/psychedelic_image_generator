use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "Pyschedelic image generator", about = "Generate random psychedelic images")]
pub struct Opt {
    #[structopt(short = "s", long = "set-wallpaper")]
    pub wallpaper: bool,

    #[structopt(short = "w", long = "width", default_value = "1920")]
    pub width: u32,

    #[structopt(short = "h", long = "height", default_value = "1080")]
    pub height: u32,

    #[structopt(short = "n", long = "num", default_value = "10")]
    pub num: usize,

    #[structopt(parse(from_os_str), short = "p", long = "path", default_value = "images")]
    pub path: PathBuf
}
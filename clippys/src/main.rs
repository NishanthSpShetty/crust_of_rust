use clap::{App, Arg};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CmdArgs {
    #[structopt(short, long)]
    config: String,
    #[structopt(short, long)]
    name: String,
    #[structopt(short, long)]
    version: String,
    #[structopt(short, long)]
    size: i64,
}

fn main() {
    //    let matches = App::new("clippys")
    //        .version("0.1")
    //        .author("NishanthSpShetty")
    //        .arg(
    //            Arg::with_name("file")
    //                .help("file to be compiled")
    //                .short("f")
    //                .long("file")
    //                .takes_value(true),
    //        )
    //        .get_matches();
    //
    //    let file = matches.value_of("file").unwrap();
    //    println!("{}, \nfile {}", matches.usage(), file);

    let cmd = CmdArgs::from_args();
    println!("{:?}", cmd);
}

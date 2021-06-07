use structopt::StructOpt;


#[derive(StructOpt)]
#[allow(dead_code)]
struct Cli {
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[allow(unused_variables)]
fn main() {
    let args = Cli::from_args();
}

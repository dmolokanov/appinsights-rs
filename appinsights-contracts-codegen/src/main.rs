use std::path::PathBuf;

use structopt::StructOpt;

use appinsights_contracts_codegen::compiler;

fn main() {
    let opts = Opt::from_args();
    if let Err(err) = compiler::compile_all(opts.input_dir, opts.output_dir) {
        eprintln!("{}", err)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
pub struct Opt {
    /// A path to directory with all schema files
    #[structopt(parse(from_os_str), short = "i", long = "input-dir")]
    input_dir: PathBuf,

    /// A path to directory to output generate data contract files to
    #[structopt(parse(from_os_str), short = "o", long = "output-dir")]
    output_dir: PathBuf,
}

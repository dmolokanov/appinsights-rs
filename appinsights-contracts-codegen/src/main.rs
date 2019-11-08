use std::path::PathBuf;

use structopt::StructOpt;

use appinsights_contracts_codegen::bond::Compiler;
use appinsights_contracts_codegen::Result;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err)
    }
}

fn run() -> Result<()> {
    let opts = Opt::from_args();

    let compiler = Compiler::new();
    compiler.compile_all(&opts.input_dir, &opts.output_dir)?;

    Ok(())
}

//fn process_schema(path: &Path, output_dir: &Path) -> Result<()> {
//    let dest_path = path
//        .file_stem()
//        .and_then(|stem| stem.to_str())
//        .map(|stem| format!("{}.rs", stem.to_lowercase()))
//        .map(|filename| output_dir.join(filename))
//        .ok_or("Unable to get a file name")?;
//
//    //    generate(&schema, &dest_path)?;
//    let compiler = Compiler::new();
//    compiler.compile(path, output_dir)?;
//
//    Ok(())
//}

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

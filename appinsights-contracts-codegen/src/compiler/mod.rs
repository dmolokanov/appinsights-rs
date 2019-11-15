mod generator;
mod module;
mod visitor;

pub use module::Module;
pub use visitor::Visitor;

use std::convert::TryFrom;
use std::fs;
use std::path::{Path, PathBuf};

use crate::compiler::generator::{PackageGenerator, SchemaGenerator};
use crate::parser::Parser;
use crate::Result;

pub fn compile_all(input_dir: PathBuf, output_dir: PathBuf) -> Result<()> {
    let mut modules: Vec<_> = fs::read_dir(&input_dir)?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .map(|path| Module::try_from((path, output_dir.clone())).expect("unable to read module path"))
        .collect();
    modules.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    compile_files(modules.iter())?;
    compile_package(modules.iter(), &output_dir.join("mod.rs"))?;

    Ok(())
}

fn compile_files<'a>(modules: impl Iterator<Item = &'a Module>) -> Result<()> {
    for module in modules {
        if let Err(err) = compile(module) {
            eprintln!("{}: {}", module.file_name(), err);
        } else {
            println!("{}: ok", module.file_name());
        }
    }

    Ok(())
}

fn compile(module: &Module) -> Result<()> {
    let parser = Parser::default();
    let schema = parser.parse(&module.source_path())?;

    let mut generator = SchemaGenerator::new();
    generator.visit_schema(&schema);

    fs::write(&module.path(), generator.to_string())?;
    Ok(())
}

fn compile_package<'a>(modules: impl Iterator<Item = &'a Module>, path: &Path) -> Result<()> {
    let mut generator = PackageGenerator::new();
    for module in modules {
        generator.visit_module(module);
    }

    fs::write(path, generator.to_string())?;
    Ok(())
}

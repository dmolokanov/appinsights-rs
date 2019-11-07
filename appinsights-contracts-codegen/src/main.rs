use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err)
    }
}

fn run() -> Result<()> {
    let opts = Opt::from_args();

    let files = fs::read_dir(&opts.input_dir)?.filter_map(|entry| entry.ok().map(|entry| entry.path()));
    for file in files.filter(|file| file.ends_with("Envelope.json")).take(1) {
        process_schema(&file, &opts.output_dir)?;
    }

    Ok(())
}

fn process_schema(path: &Path, _output_dir: &Path) -> Result<()> {
    let schema: Schema = serde_json::from_reader(File::open(&path)?)?;
    dbg!(schema);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Schema {
    namespaces: Vec<Namespace>,
    imports: Vec<String>,
    declarations: Vec<Declaration>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "tag")]
#[serde(deny_unknown_fields)]
enum Declaration {
    Struct(Struct),
    Enum(Enum),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Struct {
    struct_base: Option<StructBase>,
    struct_fields: Vec<Field>,

    decl_params: Vec<DeclarationParam>,
    decl_name: String,
    decl_attributes: Vec<Attribute>,
    decl_namespaces: Vec<Namespace>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct StructBase {
    declaration: Box<Declaration>,
    type_: StructBaseType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
enum StructBaseType {
    User,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Field {
    field_modifier: FieldModifier,
    field_default: Option<FieldDefault>,
    field_type: FieldType,
    field_name: String,
    field_attributes: Vec<Attribute>,
    field_ordinal: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
enum FieldModifier {
    Optional,
    Required,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct FieldDefault {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(deny_unknown_fields)]
enum FieldType {
    // basic types
    Bool,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Double,
    String,
    WString,
    // todo container types blob, list<T>, vector<T>, set<T>, map<K, T>, nullable<T>.
    //    Map {
    //        #[serde(rename = "type")]
    //        type_: String,
    //        key: Box<FieldType>,
    //        element: Box<FieldType>,
    //    },
    // todo user-defined types enum, struct or bonded<T> where T is a struct
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Attribute {
    attr_name: Vec<String>,
    attr_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct DeclarationParam {
    param_constraint: Option<String>,
    param_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Namespace {
    name: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Enum {}

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

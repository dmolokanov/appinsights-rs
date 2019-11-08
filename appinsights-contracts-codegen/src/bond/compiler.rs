use crate::bond::*;
use crate::Result;
use codegen::Scope;
use heck::SnakeCase;
use std::fs;
use std::path::Path;

trait Visitor<T> {
    type Result;

    fn visit(&self, item: T) -> Self::Result;
}

pub struct Compiler {
    //    generator: CodeGenerator,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
//            generator: CodeGenerator,
        }
    }

    pub fn compile_all(&self, input_dir: &Path, output_dir: &Path) -> Result<()> {
        let files = fs::read_dir(&input_dir)?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|file| 1 == 0 || file.ends_with("Envelope.json"));

        let mut package = codegen::Scope::new();

        for input in files {
            let stem = input
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|stem| stem.to_lowercase())
                .ok_or("Unable to get a file name")?;

            let output = output_dir.join(format!("{}.rs", stem));

            self.compile(&input, &output)?;

            package.raw(&format!("mod {};", stem));
            package.raw(&format!("pub use {}::*;", stem));
        }

        let package_path = output_dir.join("mod.rs");
        fs::write(&package_path, package.to_string())?;

        Ok(())
    }

    pub fn compile(&self, input: &Path, output: &Path) -> Result<()> {
        let parser = Parser::new();
        let schema = parser.parse(input)?;

        let generator = CodeGenerator;
        let module = generator.visit(schema);

        fs::write(output, module.to_string())?;
        Ok(())
    }
}

struct CodeGenerator;

impl Visitor<Schema> for CodeGenerator {
    type Result = Scope;

    fn visit(&self, item: Schema) -> Self::Result {
        let mut module = Scope::new();

        for declaration in item.declarations {
            match declaration {
                UserTypeDeclaration::Struct(struct_) => {
                    let (struct_, impl_) = self.visit(struct_);
                    module.push_struct(struct_);
                    module.push_impl(impl_);
                }
                UserTypeDeclaration::Enum(enum_) => {
                    let enum_ = self.visit(enum_);
                    module.push_enum(enum_);
                }
            };
        }

        module
    }
}

impl Visitor<Struct> for CodeGenerator {
    type Result = (codegen::Struct, codegen::Impl);

    fn visit(&self, item: Struct) -> Self::Result {
        let mut struct_: codegen::Struct = codegen::Struct::new(&item.decl_name);
        struct_.vis("pub");

        let mut impl_ = codegen::Impl::new(&item.decl_name);

        let mut block = codegen::Block::new("Self");
        let constructor = impl_.new_fn("new").vis("pub").ret("Self");

        for field in item.struct_fields {
            let visitor = FieldTypeVisitor;
            let field_type = visitor.visit(field.field_type);

            let field_type = match field.field_modifier {
                FieldModifier::Optional => {
                    let mut type_ = codegen::Type::new("Option");
                    type_.generic(field_type);
                    type_
                }
                FieldModifier::Required => field_type,
            };

            let field_name = field.field_name.to_snake_case();
            struct_.field(&field_name, &field_type);
            constructor.arg(&field_name, &field_type);
            block.line(field_name);
        }

        constructor.push_block(block);

        if let Some(doc) = self.visit(item.decl_attributes) {
            struct_.doc(&doc);
        }

        (struct_, impl_)
    }
}

struct FieldTypeVisitor;

impl Visitor<Type> for FieldTypeVisitor {
    type Result = codegen::Type;

    fn visit(&self, item: Type) -> Self::Result {
        match item {
            Type::Basic(type_) => type_.into(),
            Type::Complex(type_) => type_.into(),
        }
    }
}

//impl<T:AsRef<BasicType>> Into<codegen::Type> for T{
impl Into<codegen::Type> for BasicType {
    fn into(self) -> codegen::Type {
        let name = match self {
            BasicType::Bool => "bool",
            BasicType::UInt8 => "u8",
            BasicType::UInt16 => "u16",
            BasicType::UInt32 => "u32",
            BasicType::UInt64 => "u64",
            BasicType::Int8 => "i8",
            BasicType::Int16 => "i16",
            BasicType::Int32 => "i32",
            BasicType::Int64 => "i64",
            BasicType::Float => "f32",
            BasicType::Double => "f64",
            BasicType::String => "String",
            BasicType::WString => "String",
        };

        codegen::Type::new(name)
    }
}

impl Into<codegen::Type> for ComplexType {
    fn into(self) -> codegen::Type {
        match self {
            ComplexType::Map { key, element } => {
                // TODO import HashMap
                let mut type_ = codegen::Type::new("HashMap");
                type_.generic(key).generic(element);
                type_
            }
            //            ComplexType::Parameter { .. } => {}
            //            ComplexType::Vector { .. } => {}
            //            ComplexType::Nullable { element } => {
            //                let mut type_ = codegen::Type::new("Options");
            //                type_.generic(element);
            //                type_
            //            }
            //            ComplexType::User { .. } => {}
            _ => panic!("{:?}", self),
        }
    }
}

impl Visitor<Enum> for CodeGenerator {
    type Result = codegen::Enum;

    fn visit(&self, item: Enum) -> Self::Result {
        let mut enum_ = codegen::Enum::new(&item.decl_name);
        enum_.vis("pub");

        for const_ in item.enum_constants {
            enum_.new_variant(&const_.constant_name);

            if let Some(_) = &const_.constant_value {
                panic!("enum value is not supported: {:#?}", const_)
            }
        }

        if let Some(doc) = self.visit(item.decl_attributes) {
            enum_.doc(&doc);
        }

        enum_
    }
}

impl Visitor<Vec<Attribute>> for CodeGenerator {
    type Result = Option<String>;

    fn visit(&self, items: Vec<Attribute>) -> Self::Result {
        items.into_iter().filter_map(|attr| self.visit(attr)).find(|_| true)
    }
}

impl Visitor<Attribute> for CodeGenerator {
    type Result = Option<String>;

    fn visit(&self, item: Attribute) -> Self::Result {
        if item.attr_name.iter().any(|name| name == "Description") {
            Some(item.attr_value)
        } else {
            None
        }
    }
}

//struct UserTypeDeclarationVisitor;
//
//impl Visitor<UserTypeDeclaration> for UserTypeSeclarationVisitor {
//    type Result = ();
//
//    fn visit(&self, item: &UserTypeDeclaration) -> Self::Result {
//        unimplemented!()
//    }
//}

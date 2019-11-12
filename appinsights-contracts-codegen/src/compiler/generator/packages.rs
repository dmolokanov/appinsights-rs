use crate::compiler::Module;

pub struct PackageGenerator {
    modules: Vec<String>,
    usages: Vec<String>,
}

impl PackageGenerator {
    pub fn new() -> Self {
        Self {
            modules: Vec::default(),
            usages: Vec::default(),
        }
    }

    pub fn visit_module(&mut self, module: &Module) {
        self.modules.push(format!("mod {};", module.name()));
        self.usages.push(format!("pub use {}::*;", module.name()));
    }
}

impl ToString for PackageGenerator {
    fn to_string(&self) -> String {
        codegen::Scope::new()
            .raw("// NOTE: This file was automatically generated.")
            .raw("#![allow(unused_variables, dead_code, unused_imports)]")
            .raw(&self.modules.join("\n"))
            .raw(&self.usages.join("\n"))
            .push_trait(telemetry_data_trait())
            .to_string()
    }
}

fn telemetry_data_trait() -> codegen::Trait {
    let mut telemetry_data = codegen::Trait::new("TelemetryData");
    telemetry_data
        .vis("pub")
        .doc("Common interface implemented by telemetry data contacts.")
        .new_fn("envelope_name")
        .doc(&format!(
            "Returns the name used when this is embedded within an [{name}](trait.{name}.html) container.",
            name = "Envelope"
        ))
        .arg_ref_self()
        .arg("key", "&str")
        .ret("String")
        .line("let mut name = self.base_type();")
        .line("name.truncate(name.len() - 4);")
        .line("")
        .push_block({
            let mut block = codegen::Block::new("if key.is_empty()");
            block.line(r#"format!("Microsoft.ApplicationInsights.{}.{}", key, name)"#);
            block
        })
        .push_block({
            let mut block = codegen::Block::new("else");
            block.line(r#"format!("Microsoft.ApplicationInsights.{}", name)"#);
            block
        });

    telemetry_data
        .new_fn("base_type")
        .doc(&format!(
            "Returns the base type when placed within an [{name}](trait.{name}.html) container.",
            name = "Data"
        ))
        .arg_ref_self()
        .ret("String");

    telemetry_data
}

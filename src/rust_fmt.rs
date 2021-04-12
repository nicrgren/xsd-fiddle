use crate::{CodeFormatter, ObjectImpl, Primitive, TypeName};
use heck::SnakeCase;
use std::{collections::HashSet, fmt};

static INDENT: &str = "    ";

/// Formatter that writes ObjectImpls to Rust files.
pub struct RustFmt;

impl CodeFormatter for RustFmt {
    fn write_impl_file<W>(&mut self, w: &mut W, object: &ObjectImpl) -> fmt::Result
    where
        W: fmt::Write,
    {
        if object
            .fields
            .iter()
            .any(|f| f.type_name.import_statement().is_some())
        {
            let mut imports = object
                .fields
                .iter()
                .filter_map(|f| f.type_name.import_statement())
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            imports.sort();

            w.write_str("use super::{\n")?;
            for s in imports {
                w.write_str(INDENT)?;
                w.write_str(&s)?;
                w.write_str(",\n")?;
            }

            w.write_str("};\n\n")?;
        }

        writeln!(w, "pub struct {} {{", &object.name)?;
        for f in &object.fields {
            w.write_str(INDENT)?;

            w.write_str("pub ")?;
            w.write_str(&f.name.to_snake_case())?;
            w.write_str(": ")?;

            if f.required {
                self.write_type(w, &f.type_name)?;
            } else {
                w.write_str("Option<")?;
                self.write_type(w, &f.type_name)?;
                w.write_char('>')?;
            }

            w.write_char(',')?;
            w.write_char('\n')?;
        }

        w.write_char('}')?;

        w.write_str("\n\nimpl ")?;
        w.write_str(&object.name)?;
        w.write_str(" {\n")?;

        w.write_str(INDENT)?;
        w.write_str("pub fn required(")?;
        w.write_char('\n')?;

        for rf in object.fields.iter().filter(|f| f.required) {
            w.write_str(INDENT)?;
            w.write_str(INDENT)?;
            w.write_str(&rf.name.to_snake_case())?;
            w.write_str(": ")?;
            self.write_type(w, &rf.type_name)?;
            w.write_str(",\n")?;
        }
        w.write_str(INDENT)?;
        w.write_str(") -> Self {\n")?;

        w.write_str(INDENT)?;
        w.write_str(INDENT)?;
        w.write_str("Self {\n")?;

        for rf in object.fields.iter().filter(|f| f.required) {
            w.write_str(INDENT)?;
            w.write_str(INDENT)?;
            w.write_str(INDENT)?;
            w.write_str(&rf.name.to_snake_case())?;
            w.write_str(",\n")?;
        }

        for rf in object.fields.iter().filter(|f| !f.required) {
            w.write_str(INDENT)?;
            w.write_str(INDENT)?;
            w.write_str(INDENT)?;
            w.write_str(&rf.name.to_snake_case())?;
            w.write_str(": None")?;
            w.write_str(",\n")?;
        }

        w.write_str(INDENT)?;
        w.write_str(INDENT)?;
        w.write_str("}\n")?;

        w.write_str(INDENT)?;
        w.write_str("}\n")?;
        w.write_char('}')?;

        Ok(())
    }

    fn write_type<W>(&mut self, w: &mut W, p: &TypeName) -> fmt::Result
    where
        W: fmt::Write,
    {
        match p {
            TypeName::Primitive(Primitive::Bool) => w.write_str("bool"),
            TypeName::Primitive(Primitive::Int) => w.write_str("i32"),
            TypeName::Primitive(Primitive::Long) => w.write_str("i64"),
            TypeName::Primitive(Primitive::Double) => w.write_str("f64"),
            TypeName::Primitive(Primitive::String) => w.write_str("String"),
            // TypeName::Nullable(ref inner) => {
            //     w.write_str("Option<")?;
            //     self.write_type(w, inner)?;
            //     w.write_str(">")
            // }
            TypeName::Array(ref inner) => {
                w.write_str("Vec<")?;
                self.write_type(w, inner)?;
                w.write_str(">")
            }
            TypeName::Object(ref name) => w.write_str(&name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Field, Primitive};

    #[test]
    fn test_generating_simple_struct_file() {
        let mut buf = String::new();
        let mut f = RustFmt;

        let object = ObjectImpl {
            name: "ImAStruct".into(),
            fields: vec![
                Field::new("f1", Primitive::Bool),
                Field::new("f2", Primitive::Int),
                Field::new("f3", Primitive::Long),
            ],
        };

        f.write_impl_file(&mut buf, &object).expect("Writing");

        assert_eq!(
            r#"
pub struct ImAStruct {
    f1: bool,
    f2: i32,
    f3: i64,
}
"#
            .trim(),
            &buf
        );
    }

    #[test]
    fn test_generating_struct_file() {
        let mut buf = String::new();
        let mut f = RustFmt;

        let object = ObjectImpl {
            name: "ImAStruct".into(),
            fields: vec![
                Field::new("f1", Primitive::Bool),
                Field::new("f2", Primitive::Int),
                Field::new("f3", Primitive::Long),
                Field::new("bool_array", TypeName::array(Primitive::Bool)),
                Field::new("thing_array", TypeName::object("Thing")),
            ],
        };

        f.write_impl_file(&mut buf, &object).expect("Writing");

        assert_eq!(
            r#"
use super::{
    Thing,
};

pub struct ImAStruct {
    f1: bool,
    f2: i32,
    f3: i64,
    bool_array: Vec<bool>,
    thing_array: Thing,
}
"#
            .trim(),
            &buf
        );
    }
}

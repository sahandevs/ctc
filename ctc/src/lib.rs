use anyhow::bail;
use proc_macro::TokenStream;
use toml::Value;
macro_rules! invalid {
    () => {
        anyhow::bail!("invalid type")
    };
}

macro_rules! try_type {
    ($type:ident, $val:ident) => {{
        let mut result = Vec::new();
        let mut failed = false;
        for item in &$val {
            match item {
                Value::$type(x) => result.push(x.clone()),
                _ => failed = true,
            };
        }
        if failed {
            None
        } else {
            Some(result)
        }
    }};
}

const ATTRS: &str = "#![allow(non_upper_case_globals)] #![allow(dead_code)]";

fn value_to_rs(name: String, value: Value) -> Result<String, anyhow::Error> {
    let name: String = name
        .chars()
        .map(|x| match x {
            '-' => '_',
            x => x,
        })
        .filter(|x| x.is_alphabetic() || x == &'_')
        .collect();
    Ok(match value {
        Value::String(x) => format!("pub const {}: &str = r#\"{}\"#;", name, x),
        Value::Integer(x) => format!("pub const {}: i64 = {};", name, x),
        Value::Float(x) => format!("pub const {}: f64 = {};", name, x),
        Value::Boolean(x) => format!("pub const {}: bool = {};", name, x),
        Value::Datetime(_) => invalid!(),
        Value::Array(x) => {
            let size = x.len();
            if let Some(strings) = try_type!(String, x) {
                format!(
                    "pub const {}: [&str;{}] = [{}];",
                    name,
                    size,
                    strings
                        .iter()
                        .map(|x| format!("r#\"{}\"#", x))
                        .collect::<Vec<_>>()
                        .join(",")
                )
            } else if let Some(ints) = try_type!(Integer, x) {
                format!(
                    "pub const {}: [i64;{}] = [{}];",
                    name,
                    size,
                    ints.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            } else if let Some(floats) = try_type!(Float, x) {
                format!(
                    "pub const {}: [f64;{}] = [{}];",
                    name,
                    size,
                    floats
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            } else if let Some(bools) = try_type!(Boolean, x) {
                format!(
                    "pub const {}: [bool;{}] = [{}];",
                    name,
                    size,
                    bools
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            } else {
                invalid!()
            }
        }
        Value::Table(x) => {
            let mut items = Vec::new();
            for (key, val) in x {
                items.push(value_to_rs(key, val)?);
            }
            format!("pub mod {} {{ {} {} }}", name, ATTRS, items.join("\n"))
        }
    })
}

fn load(conf_file_name: &str, out: &str) -> Result<TokenStream, anyhow::Error> {
    let current_dir = std::env::current_dir()?;
    let conf_file_name = current_dir.join(conf_file_name);
    let conf = {
        let raw = std::fs::read_to_string(&conf_file_name);
        match raw {
            Ok(raw) => raw.parse::<Value>().unwrap(),
            Err(e) => bail!("failed to open file {:?}. err: {}", conf_file_name, e),
        }
    };

    let conf_src = value_to_rs(out.into(), conf)?;
    let r: TokenStream = conf_src.parse().unwrap();
    Ok(r)
}

#[proc_macro]
pub fn import_conf(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let mut input = input.split(",");
    let file_name = input.next().expect("expected file_name at first par");
    let file_name = file_name.trim().trim_matches(|x| x == '"');

    let mod_name = input.next().expect("expected module name at second par");

    match load(file_name, mod_name) {
        Ok(x) => x,
        Err(e) => panic!("{}", e),
    }
}

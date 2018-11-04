use std::fmt;
use std::fmt::Write;
use yaml_rust::*;

// from serialize::json
fn escape_json_str(wr: &mut Write, v: &str) -> Result<(), fmt::Error> {
    wr.write_str("\"")?;

    let mut start = 0;

    for (i, byte) in v.bytes().enumerate() {
        let escaped = match byte {
            b'"' => "\\\"",
            b'\\' => "\\\\",
            b'\x00' => "\\u0000",
            b'\x01' => "\\u0001",
            b'\x02' => "\\u0002",
            b'\x03' => "\\u0003",
            b'\x04' => "\\u0004",
            b'\x05' => "\\u0005",
            b'\x06' => "\\u0006",
            b'\x07' => "\\u0007",
            b'\x08' => "\\b",
            b'\t' => "\\t",
            b'\n' => "\\n",
            b'\x0b' => "\\u000b",
            b'\x0c' => "\\f",
            b'\r' => "\\r",
            b'\x0e' => "\\u000e",
            b'\x0f' => "\\u000f",
            b'\x10' => "\\u0010",
            b'\x11' => "\\u0011",
            b'\x12' => "\\u0012",
            b'\x13' => "\\u0013",
            b'\x14' => "\\u0014",
            b'\x15' => "\\u0015",
            b'\x16' => "\\u0016",
            b'\x17' => "\\u0017",
            b'\x18' => "\\u0018",
            b'\x19' => "\\u0019",
            b'\x1a' => "\\u001a",
            b'\x1b' => "\\u001b",
            b'\x1c' => "\\u001c",
            b'\x1d' => "\\u001d",
            b'\x1e' => "\\u001e",
            b'\x1f' => "\\u001f",
            b'\x7f' => "\\u007f",
            _ => continue,
        };

        if start < i {
            wr.write_str(&v[start..i])?;
        }

        wr.write_str(escaped)?;

        start = i + 1;
    }

    if start != v.len() {
        wr.write_str(&v[start..])?;
    }

    wr.write_str("\"")?;
    Ok(())
}

pub fn yaml_to_json(yaml: &Yaml, out: &mut String) {
    match yaml {
        Yaml::Real(real) => {
            write!(out, "{}", real);
        }
        Yaml::Integer(integer) => {
            write!(out, "{}", integer);
        }
        Yaml::String(s) => {
            escape_json_str(out, s).unwrap();
        }
        Yaml::Boolean(boolean) => {
            write!(out, "{}", boolean);
        }
        Yaml::Array(array) => {
            write!(out, "[");

            let mut first = true;
            for element in array {
                if first {
                    first = false;
                } else {
                    write!(out, ",");
                }
                yaml_to_json(element, out);
            }

            write!(out, "]");
        }
        Yaml::Hash(hash) => {
            write!(out, "{{");

            let mut first = true;
            for (key, value) in hash {
                if first {
                    first = false;
                } else {
                    write!(out, ",");
                }
                if let Yaml::String(_) = key {
                    yaml_to_json(key, out);
                    write!(out, ":");
                } else {
                    panic!("Hash key must be a string")
                }
                yaml_to_json(value, out);
            }
            write!(out, "}}");
        }
        Yaml::Null => {
            write!(out, "null");
        }
        Yaml::BadValue => {
            panic!("bad value");
        }
        Yaml::Alias(_) => {
            panic!("Alias is not supported");
        }
    }
}

#[cfg(test)]
fn yaml_string_to_json_string(yaml_str: &str) -> String {
    let docs = YamlLoader::load_from_str(yaml_str).unwrap();
    let mut out = String::new();
    yaml_to_json(&docs[0], &mut out);
    out
}

#[test]
fn test_real() {
    assert_eq!(yaml_string_to_json_string("3.14"), "3.14");
    assert_eq!(yaml_string_to_json_string("-0.5"), "-0.5");
}

#[test]
fn test_integer() {
    assert_eq!(yaml_string_to_json_string("1000"), "1000");
    assert_eq!(yaml_string_to_json_string("-3"), "-3");
}

#[test]
fn test_string() {
    assert_eq!(yaml_string_to_json_string("\"basic string\""), "\"basic string\"");
    assert_eq!(yaml_string_to_json_string("\"line\\nbreak\""), "\"line\\nbreak\"");
}

#[test]
fn test_boolean() {
    assert_eq!(yaml_string_to_json_string("true"), "true");
    assert_eq!(yaml_string_to_json_string("false"), "false");
}

#[test]
fn test_array() {
    assert_eq!(yaml_string_to_json_string("[1, 2, 3]"), "[1,2,3]");
    assert_eq!(yaml_string_to_json_string("[]"), "[]");
}

#[test]
fn test_hash() {
    assert_eq!(yaml_string_to_json_string("{hello: true}"), "{\"hello\":true}");
    assert_eq!(yaml_string_to_json_string("{}"), "{}");
}

#[test]
fn test_null() {
    assert_eq!(yaml_string_to_json_string("null"), "null");
}

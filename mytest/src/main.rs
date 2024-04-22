use serde_json::{Result, Value};

fn print_json_value(value: &Value, indent_level: usize) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                println!("{}Key: {}", " ".repeat(indent_level * 4), key);
                println!("{}Value:", " ".repeat(indent_level * 4));
                print_json_value(value, indent_level + 1);
            }
        }
        Value::Array(arr) => {
            for (index, value) in arr.iter().enumerate() {
                println!("{}Index: {}", " ".repeat(indent_level * 4), index);
                println!("{}Value:", " ".repeat(indent_level * 4));
                print_json_value(value, indent_level + 1);
            }
        }
        _ => {
            println!("{}{}", " ".repeat(indent_level * 4), value);
        }
    }
}

fn print_json(json_str: &str) -> Result<()> {
    let json_value: Value = serde_json::from_str(json_str)?;

    println!("Root:");
    print_json_value(&json_value, 1);

    Ok(())
}

fn main() {
    let json_str = r#"{
        "aspsSip": [
            {
                "dummy": ""
            }
        ],
        "internalMessage": {
            "description": "COMPONENT_UP_AND_RUNNING",
            "parameters": {}
        }
    }"#;

    if let Err(err) = print_json(json_str) {
        eprintln!("Error: {}", err);
    }
}

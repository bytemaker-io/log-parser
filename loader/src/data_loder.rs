use serde_json::{json, to_string_pretty};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::ptr::null;
use regex::Regex;

use crate::data_frame;
use crate::data_frame::DataFrame;
use serde_json::error::Category::Data;
use serde_json::{Result as SResult, Value};
use tokio::io::AsyncWriteExt;
use tracing::info;

pub struct DataLoader {
    pub file: String,
}

impl DataLoader {
    pub async fn new(file: String) -> DataLoader {
        DataLoader { file }
    }
    async fn create_dir_and_file(&self) -> File {
        if !std::path::Path::new("output").exists() {
            std::fs::create_dir("output").unwrap();
        }
        let x = self
            .file
            .split("/")
            .last()
            .unwrap()
            .split(".")
            .next()
            .unwrap();
        if !std::path::Path::new(&format!("output/{}_output.txt", x)).exists() {
            std::fs::write(format!("output/{}_output.txt", x), "").unwrap();
        } else {
            std::fs::write(format!("output/{}_output.txt", x), "").unwrap();
        }
        let file = std::fs::OpenOptions::new()
            .append(true)
            .open(format!("output/{}_output.txt", x))
            .unwrap();
        return file;
    }
    async fn write_to_file(&self, data_frame: &DataFrame,  file: & mut File) {
        let data = format!("{:?}\n", data_frame);
        let mut json_data: String = String::new();
        if data_frame.message().contains("{") && data_frame.message().contains("}") {
            file.write_all("Structured  ".as_bytes()).unwrap();
            let message = data_frame.message();
            let start = message.find("{").unwrap();
            let end = message.rfind("}").unwrap();
            let message = &message[start..end + 1];
            let message = message.replace("\\n", "\n");
            let s = convert_to_json(message.to_string()).await;
            let s = s.replace("\\n", "");
            let s = s.replace("\\\"", "\"");

            // let v = serde_json::from_str(&s);
            // match v {
            //     Ok(v) => {
            //         let v: Value = v;
            //         json_data = (&*to_string_pretty(&v).unwrap()).parse().unwrap();
            //     }
            //     Err(e) => {
            //         println!("data: {:?}", data_frame.time());
            //         println!("error: {:?}", e)
            //     }
            // }
            file.write_all(data.as_bytes()).unwrap();
            print_json(&s,file).unwrap();
        } else {
            file.write_all("Simple ".as_bytes()).unwrap();
            file.write_all(data.as_bytes()).unwrap();
        }
        file.write_all("\n".as_bytes()).unwrap();
    }

    pub async fn read_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("trying to read the file: {:?}", &self.file);
        let _file = tokio::fs::File::open(&self.file).await?;
        info!("file opened successfully, file path: {:?}", &self.file);
        info!("reader created successfully, file path: {:?}", &self.file);
        let mut str: String = String::new();
        let string = tokio::fs::read_to_string(&self.file).await.unwrap();
        let mut file = self.create_dir_and_file().await;
        for line in string.lines() {
            let line = line.replace("\\n", "");
            let line= line.replace("\\r", "");
            if line == string.lines().last().unwrap() {
                if self.do_frame(&line).await {
                    let option = self.parse(&str).await;
                    if let Some(data_frame) = &option {
                        self.write_to_file(data_frame, &mut file).await;
                    }
                    str.clear();
                    str.push_str(&line);
                    str.push_str("\\n")
                } else {
                    str.push_str(&line);
                    str.push_str("\\n")
                }
                let option = self.parse(&str).await;
                if let Some(data_frame) = &option {
                    self.write_to_file(data_frame, &mut file).await;
                }
                break;
            }
            if self.do_frame(&line).await {
                if str.len() == 0 {
                    str.push_str(&line);
                    str.push_str("\\n")
                } else {
                    let option = self.parse(&str).await;
                    if let Some(data_frame) = &option {
                        self.write_to_file(data_frame, &mut file).await;
                    }
                    str.clear();
                    str.push_str(&line);
                    str.push_str("\\n")
                }
            } else if str.len() > 0 {
                str.push_str(&line);
                str.push_str("\\n")
            }
        }
        Ok(())
    }
    pub async fn do_frame(&self, log_item: &str) -> bool {
        let x = log_item.split("/").next().unwrap();
        if x.len() != 4 {
            return false;
        }
        let y = log_item.split("/").nth(1).unwrap();
        if y.len() != 3 {
            return false;
        }
        return true;
    }

    pub async fn parse(&self, log_item: &str) -> Option<data_frame::DataFrame> {
        let parts: Vec<&str> = log_item.split_whitespace().collect();
        let timestamp = parts[0..2].join(" ");
        let component_name = parts[2];
        let event_type = parts[3];
        if event_type != "PORTEVENT" && event_type != "TIMEROP" {
            return None;
        } else if event_type == "PORTEVENT" && log_item.contains("-") {
            let message_body = parts[5..].join(" ");
            let data_frame = data_frame::DataFrame::new(
                timestamp.to_string(),
                component_name.to_string(),
                "".to_string(),
                event_type.to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                message_body.to_string(),
            );
            return Some(data_frame);
        }
        let message = parts[4..].join(" ");
        let message_parts: Vec<&str> = message.split(".").collect();
        let peer_component = message_parts[0];
        let event_description = message_parts[1].split(":").nth(0).unwrap();
        let port_and_function = message_parts[1].split(":").nth(1).unwrap();
        let port = port_and_function.split("(").next().unwrap();
        let function_name = message_parts[1]
            .split(")")
            .nth(0)
            .unwrap()
            .split("(")
            .nth(1)
            .unwrap();
        let mut message_body = "";
        if let Some(index) = message.find(')') {
            message_body = &message[index + 1..];
        }
        let data_frame = data_frame::DataFrame::new(
            timestamp.to_string(),
            component_name.to_string(),
            peer_component.to_string(),
            event_type.to_string(),
            event_description.to_string(),
            port.to_string(),
            function_name.to_string(),
            message_body.to_string(),
        );
        return Some(data_frame);
    }
}

async fn convert_to_json(text: String) -> String {
    let mut lines: Vec<String> = text.split('\n').map(|s| s.to_string()).collect();
    let mut stack: Vec<usize> = Vec::new();
    let mut stack_nest: Vec<usize> = Vec::new();
    let mut sort:Vec<usize>=Vec::new();
    let mut i = 0;
    let mut num_letf_brace = 0;
    let mut num_right_brace = 0;
    for i in 0..lines.len() {
        lines[i] = lines[i].trim().to_string();
        if lines[i].contains('{') {
            stack.push(i);
        }
        if lines[i].contains('}') && !stack.is_empty() {
            let start = stack.pop().unwrap();
            if lines[start].contains('{') && lines[start + 1].trim() == "{" {
                lines[start] = lines[start].replace("{", "[");
                lines[i] = lines[i].replace("}", "]");
                if lines[start-1].contains("ac_values")
                {
                    lines[start-1]=lines[start-1].replace("{","[");
                    lines[start]="".to_string();
                    lines[i]="]".to_string();
                }
            }
        }
        if lines[i].contains(":=") {
            let parts: Vec<&str> = lines[i].split(":=").collect();
            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            lines[i] = format!("\"{}\":{}", key, value);
        }
        lines[i] = lines[i].replace(":=", ":");
        lines[i] = lines[i].replace(": ", ":");
        lines[i] = lines[i].replace(" :", ":");
        lines[i] = lines[i].replace(" :", ":");
        lines[i] = lines[i].replace(" ,", ",");
        if let Some(start_index) = lines[i].find("\"body\":\"{") {
            let start_index = start_index + "\"body\":\"{".len() - 2;
            let last_quote = lines[i][start_index..].rfind('"').unwrap() + start_index;
            lines[i] = lines[i][..start_index].to_owned()
                + &lines[i][start_index + 1..last_quote]
                + &lines[i][last_quote + 1..];
            lines[i]=lines[i].replace("\\t","");
            lines[i]=lines[i].replace("\\","");
        }
        if let Some(start_index) = lines[i].find("\"body\":\"[") {
            let start_index = start_index + "\"body\":\"[".len() - 2;
            let last_quote = lines[i][start_index..].rfind('"').unwrap() + start_index;
            lines[i] = lines[i][..start_index].to_owned()
                + &lines[i][start_index + 1..last_quote]
                + &lines[i][last_quote + 1..];
        }
        if let Some(mut index) = lines[i].find("\"msg\":'") {
            let mut index = index + "\"msg\":'".len();
            let (first, second) = lines[i].split_at(index);
            let second = second.replace("\\\"", "");
            let second = second.replace("\"", "");
            lines[i] = first.to_string() + &second;
            if let Some(index) = lines[i].find(":") {
                lines[i].insert(index+1, '\"');
            }
            lines[i].push('\"');
        }
        if let Some(mut index) = lines[i].find("\"raw\":\"") {
            let mut index = index + "\"raw\":\"".len();
            let (first, second) = lines[i].split_at(index);
            let second = second.replace("\\\"", "");
            let second = second.replace("\"", "");
            lines[i] = first.to_string() + &second;
            lines[i].push('\"');
        }
        if let Some(mut index) = lines[i].find("\"binary_message\":") {
            let mut index = index + "\"binary_message\":".len();
            let ( first, second) = lines[i].split_at(index);
            let first=first.replace("'","");
            let second = second.replace("\\\"", "");
            let second = second.replace("\"", "");
            let second = second.replace("\\r", "\r");
            let second = second.replace("\\t", "\t");
            lines[i] = first.to_string()  + &second;
        }
        if let Some(mut index) = lines[i].find("\"data\":") {
            let mut index = index + "\"data\":".len();
            let (first, second) = lines[i].split_at(index);
            let second = second.replace("\\\"", "");
            let second = second.replace("\"", "");
            let second = second.replace("\\r", "");
            let second = second.replace("\\t", "");
            let second = second.replace("\\n", "");
            lines[i] = first.to_string() + &second;
        }
        if !lines[i].contains("body") {
            let re = Regex::new(r#"\\\"(.*?)\\\""#).unwrap();
            if let Some(captures) = re.captures(&lines[i]) {
                if let Some(matched) = captures.get(1) {
                    lines[i] = lines[i].replace(&captures[0], matched.as_str());
                }
            }
        }
        if !lines[i].contains(":") && !lines[i].contains("[") && !lines[i].contains("]") && !lines[i].contains("}") && !lines[i].contains("{") {
            if lines[i]==""{
                continue;
            }
            lines[i]=lines[i].replace(",","");
            if !lines[i + 1].contains("}") {
                lines[i] = format!("{}:true,", lines[i]);
            }
            else {
                lines[i] = format!("{}:true", lines[i]);
            }
        }
        if lines[i].contains(":{") || lines[i].contains(":\"") || lines[i].contains(":[") {
            continue;
        }
        if lines[i].contains(":") {
            let index = lines[i].find(":").unwrap();
            lines[i].insert(index + 1, '\"');
            if lines[i].ends_with(",") {
                lines[i] = format!("{}\",", &lines[i][..lines[i].len() - 1]);
            } else {
                lines[i] = format!("{}\"", lines[i]);
            }
        }
    }
    for i in 0..lines.len() {
        if lines[i].contains("{") {
            num_letf_brace += 1;
        }
        if lines[i].contains("}") {
            num_right_brace += 1;
        }
    }
    if num_letf_brace != num_right_brace {
        lines.remove(lines.len()-3);
    }
    lines.join("\n")
}

fn print_json_value(value: &Value, indent_level: usize, file: &mut File) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                file.write_all(format!("{}Key: {}\n", " ".repeat(indent_level * 4), key).as_bytes()).unwrap();
                file.write_all(format!("{}Value:\n", " ".repeat(indent_level * 4)).as_bytes()).unwrap();
                print_json_value(value, indent_level + 1,file);
            }
        }
        Value::Array(arr) => {
            for (index, value) in arr.iter().enumerate() {
                file.write_all(format!("{}Index: {}\n", " ".repeat(indent_level * 4), index).as_bytes()).unwrap();
                file.write_all(format!("{}Value:\n", " ".repeat(indent_level * 4)).as_bytes()).unwrap();
                print_json_value(value, indent_level + 1,file);
            }
        }
        _ => {
            file.write_all(format!("{}{}\n", " ".repeat(indent_level * 4), value).as_bytes()).unwrap();
        }
    }
}

fn print_json(json_str: &str, file: &mut File) -> serde_json::Result<()> {
    let json_value: Value = serde_json::from_str(json_str)?;
    file.write_all("Root:\n".as_bytes()).unwrap();
    print_json_value(&json_value, 1,file);
    Ok(())
}

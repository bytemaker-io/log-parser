use std::fs::File;
use std::io::Write;

use tokio::io::AsyncBufReadExt;
use tracing::info;

use crate::data_frame;
use crate::data_frame::DataFrame;

pub struct data_loder {
    pub file: String,
}

impl data_loder {
    pub async fn new(file: String) -> data_loder {
        data_loder { file }
    }
    async fn create_dir_and_file(&self) -> File {
        //if the output directory does not exist, create it
        if !std::path::Path::new("output").exists() {
            std::fs::create_dir("output").unwrap();
        }
        //get the file name
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
            //erase the content of the file
            std::fs::write(format!("output/{}_output.txt", x), "").unwrap();
        }

        //open the file in append mode and return the handle
        let file = std::fs::OpenOptions::new()
            .append(true)
            .open(format!("output/{}_output.txt", x))
            .unwrap();
        return file;
    }
    async fn write_to_file(&self, data_frame: &DataFrame, mut file: &File) {
        let data = format!("{:?}\n", data_frame);
        if data_frame.message().contains("{") && data_frame.message().contains("}") {
            file.write_all("Structured  ".as_bytes()).unwrap();
        } else {
            file.write_all("Simple ".as_bytes()).unwrap();
        }
        file.write_all(data.as_bytes()).unwrap();
    }
    pub async fn read_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("trying to read the file: {:?}", &self.file);
        let _file = tokio::fs::File::open(&self.file).await?;
        info!("file opened successfully, file path: {:?}", &self.file);
        info!("reader created successfully, file path: {:?}", &self.file);
        let mut str: String = String::new();
        let string = tokio::fs::read_to_string(&self.file).await.unwrap();
        //create a file to write the output
        let file = self.create_dir_and_file().await;
        for line in string.lines() {
            //if the line is the last line of the file
            if line == string.lines().last().unwrap() {
                str.push_str(&line);
                let option = self.parse(&str).await;
                if let Some(data_frame) = &option {
                    // info!{"Log item is {}----------------Data frame is {:?}",str,_data_frame};
                    self.write_to_file(data_frame, &file).await;
                }
                break;
            }
            if self.do_frame(&line).await {
                if str.len() == 0 {
                    str.push_str(&line);
                } else {
                    let option = self.parse(&str).await;
                    if let Some(data_frame) = &option {
                        self.write_to_file(data_frame, &file).await;
                        // info!{"Log item is {}----------------Data frame is {:?}",str,_data_frame};
                    }
                    str.clear();
                    str.push_str(&line);
                }
            } else if str.len() > 0 {
                str.push_str(&line);
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
    /**
     * This function is used to parse the log item and return the data frame
     */
    pub async fn parse(&self, log_item: &str) -> Option<data_frame::DataFrame> {
        // split the log item by whitespace
        let parts: Vec<&str> = log_item.split_whitespace().collect();
        let timestamp = parts[0..2].join(" ");
        let component_name = parts[2];
        let event_type = parts[3];
        // if the event type is not PORTEVENT or TIMEROP, return None
        if event_type != "PORTEVENT" && event_type != "TIMEROP" {
            return None;
        }
        // if the event type is PORTEVENT and the log item contains "-", return the data frame
        else if event_type == "PORTEVENT" && log_item.contains("-") {
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
        let Event_description = message_parts[1].split(":").nth(0).unwrap();
        let port_and_function = message_parts[1].split(":").nth(1).unwrap();
        let port = port_and_function.split("(").next().unwrap();
        let function_name = message_parts[1]
            .split(")")
            .nth(0)
            .unwrap()
            .split("(")
            .nth(1)
            .unwrap();

        let mut message_body = message_parts[1].split(")").nth(1).unwrap();

        let data_frame = data_frame::DataFrame::new(
            timestamp.to_string(),
            component_name.to_string(),
            peer_component.to_string(),
            event_type.to_string(),
            Event_description.to_string(),
            port.to_string(),
            function_name.to_string(),
            message_body.to_string(),
        );
        return Some(data_frame);
    }
}

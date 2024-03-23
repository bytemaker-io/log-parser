
use regex::Regex;
use tokio::io::{AsyncBufReadExt};
use tracing::{info};

use crate::data_frame;


pub struct data_loder {
    pub file:String
}

impl data_loder {
    pub async fn new(file:String) -> data_loder {
        data_loder {
            file
        }
    }
    pub async fn read_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("trying to read the file: {:?}", &self.file);
        let file = tokio::fs::File::open(&self.file).await?;
        info!("file opened successfully, file path: {:?}", &self.file);
        let mut reader = tokio::io::BufReader::new(file);
        let mut buffer = String::new();
        info!("reader created successfully, file path: {:?}", &self.file);
        let mut str:String=String::new();
        let _next_data_frame=String::new();
        //在当前目录创建一个文件，名字加上之前的名字再加上output.txt
        let output_file_name=&self.file.split("/").last().unwrap().to_string();
        let _output_file_path=format!("{}output.txt",output_file_name);

        while let Ok(n) = reader.read_line(&mut buffer).await {

            if n == 0 &&str.len()>0 {
                let option = self.parse(str.as_str()).await;
                if let Some(_data_frame) = &option {
                    info!{"Log item is {}----------------Data frame is {:?}",str,option};

                }
                str.clear();

                break;
            }
            //delete the new line character
            buffer.pop();
            if self.do_frame(&buffer).await {
                if str.len()==0 {
                    str.push_str(buffer.clone().as_str());
                }
                else{
                    //It means that we have got the new data frame
                    //parse the last data frame here,and then clear the str
                    self.parse(str.as_str()).await;
                    let option = self.parse(str.as_str()).await;
                    if let Some(_data_frame) = &option {
                        info!{"Log item is {}----------------Data frame is {:?}",str,option};
                    }
                    str.clear();
                    str.push_str(buffer.clone().as_str());
                }
            }
            else if str.len()>0 {
                str.push_str(buffer.clone().as_str());
            }
            buffer.clear();
        }
        Ok(())
    }
    pub async fn do_frame(&self, log_item:&str) -> bool
    {
        // println!("log_item: {}", log_item);
        let re = Regex::new(r"\d{4}/[A-Za-z]+/\d{2} \d{2}:\d{2}:\d{2}\.\d+").unwrap();
        if let Some(_captures) = re.captures(log_item) {
        return true;
        }
        return false;
    }
    pub async fn parse(&self, log_item:&str) ->Option<data_frame::DataFrame>
    {


        let parts: Vec<&str> = log_item.split_whitespace().collect();
        let timestamp = parts[0..2].join(" ");
        let component_name = parts[2];
        let event_type = parts[3];
        if event_type!="PORTEVENT" && event_type!="TIMEROP"{
            return None;
        }
        else if event_type=="PORTEVENT" && log_item.contains("-") {
            let message_body=parts[5..].join(" ");
            let _data_frame=data_frame::DataFrame::new(timestamp.to_string(),component_name.to_string(),"".to_string(),event_type.to_string(),"".to_string(),0,"".to_string(),message_body.to_string());
        }
        let message = parts[4..].join(" ");
        let message_parts: Vec<&str> = message.split(".").collect();
        let peer_component = message_parts[0];
        let Event_description = message_parts[1].split(":").nth(0).unwrap(); // 提取以 ":" 为分割的第一个元素，即事件描述
        let port_and_function= message_parts[1].split(":").nth(1).unwrap();
        let port=port_and_function.split("(").next().unwrap(); // 提取以 " " 为分割的第一个元素，即端口号
        let function_name=message_parts[1].split(")").nth(0).unwrap().split("(").nth(1).unwrap();
        let message_body=message_parts[1].split(")").nth(1).unwrap(); // 提取以 " " 为分割的第一个元素，即函数名
        let data_frame=data_frame::DataFrame::new(timestamp.to_string(),component_name.to_string(),peer_component.to_string(),event_type.to_string(),Event_description.to_string(),port.parse::<i32>().unwrap(),function_name.to_string(),message_body.to_string());
        return Some(data_frame);

    }
}


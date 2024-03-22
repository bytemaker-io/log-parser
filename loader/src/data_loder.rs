use tokio::io::AsyncBufReadExt;

pub struct data_loder {
    pub file_names: Vec<String>,
    pub file_path: Option<String>,
}

impl data_loder {
    pub async fn new(file_names: Vec<String>, file_path: Option<String>) -> data_loder {
        data_loder {
            file_names,
            file_path,
        }
    }
    pub async fn rede_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = tokio::fs::File::open(file_path).await?;
        let mut reader = tokio::io::BufReader::new(file);
        let mut buffer = String::new();
        while let Ok(n) = reader.read_line(&mut buffer).await {
            if n == 0 {
                break;
            }
            println!("{}", buffer);
            buffer.clear();
        }
        Ok(())
    }
}
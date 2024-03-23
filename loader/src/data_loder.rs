use tokio::io::AsyncBufReadExt;
use tracing::info;

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
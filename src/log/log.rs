use std::{fs::File, io::Write, path::Path, time::{SystemTime, UNIX_EPOCH}};

use chrono::{Datelike, Local, Timelike};
use serde::{Deserialize, Serialize};

use crate::consts::{FILEPATH, FILEPATH_PREFIX};
#[derive(Serialize, Deserialize)]
pub struct ExchangeLog{
    pub exchange_log:f64,
}
#[derive(Serialize, Deserialize)]
pub struct Log {
    exchange_log:ExchangeLog,
    pub action_message: String,
    pub time: u64,
}
impl ExchangeLog{
    pub fn new()-> ExchangeLog{
        ExchangeLog{
            exchange_log:0.0
        }
    }
}

impl Log {
    pub fn new(message: String) -> Log {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Log {
            exchange_log:ExchangeLog::new(),
            action_message: message,
            time,
        }
    }
    pub fn save_log(&self) -> std::io::Result<()> {
        // Tarih ve saat bilgilerini al
        let now = Local::now();
        let file_name = format!(
            "{}{:02}_{:02}_{:04}_{:02}:{:02}.json",
            FILEPATH_PREFIX,
            now.day(),
            now.month(),
            now.year(),
            now.hour(),
            now.minute()
        );

        let path = Path::new(FILEPATH);
        let mut file = File::create(path)?;

        // JSON formatında veriyi seri hale getir
        let json = serde_json::to_string(self)?;
        
        // JSON verisini dosyaya yaz
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
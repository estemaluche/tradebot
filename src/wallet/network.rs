use std::thread;
use std::time::Duration;
use crate::currency::currencies::CryptoUnit::CurrencyUnit;

pub struct CurrencyNetwork {
    pub api_key: String,
    pub units: Vec<CurrencyUnit>,
}

impl CurrencyNetwork {
    pub fn new(api_key: String) -> CurrencyNetwork {
        CurrencyNetwork {
            api_key,
            units: Vec::new(),
        }
    }
    pub fn connect_wallet(){
        
    }

    pub fn get_units(&self) -> Result<Vec<CurrencyUnit>, String> {
        let units: Vec<CurrencyUnit> = self.units.iter().cloned().collect(); //REFERANCE AND CLONE DATA
        thread::sleep(Duration::from_secs(10)); // UPDATE RATE
        Ok(units)
        
    }
}

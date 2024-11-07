use crate::{consts::WALLET_API_KEY, currency::currencies::CryptoUnit::CurrencyUnit};

pub struct Wallet{
    pub api_key:String,
    pub balance:f64,
    pub units:CurrencyUnit
}
impl Wallet{
    pub fn new() -> Wallet{
        Wallet{
            api_key: WALLET_API_KEY.to_string(),
            balance: 0.0,
            units: CurrencyUnit::new(),
        }
    }
}
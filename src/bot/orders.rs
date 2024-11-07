pub enum ACTION {
    BUY,
    SELL,
}

impl ACTION {
    pub fn execute(&self, amount: f64) {
        match self {
            ACTION::BUY => {
                println!("Buying asset for {} units...", amount);
            },
            ACTION::SELL => {
                println!("Selling asset for {} units...", amount);
            },
        }
    }
}
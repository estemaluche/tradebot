pub mod rave {
    use crate::currency::currencies::CryptoUnit::CurrencyUnit;

    pub struct RAVEAI {
        pub potential:f64
    }   
    impl RAVEAI{
        pub fn new() -> RAVEAI{
            let _bot = RAVEAI{
                potential: 0.0,
            };
            _bot
        }
        pub fn calc_best_unit(units:&Vec<CurrencyUnit>) -> CurrencyUnit{
            CurrencyUnit::new()//TEST
        }
    }
    
}

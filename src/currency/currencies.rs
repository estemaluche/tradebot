pub mod CryptoUnit {
    use crate::log::log::Log;
    #[derive(Clone)]
    pub struct CurrencyUnit {
        pub name: String,
        pub id: i32,
        pub instant_value: f64, //this value will be updated every 20 minutes
        pub exchange: f64,  // this value will be the purchase transaction record (b_log)
        pub yearly_val:f64,
        pub half_yearly_val:f64,
        pub monthly_val:f64,
        pub weekly_val:f64,
        pub daily_val:f64,
        pub yearly_val_per:f64,
        pub half_yearly_val_per:f64,
        pub monthly_val_per:f64,
        pub weekly_val_per:f64,
        pub daily_val_per:f64,

    }
    pub struct TransactionLog {
        log: Log,
        file_path:String,
        trade_action_log: Log,
    }
    // Use Reference for copy value and need for Multithread Architecture
    impl TransactionLog {
        pub fn new(log_message:String,action_msg:String) -> TransactionLog {
            TransactionLog{
                log: Log::new(log_message),
                trade_action_log: Log::new(action_msg),
                file_path: "".to_string(),
            }
        }
    }

    // Static variable to keep track of the current ID
    static mut ID_COUNTER: i32 = 0;

    impl CurrencyUnit {
        pub fn new() -> CurrencyUnit {
            unsafe {
                // Increment the ID counter and assign it to the new CurrencyUnit
                let id = ID_COUNTER;
                ID_COUNTER += 1; // Increment ID counter after assigning

                CurrencyUnit {
                    name: "".to_string(),
                    id,
                    instant_value: 0.0,
                    exchange: 0.0,
                    yearly_val: 0.0,
                    half_yearly_val: 0.0,
                    monthly_val: 0.0,
                    weekly_val: 0.0,
                    daily_val: 0.0,
                    yearly_val_per: 0.0,
                    half_yearly_val_per: 0.0,
                    monthly_val_per: 0.0,
                    weekly_val_per: 0.0,
                    daily_val_per: 0.0,
                }
            }
        }
    }

}

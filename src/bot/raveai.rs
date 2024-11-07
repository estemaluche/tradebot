pub mod rave {
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
    }
}

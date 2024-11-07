mod bot;
mod consts;
mod currency;
mod log;
mod wallet;
mod app {
    use crate::bot::consensus;
    use crate::bot::orders::ACTION;
    use crate::bot::raveai;
    use crate::consts::NETWORK_API_KEY;
    use crate::currency::currencies::CryptoUnit::CurrencyUnit;
    use crate::currency::currencies::CryptoUnit::TransactionLog;
    use crate::log::log::ExchangeLog;
    use crate::log::log::Log;
    use crate::wallet::network;
    use crate::wallet::network::CurrencyNetwork;
    use crate::wallet::wallet::Wallet;
    pub fn start() {
        let mut raveai = raveai::rave::RAVEAI::new();
        let currency = CurrencyUnit::new();
        let message = "".to_string();
        let exchange_log = ExchangeLog::new();
        let log = Log::new(message);
        let action_log = "".to_string();
        let transaction = TransactionLog::new(log.action_message, action_log);
        let network = CurrencyNetwork::new(NETWORK_API_KEY.to_string());
        let mut wallet = Wallet::new();
        //TEST CASE
        /*let unit = CurrencyUnit::new();
        let mut curr_vec = vec![];
        curr_vec.push(unit);
        let network = CurrencyNetwork{
            api_key: NETWORK_API_KEY.to_string(),
            units: curr_vec,
        };*/

        let consensus = consensus::consensusalgorithm::action_order(
            exchange_log,
            &mut raveai,
            network,
            &mut wallet,
        );
    }
}
fn main() {
    app::start();
}

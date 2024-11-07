pub mod consensusalgorithm {

    use crate::{
        bot::{orders::ACTION, raveai::rave::RAVEAI},
        log::log::ExchangeLog,
        wallet::{network::CurrencyNetwork, wallet::Wallet},
    };
    pub fn action_order(
        log: ExchangeLog,
        ai: &mut RAVEAI,
        c_network: CurrencyNetwork,
        wallet: &mut Wallet,
    ) {
        let network = &c_network;
        loop {
            // `get_units` çağrısını match ile kontrol et
            match CurrencyNetwork::get_units(&network) {
                Ok(units) => {
                    // En iyi potansiyeli olan birimi bul
                    let best_unit = RAVEAI::calc_best_unit(&units);
                    let m = &best_unit.instant_value;
                    let exchange: f64 = log.exchange_log; // Alınan fiyat
                    let potential_increase = calc_potential(ai, network);
                    println!("Calculated potential increase: {:?}", potential_increase);
                    // Koşullar
                    if m - exchange < potential_increase / 3.0 && m - exchange > 0.0 {
                        let amount_to_buy = wallet.balance * 0.6; // %60'lık kısmı al
                        ACTION::BUY.execute(amount_to_buy);
                    } else if m - exchange > potential_increase / 2.0 && m - exchange > 0.0 {
                        ACTION::SELL.execute(exchange);
                    } else {
                        continue;
                    }
                }
                Err(e) => {
                    println!("Error retrieving units: {}", e);
                }
            }      
        }
    }

    pub fn calc_potential(ai: &mut RAVEAI, network: &CurrencyNetwork) -> f64 {
        let mut total_value = 0.0;
        let mut total_value_per = 0.0;
    
        let unit_count = match network.get_units() { // Doğru çağrı
            Ok(units) => {
                for unit in &units { // Referans kullanıldı
                    total_value += unit.yearly_val
                        + unit.half_yearly_val
                        + unit.monthly_val
                        + unit.weekly_val
                        + unit.daily_val; // Toplama
                    total_value_per += unit.yearly_val_per / 100.0
                        + unit.half_yearly_val_per / 100.0
                        + unit.monthly_val_per / 100.0
                        + unit.weekly_val_per / 100.0
                        + unit.daily_val_per / 100.0; // Yüzde toplama
                }
                units.len() as f64 // Birim sayısını döndür
            }
            Err(e) => {
                println!("Some Error Occurred in Taking Units: {:?}", e);
                return 0.0; // Hata durumunda sıfır döndür
            }
        };
    
        if unit_count > 0.0 {
            total_value /= unit_count; // Ortalama değer
            total_value_per /= unit_count; // Ortalama yüzdelik
            ai.potential = total_value * total_value_per; // Potansiyel hesaplama
        }
    
        ai.potential // Düzeltildi
    }
    
    
    
}

fn main() {
    let instant_sepa_cost = 1.0;
    let deposit_amount = 2000.0;

    let kraken_btc_price = 33604.0;
    let binance_btc_price = 33847.0;

    let _kraken_maker_fee_rate = 0.0016;
    let kraken_taker_fee_rate = 0.0026;

    let kraken_withdraw_fee = 0.00015;

    let binance_fee_percent = 0.001;

    let kraken_deposit_amount = deposit_amount - instant_sepa_cost;
    let without_fee_btc_amount = kraken_deposit_amount / kraken_btc_price;
    let btc_amount_with_fee = without_fee_btc_amount * (1.0 - kraken_taker_fee_rate);
    let btc_amount_receive_by_binance = btc_amount_with_fee  - kraken_withdraw_fee;
    let eur_amount_without_fee = binance_btc_price * btc_amount_receive_by_binance;
    let eur_amount_with_fee = eur_amount_without_fee * (1.0 - binance_fee_percent );
    println!("the eur amount is {}", eur_amount_with_fee );
}

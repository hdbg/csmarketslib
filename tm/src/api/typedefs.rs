use rust_decimal::Decimal;
use serde_with::DisplayFromStr;
#[serde_with::serde_as]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Offer {
    pub market_hash_name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub volume: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub price: Decimal,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ItemStatus {
    Sale = 1,
    ShouldTransfer = 2,
    WaitingForTransfer = 3,
    Pickable = 4,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub enum Currency {
    EUR,
    USD,
    RUB,
}

impl Currency {
    pub fn precision(&self) -> u64 {
        match &self {
            Currency::RUB => 2,
            _ => 3,
        }
    }
}

impl Into<&'static str> for Currency {
    fn into(self) -> &'static str {
        match self {
            Currency::EUR => "EUR",
            Currency::USD => "USD",
            Currency::RUB => "RUB",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Item {
    pub item_id: String,
    pub assetid: String,
    pub classid: String,
    pub instanceid: String,
    pub real_instance: String,
    pub market_hash_name: String,

    pub currency: Currency,
    pub price: Decimal,
}

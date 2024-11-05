pub mod price_list {
    pub mod responses {
        use crate::api::typedefs;

        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct BestOffersResponses {
            pub items: Vec<typedefs::Offer>,
        }
    }

    use derive_builder::Builder;

    use crate::api::{
        typedefs::{self, Currency},
        Endpoint,
    };

    #[derive(Builder, Clone, Debug)]
    pub struct PriceList {
        currency: Currency,
    }

    impl PriceList {
        pub fn builder() -> PriceListBuilder {
            PriceListBuilder::create_empty()
        }
    }

    impl Endpoint for PriceList {
        type Data = responses::BestOffersResponses;

        const METHOD: http::Method = http::Method::GET;
        const REQUIRES_API_KEY: bool = false;

        fn endpoint(&self) -> std::borrow::Cow<'_, str> {
            let currency: &str = self.currency.into();
            format!("/prices/{}.json", currency).into()
        }

        fn body(&self) -> Option<crate::api::Body> {
            None
        }

        fn params(&self) -> Vec<(&'static str, std::borrow::Cow<'_, str>)> {
            Vec::default()
        }
    }
}

pub mod items {
    pub mod sale {
        use derive_builder::Builder;
        use rust_decimal::{Decimal, MathematicalOps};

        use crate::api::{typedefs::Currency, Endpoint};

        #[derive(Builder, Clone, Debug)]
        pub struct SetPrice {
            #[builder(setter(into))]
            pub item_id: String,
            #[builder(setter(into))]
            pub price: Decimal,
            pub currency: Currency,
        }

        impl SetPrice {
            pub fn builder() -> SetPriceBuilder {
                SetPriceBuilder::create_empty()
            }
        }

        impl Endpoint for SetPrice {
            type Data = ();

            const METHOD: http::Method = http::Method::GET;

            const REQUIRES_API_KEY: bool = true;

            fn endpoint(&self) -> std::borrow::Cow<'_, str> {
                "/set-price".into()
            }

            fn params(&self) -> Vec<(&'static str, std::borrow::Cow<'_, str>)> {
                let price = self.price.round_dp_with_strategy(
                    self.currency.precision() as u32,
                    rust_decimal::RoundingStrategy::ToZero,
                ) * Decimal::from(Decimal::TEN).powu(self.currency.precision());

                let price: i64 = i64::try_from(price).unwrap();

                let currency: &str = self.currency.into();

                vec![
                    ("item_id", self.item_id.clone().into()),
                    ("price", price.to_string().into()),
                    ("cur", currency.into()),
                ]
            }

            fn body(&self) -> Option<crate::api::Body> {
                None
            }
        }
    }

    pub mod list {
        use crate::api::Endpoint;

        pub mod responses {
            use crate::api::typedefs::Item;

            #[derive(serde::Serialize, serde::Deserialize, Debug)]
            pub struct MyItems {
                pub items: Vec<Item>,
            }
        }

        pub struct Items;

        impl Endpoint for Items {
            type Data = responses::MyItems;

            const METHOD: http::Method = http::Method::GET;

            const REQUIRES_API_KEY: bool = true;

            fn endpoint(&self) -> std::borrow::Cow<'_, str> {
                "/items".into()
            }

            fn params(&self) -> Vec<(&'static str, std::borrow::Cow<'_, str>)> {
                Vec::new()
            }

            fn body(&self) -> Option<crate::api::Body> {
                None
            }
        }
    }
}

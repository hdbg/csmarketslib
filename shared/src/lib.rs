pub mod http {
    pub trait Endpoint {
        type Marker;
        type Output: Sized + DeserializeOwned + std::fmt::Debug;

        type Input: Sized;

        const METHOD: http::Method;
        const REQUIRES_API_KEY: bool;

        fn from_input(input: Self::Input) -> Self;

        fn endpoint(&self) -> Cow<'_, str>;
        fn params(&self) -> Vec<(&'static str, Cow<'_, str>)>;

        fn body(&self) -> Option<Body>;
    }


    pub struct Body {
        pub data: Vec<u8>,
        pub content_type: &'static str,
    }

    pub trait Client {
        type Marker;
        fn exec<E: Endpoint<Marker = Self::Marker>>(
            &self,
            endpoint: E,
        ) -> impl Future<Output = eyre::Result<E::Output>> + Send;
    }

    pub struct EndpointAdapter<Source: Endpoint, Input: Sized, Output: + Sized> 
        where
            Input: Into<Source::Input>,
            Output: From<Source::Output>
     {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub(crate) struct Elisa {
    products: std::vec::Vec<super::Product>,
}

static LIMITER: once_cell::sync::Lazy<
    std::sync::Arc<tokio::sync::Mutex<crate::client::BetweenCalls>>,
> = once_cell::sync::Lazy::new(|| {
    std::sync::Arc::new(tokio::sync::Mutex::new(crate::client::BetweenCalls::new(
        5000,
    )))
});

impl Elisa {
    /// Create a new Elisa instance.
    pub(crate) async fn new(
        postal_code: &std::primitive::str,
        street_address: &std::primitive::str,
        cache_fixed_boardband_products: std::primitive::bool,
    ) -> std::result::Result<Self, crate::client::JSONError> {
        let mut products: std::vec::Vec<super::Product> = std::vec::Vec::<super::Product>::new();
        for address in crate::client::Client::new(
            // We should alwaus cache the address result, because it is not likely to change.
            Some("elisa/address/search"),
            Some(std::sync::Arc::clone(&LIMITER)),
        )?
        .get_json::<std::vec::Vec<super::Address>>(&format!(
            "https://elisa.fi/kauppa/rest/address/search/{}/{}",
            postal_code, street_address
        ))
        .await?
        {
            for product in crate::client::Client::new(
                if cache_fixed_boardband_products {
                    Some("elisa/products/fixedBroadbandProducts")
                } else {
                    None
                },
                Some(std::sync::Arc::clone(&LIMITER)),
            )?
            .get_json::<super::Response>(&format!(
                "https://elisa.fi/kauppa/rest/products/fixedBroadbandProducts/{}/{}",
                postal_code, address.address_id
            ))
            .await?
            .fbb_products
            {
                // Do not include mobile broadband products.
                if product.include() {
                    products.push(product.clone());
                }
            }
        }
        Ok(Self { products })
    }

    /// Get internet products.
    pub(crate) fn products(&self) -> std::vec::Vec<super::Product> {
        self.products.clone()
    }
}

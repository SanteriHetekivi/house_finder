/// Trait to define house selling announcement.
pub(crate) trait Announcement {
    /// URL for the announcement.
    fn url(&self) -> std::string::String;

    /// Location for the house.
    fn location(&self) -> std::option::Option<longitude::Location>;

    /// Square meters for the house.
    fn square_meters_house(&self) -> std::option::Option<std::primitive::u16>;

    /// Total square meters for the whole property.
    fn square_meters_total(&self) -> std::option::Option<std::primitive::u16>;

    /// Price in euros.
    fn euros(&self) -> std::option::Option<std::primitive::u32>;

    /// Street address.
    fn street_address(&self) -> std::string::String;

    /// Construction year.
    fn year(&self) -> std::option::Option<std::primitive::u16>;

    /// Postal code.
    ///
    /// # Arguments
    /// * `cache` - Use cache for HTTP request?
    async fn postal_code(
        &mut self,
    ) -> std::result::Result<std::string::String, crate::etuovi::RegexError>;

    async fn floors(
        &mut self,
    ) -> std::result::Result<std::option::Option<std::primitive::u8>, crate::etuovi::RegexError>;

    /// Text for the announcement.
    ///
    /// # Arguments
    /// * `cache` - Use cache for HTTP request?
    async fn text(&mut self)
        -> std::result::Result<std::string::String, crate::etuovi::RegexError>;
}

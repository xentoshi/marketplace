use anchor_lang::error_code;

#[error_code]
pub enum MarketplaceError {
    #[msg("Name too long")]
    NameTooLong,
    #[msg("Unauthorized Delist")]
    UnauthorizedDelist,
    #[msg("Invalid Name Length")]
    InvalidNameLength,
}
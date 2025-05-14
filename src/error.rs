//! Module for handling errors related to the booking system.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BookingError {
    /// Error when the item with ID {0} is unavailable.
    #[error("Booking item with ID {0} is unavailable.")]
    ItemUnavailable(String),

    /// Error when the booking status is invalid, e.g., an unregistered status.
    #[error("Booking status is invalid: {0}")]
    InvalidStatus(String),

    /// Error when booking creation fails, e.g., due to an internal error.
    #[error("Failed to create booking: {0}")]
    CreationFailed(String),

    /// Error when booking rule validation fails.
    #[error("Booking rule validation failed: {0}")]
    RuleValidationFailed(String),

    /// Error when the booking item quantity exceeds the available limit.
    #[error("Booking item quantity exceeds available limit.")]
    QuantityExceeded,

    /// General error that doesn't fit other categories.
    #[error("General error: {0}")]
    GeneralError(String),
}

impl BookingError {
    /// Creates an InvalidStatus error with the specific status message.
    pub fn new_invalid_status(status: &str) -> Self {
        BookingError::InvalidStatus(status.to_string())
    }

    /// Creates an ItemUnavailable error with the item ID that is unavailable.
    pub fn new_item_unavailable(item_id: &str) -> Self {
        BookingError::ItemUnavailable(item_id.to_string())
    }

    /// Creates a CreationFailed error with a specific failure message.
    pub fn new_creation_failed(message: &str) -> Self {
        BookingError::CreationFailed(message.to_string())
    }

    /// Creates a RuleValidationFailed error with a specific validation failure message.
    pub fn new_rule_validation_failed(message: &str) -> Self {
        BookingError::RuleValidationFailed(message.to_string())
    }

    /// Creates a QuantityExceeded error.
    pub fn new_quantity_exceeded() -> Self {
        BookingError::QuantityExceeded
    }

    /// Creates a GeneralError with a specific message.
    pub fn new_general_error(message: &str) -> Self {
        BookingError::GeneralError(message.to_string())
    }
}

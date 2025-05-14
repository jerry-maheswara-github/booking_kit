//! Module for handling errors related to the booking system.

use thiserror::Error;
use crate::model::status::BookingStatus;

/// Enum used to represent errors that occur during the booking process.
#[derive(Debug, Error)]
pub enum BookingError {
    /// Error when the item with ID {0} is unavailable.
    #[error("Booking item with ID {0} is unavailable.")]
    ItemUnavailable(String),

    /// Error when the booking status is invalid, e.g., an unregistered status.
    #[error("Booking status is invalid: {0}")]
    InvalidStatus(String),

    /// Error when a transition between statuses is not allowed.
    #[error("Invalid status transition from {from:?} to {to:?}")]
    InvalidStatusTransition {
        from: BookingStatus,
        to: BookingStatus,
    },

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

    /// Creates an InvalidStatusTransition error with the provided from and to status.
    ///
    /// This error is returned when an attempt is made to transition between two booking statuses
    /// in an invalid or unsupported way. It helps to catch scenarios where a booking cannot be 
    /// moved from one status to another due to business logic constraints.
    ///
    /// # Parameters
    /// - `from`: The current status of the booking.
    /// - `to`: The status the booking is trying to transition to.
    ///
    /// # Example
    /// ```rust
    /// use booking_kit::error::BookingError;
    /// use booking_kit::model::status::BookingStatus;
    /// let error = BookingError::new_invalid_transition(BookingStatus::Pending, BookingStatus::Expired);
    /// assert_eq!(error.to_string(), "Invalid status transition from Pending to Expired");
    /// ```
    pub fn new_invalid_transition(from: BookingStatus, to: BookingStatus) -> Self {
        BookingError::InvalidStatusTransition { from, to }
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

//! Provides validation logic and custom rules for bookings.

use crate::model::entities::Booking;

/// A trait representing a validation rule for bookings.
///
/// The `BookingRule` trait defines a common interface for implementing validation rules
/// that can be applied to bookings. Any rule that implements this trait must define the
/// `validate` method to check the validity of a booking based on specific criteria.
///
/// The `validate` method returns a `Result<(), String>`: 
/// - `Ok(())` if the booking is valid according to the rule,
/// - `Err(String)` if the booking is invalid, with an error message explaining why.
///
/// # Example:
///
/// ```rust
/// use booking_kit::model::entities::Booking;
/// use booking_kit::model::rules::BookingRule;
///
/// struct MaxItemRule;
///
/// impl BookingRule for MaxItemRule {
///     fn validate(&self, booking: &Booking) -> Result<(), String> {
///         if booking.items.len() > 10 {
///             Err("Booking cannot contain more than 10 items.".to_string())
///         } else {
///             Ok(())
///         }
///     }
/// }
/// ```
pub trait BookingRule {
    /// Validates the booking according to the rule.
    ///
    /// # Arguments
    ///
    /// * `booking` - The booking to be validated.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the booking is valid, or `Err(String)` with an error message if invalid.
    fn validate(&self, booking: &Booking) -> Result<(), String>;
}

/// A rule that ensures a booking contains at least one item.
///
/// The `MinItemRule` is a simple validation rule that checks if a booking contains at least
/// one item. This rule is commonly used in booking systems to ensure that empty bookings are not created.
///
/// # Example:
///
/// ```rust
/// use chrono::Utc;
/// use booking_kit::model::entities::Booking;
/// use booking_kit::model::rules::{BookingRule, MinItemRule};
/// use booking_kit::model::status::BookingStatus;
/// let booking = Booking {
///     id: "booking_1".to_string(),
///     user_id: "user_1".to_string(),
///     items: vec![],
///     status: BookingStatus::Pending,
///     created_at: Utc::now(),
/// };
///
/// let min_item_rule = MinItemRule;
/// let result = min_item_rule.validate(&booking);
///
/// match result {
///     Ok(_) => println!("Booking is valid."),
///     Err(msg) => println!("Booking invalid: {}", msg),
/// }
/// ```
pub struct MinItemRule;

impl BookingRule for MinItemRule {
    /// Validates that the booking contains at least one item.
    ///
    /// # Arguments
    ///
    /// * `booking` - The booking to be validated.
    ///
    /// # Returns
    ///
    /// Returns `Err` with a message if the booking does not contain at least one item, 
    /// otherwise returns `Ok(())`.
    fn validate(&self, booking: &Booking) -> Result<(), String> {
        if booking.items.is_empty() {
            Err("Booking must contain at least one item.".into())
        } else {
            Ok(())
        }
    }
}

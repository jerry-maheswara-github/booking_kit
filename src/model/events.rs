//! Contains domain events for tracking booking actions (e.g., booking creation and cancellation).

use chrono::{DateTime, Utc};

/// Represents the event when a booking is created.
///
/// The `BookingCreated` struct is used to capture the details of a booking creation event. It
/// contains the booking ID and the timestamp of when the booking was created.
///
/// # Fields:
///
/// - `booking_id`: The unique identifier for the booking that was created.
/// - `timestamp`: The timestamp when the booking creation event occurred.
///
/// # Example:
///
/// ```rust
/// use chrono::{Utc, DateTime};
/// use booking_kit::model::events::BookingCreated;
///
/// let booking_created = BookingCreated {
///     booking_id: "booking_123".to_string(),
///     timestamp: Utc::now(),
/// };
///
/// println!("Booking created with ID: {}", booking_created.booking_id);
/// println!("Timestamp: {}", booking_created.timestamp);
/// ```
#[derive(Debug, Clone)]
pub struct BookingCreated {
    /// The unique identifier for the booking that was created.
    pub booking_id: String,

    /// The timestamp when the booking creation event occurred.
    pub timestamp: DateTime<Utc>,
}

/// Represents the event when a booking is canceled.
///
/// The `BookingCanceled` struct is used to capture the details of a booking cancellation event.
/// It includes the booking ID, the reason for cancellation, and the timestamp of when the cancellation occurred.
///
/// # Fields:
///
/// - `booking_id`: The unique identifier for the booking that was canceled.
/// - `reason`: The reason for the booking cancellation.
/// - `timestamp`: The timestamp when the booking cancellation event occurred.
///
/// # Example:
///
/// ```rust
/// use chrono::{Utc, DateTime};
/// use booking_kit::model::events::BookingCanceled;
///
/// let booking_canceled = BookingCanceled {
///     booking_id: "booking_123".to_string(),
///     reason: "User requested cancellation".to_string(),
///     timestamp: Utc::now(),
/// };
///
/// println!("Booking canceled with ID: {}", booking_canceled.booking_id);
/// println!("Reason: {}", booking_canceled.reason);
/// println!("Timestamp: {}", booking_canceled.timestamp);
/// ```
#[derive(Debug, Clone)]
pub struct BookingCanceled {
    /// The unique identifier for the booking that was canceled.
    pub booking_id: String,

    /// The reason for the booking cancellation.
    pub reason: String,

    /// The timestamp when the booking cancellation event occurred.
    pub timestamp: DateTime<Utc>,
}

//! Contains the various statuses a booking can have, such as `Pending`, `Confirmed`, and `Canceled`.

use serde::{Deserialize, Serialize};

/// Enum representing the various states a booking can have.
///
/// The `BookingStatus` enum defines the possible states of a booking throughout its lifecycle.
/// Each status represents a different stage or condition of a booking, allowing the system to track and
/// manage bookings effectively.
///
/// # Variants:
///
/// - `Pending`: The booking has been created but not yet confirmed.
/// - `Confirmed`: The booking has been confirmed and is now finalized.
/// - `Canceled`: The booking has been canceled by the user or the system.
/// - `Expired`: The booking was not confirmed in time and has expired.
/// - `Failed`: The booking was not confirmed in time and has failed.
///
/// # Example:
///
/// ```rust
/// use booking_kit::model::status::BookingStatus;
/// let mut booking_status = BookingStatus::Pending;
///
/// // Update status
/// booking_status = BookingStatus::Confirmed;
///
/// if booking_status == BookingStatus::Confirmed {
///     println!("The booking is confirmed.");
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BookingStatus {
    /// The booking has been created but not yet confirmed.
    Pending,

    /// The booking has been confirmed and is now finalized.
    Confirmed,

    /// The booking has been canceled by the user or the system.
    Canceled,

    /// The booking was not confirmed in time and has expired.
    Expired,
    
    /// The booking was failed.
    Failed,

    /// The booking has been fulfilled or used successfully.
    Completed,
}

impl BookingStatus {
    /// Validates if a transition from `self` to `next` is allowed.
    pub fn can_transition_to(&self, next: &BookingStatus) -> bool {
        use BookingStatus::*;

        match (self, next) {
            // Pending → Confirmed, Canceled, Expired, or Failed
            (Pending, Confirmed)
            | (Pending, Canceled)
            | (Pending, Expired)
            | (Pending, Failed) => true,

            // Confirmed → Completed, Canceled
            (Confirmed, Completed) | (Confirmed, Canceled) => true,

            // Completed = final state
            (Completed, _) => false,

            // Canceled, Expired, Failed = final states
            (Canceled, _) => false,
            (Expired, _) => false,
            (Failed, _) => false,

            // Prevent no-op transition unless explicitly allowed
            (current, next) => current == next,
        }
    }
}

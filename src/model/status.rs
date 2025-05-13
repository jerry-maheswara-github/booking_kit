//! Contains the various statuses a booking can have, such as `Pending`, `Confirmed`, and `Canceled`.

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookingStatus {
    /// The booking has been created but not yet confirmed.
    Pending,

    /// The booking has been confirmed and is now finalized.
    Confirmed,

    /// The booking has been canceled by the user or the system.
    Canceled,

    /// The booking was not confirmed in time and has expired.
    Expired,
}

//! Defines core entities like `Booking`, and related data structures.

use serde::{Deserialize, Serialize};
use crate::model::status::BookingStatus;
use crate::traits::Bookable;

/// Represents a generic booking entry.
///
/// This struct is designed to be flexible and can represent any kind of bookable item,
/// such as hotel rooms, flights, events, or rentals.
///
/// # Type Parameters
/// - `T`: The type of item being booked. Must implement the `Bookable` trait.
/// - `ID`: Identifier type used for both booking ID and user ID. Typically `String` or `&str`.
/// - `Timestamp`: Type representing time values. This is left generic to allow use of custom types like `String`, `i64`, or `chrono::DateTime`.
/// - `Metadata`: Optional additional data specific to the domain, such as payment info or notes.
///
/// # Fields
/// - `id`: Unique identifier for this booking.
/// - `user_id`: Optional ID of the user making the booking.
/// - `item`: The item that is being booked.
/// - `status`: Current status of the booking (e.g., `Pending`, `Confirmed`, `Canceled`).
/// - `created_at`: Timestamp indicating when the booking was created.
/// - `expires_at`: Optional timestamp indicating when the booking expires (if applicable).
/// - `metadata`: Optional metadata for extending the booking with domain-specific information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Booking<T, ID, Timestamp, Metadata> {
    pub id: ID,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ID>,
    pub item: T,
    pub status: BookingStatus,
    pub created_at: Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl<T, ID, Timestamp, Metadata> Booking<T, ID, Timestamp, Metadata>
where
    T: Bookable,
{
    /// Returns the ID of the booked item.
    ///
    /// This works for any `T` as long as it implements `Bookable`,
    /// so you don't need to know the structure of `T` itself.
    pub fn item_id(&self) -> &str {
        self.item.id()
    }

    /// Checks if the booking is in a `Pending` state.
    ///
    /// This is useful to determine if a booking is awaiting confirmation.
    pub fn is_pending(&self) -> bool {
        self.status == BookingStatus::Pending
    }

    /// Checks if the booking has been confirmed.
    ///
    /// Use this to verify that the booking was successfully completed.
    pub fn is_confirmed(&self) -> bool {
        self.status == BookingStatus::Confirmed
    }

    /// Checks if the booking was canceled.
    pub fn is_canceled(&self) -> bool {
        self.status == BookingStatus::Canceled
    }

    /// Checks if the booking has expired.
    ///
    /// This does not automatically evaluate time; it simply checks the status field.
    pub fn is_expired(&self) -> bool {
        self.status == BookingStatus::Expired
    }

    /// Returns `true` if the booking is considered active.
    ///
    /// Typically, this includes `Pending` and `Confirmed` states.
    pub fn is_active(&self) -> bool {
        matches!(self.status, BookingStatus::Pending | BookingStatus::Confirmed)
    }

    /// Returns `true` if the booking is in a final state and can no longer be modified.
    ///
    /// Final states usually include `Canceled` and `Expired`.
    pub fn is_final(&self) -> bool {
        matches!(
            self.status,
            BookingStatus::Canceled |
            BookingStatus::Expired |
            BookingStatus::Failed |
            BookingStatus::Completed
        )
    }

    /// Returns true if the booking is completed successfully.
    pub fn is_completed(&self) -> bool {
        self.status == BookingStatus::Completed
    }

}

//! Stateless manager providing booking logic APIs.

use crate::error::BookingError;
use crate::model::booking::Booking;
use crate::model::status::BookingStatus;
use crate::traits::Bookable;

/// `BookingManager` is a struct that manages the booking process, including handling reservations,
/// validating data, and interacting with available resources.
pub struct BookingManager;

impl BookingManager {
    /// Creates a new booking with status `Pending`, without checking item availability.
    ///
    /// Use this method if you trust the availability logic is handled externally.
    pub fn create<T, ID, Timestamp, Metadata>(
        booking_id: ID,
        user_id: Option<ID>,
        item: T,
        created_at: Timestamp,
        expires_at: Option<Timestamp>,
        metadata: Option<Metadata>,
    ) -> Booking<T, ID, Timestamp, Metadata>
    where
        T: Bookable,
        ID: Clone,
    {
        Booking {
            id: booking_id,
            user_id,
            item,
            status: BookingStatus::Pending,
            created_at,
            expires_at,
            metadata,
        }
    }

    /// Attempts to create a booking after checking item availability via `Bookable::is_available()`.
    ///
    /// Returns `BookingError::ItemUnavailable` if the item is not available.
    pub fn try_create<T, ID, Timestamp, Metadata>(
        booking_id: ID,
        user_id: Option<ID>,
        item: T,
        created_at: Timestamp,
        expires_at: Option<Timestamp>,
        metadata: Option<Metadata>,
    ) -> Result<Booking<T, ID, Timestamp, Metadata>, BookingError>
    where
        T: Bookable,
        ID: Clone,
    {
        if !item.is_available() {
            return Err(BookingError::new_item_unavailable(item.id()));
        }

        Ok(Booking {
            id: booking_id,
            user_id,
            item,
            status: BookingStatus::Pending,
            created_at,
            expires_at,
            metadata,
        })
    }

    /// Marks an existing booking as `Confirmed`.
    ///
    /// Typically used when payment or approval is completed.
    pub fn confirm<T, ID, Timestamp, Metadata>(
        booking: &mut Booking<T, ID, Timestamp, Metadata>,
    ) -> Result<(), BookingError>
    where
        T: Bookable,
    {
        let from = booking.status.clone();
        let to = BookingStatus::Confirmed;

        if !from.can_transition_to(&to) {
            return Err(BookingError::new_invalid_transition(from, to));
        }

        booking.status = to;
        Ok(())
    }

    /// Marks an existing booking as `Canceled`.
    ///
    /// Can be used due to user request, timeout, or manual rejection.
    pub fn cancel<T, ID, Timestamp, Metadata>(booking: &mut Booking<T, ID, Timestamp, Metadata>) {
        booking.status = BookingStatus::Canceled;
    }

    /// Marks an existing booking as `Expired`.
    ///
    /// Should be used when the booking is no longer valid due to time constraints or policies.
    pub fn expire<T, ID, Timestamp, Metadata>(booking: &mut Booking<T, ID, Timestamp, Metadata>) {
        booking.status = BookingStatus::Expired;
    }
}

use crate::error::BookingError;
use crate::model::booking::Booking;
use crate::model::status::BookingStatus;
use crate::traits::Bookable;

/// Manager tanpa state, hanya menyediakan API logika booking

pub struct BookingManager;
 
impl BookingManager {
    pub fn create<T, ID, Timestamp, Metadata>(
        booking_id: ID,
        user_id: ID,
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

    pub fn try_create<T, ID, Timestamp, Metadata>(
        booking_id: ID,
        user_id: ID,
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

    pub fn confirm<T, ID, Timestamp, Metadata>(booking: &mut Booking<T, ID, Timestamp, Metadata>) {
        booking.status = BookingStatus::Confirmed;
    }

    pub fn cancel<T, ID, Timestamp, Metadata>(booking: &mut Booking<T, ID, Timestamp, Metadata>) {
        booking.status = BookingStatus::Canceled;
    }

    pub fn expire<T, ID, Timestamp, Metadata>(booking: &mut Booking<T, ID, Timestamp, Metadata>) {
        booking.status = BookingStatus::Expired;
    }
    
}

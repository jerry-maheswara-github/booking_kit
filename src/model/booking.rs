//! Defines core entities like `Booking`, and related data structures.

use serde::{Deserialize, Serialize};
use crate::model::status::BookingStatus;
use crate::traits::Bookable;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Booking<T, ID, Timestamp, Metadata> {
    pub id: ID,
    pub user_id: ID,
    pub item: T,
    pub status: BookingStatus,
    pub created_at: Timestamp,
    pub expires_at: Option<Timestamp>,
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
}

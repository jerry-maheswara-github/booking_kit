//! Defines core entities like `Booking`, `BookingItem`, and related data structures.

use chrono::{DateTime, Utc};
use crate::model::status::BookingStatus;

/// Represents a booking made by a user.
///
/// The `Booking` struct contains all the necessary details for a booking, such as the unique
/// booking ID, the user who made the booking, the items included in the booking, the booking's
/// current status, and the creation timestamp.
///
/// # Fields:
///
/// - `id`: A unique identifier for the booking.
/// - `user_id`: The identifier of the user who made the booking.
/// - `items`: A list of items included in the booking, represented as `BookingItem` structs.
/// - `status`: The current status of the booking (e.g., `Pending`, `Confirmed`, `Canceled`, etc.).
/// - `created_at`: The timestamp when the booking was created.
///
/// # Example:
///
/// ```rust
/// use chrono::{Utc, DateTime};
/// use booking_kit::model::entities::{Booking, BookingItem};
/// use booking_kit::model::status::BookingStatus;
///
/// let item = BookingItem {
///     item_id: "item_1".to_string(),
///     quantity: 2,
///     price_per_unit: 100.0,
/// };
///
/// let booking = Booking {
///     id: "booking_1".to_string(),
///     user_id: "user_1".to_string(),
///     items: vec![item],
///     status: BookingStatus::Pending,
///     created_at: Utc::now(),
/// };
///
/// println!("Booking ID: {}", booking.id);
/// println!("Status: {:?}", booking.status);
/// ```
#[derive(Debug, Clone)]
pub struct Booking {
    /// A unique identifier for the booking.
    pub id: String,

    /// The identifier of the user who made the booking.
    pub user_id: String,

    /// A list of items included in the booking.
    pub items: Vec<BookingItem>,

    /// The current status of the booking.
    pub status: BookingStatus,

    /// The timestamp when the booking was created.
    pub created_at: DateTime<Utc>,
}

/// Represents an item within a booking.
///
/// The `BookingItem` struct contains the details of an individual item that is part of a booking,
/// such as the item ID, quantity, and price per unit.
///
/// # Fields:
///
/// - `item_id`: A unique identifier for the item being booked.
/// - `quantity`: The quantity of the item being reserved.
/// - `price_per_unit`: The price for each unit of the item.
///
/// # Example:
///
/// ```rust
/// use booking_kit::model::entities::BookingItem;
/// let item = BookingItem {
///     item_id: "item_1".to_string(),
///     quantity: 2,
///     price_per_unit: 50.0,
/// };
///
/// println!("Item ID: {}", item.item_id);
/// println!("Total Price: {}", item.quantity as f64 * item.price_per_unit);
/// ```
#[derive(Debug, Clone)]
pub struct BookingItem {
    /// A unique identifier for the item being booked.
    pub item_id: String,

    /// The quantity of the item being reserved.
    pub quantity: u32,

    /// The price per unit of the item.
    pub price_per_unit: f64,
}

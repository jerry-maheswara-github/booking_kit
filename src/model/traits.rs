//! A trait representing an entity that can be reserved or booked.

/// The `Reservable` trait defines the basic functionality for any entity that can be reserved,
/// such as items, rooms, seats, or tickets. It allows checking availability, reserving, and releasing resources.
///
/// Implementing this trait is essential for entities that need to be part of a booking system, providing methods
/// to interact with the booking process.
///
/// # Methods:
///
/// - `get_id`: Returns the unique identifier for the reservable item.
/// - `is_available`: Checks whether the item is available for booking, given the requested quantity.
/// - `reserve`: Reserves the specified quantity of the item.
/// - `release`: Releases the reserved quantity of the item back to availability.
///
/// # Example:
///
/// ```rust
/// use booking_kit::model::traits::Reservable;
///
/// struct HotelRoom {
///     id: String,
///     available_quantity: u32,
/// }
///
/// impl Reservable for HotelRoom {
///     fn get_id(&self) -> String {
///         self.id.clone()
///     }
///
///     fn is_available(&self, quantity: u32) -> bool {
///         self.available_quantity >= quantity
///     }
///
///     fn reserve(&mut self, quantity: u32) {
///         if self.is_available(quantity) {
///             self.available_quantity -= quantity;
///         }
///     }
///
///     fn release(&mut self, quantity: u32) {
///         self.available_quantity += quantity;
///     }
/// }
/// ```
pub trait Reservable {
    /// Returns the unique identifier for the reservable item.
    fn get_id(&self) -> String;

    /// Checks if the item is available for booking, given the requested quantity.
    ///
    /// # Arguments
    ///
    /// * `quantity` - The requested quantity to check for availability.
    ///
    /// # Returns
    ///
    /// Returns `true` if the requested quantity is available, otherwise `false`.
    fn is_available(&self, quantity: u32) -> bool;

    /// Reserves the specified quantity of the item.
    ///
    /// # Arguments
    ///
    /// * `quantity` - The quantity to reserve.
    ///
    /// # Notes
    ///
    /// This method should adjust the availability of the item accordingly.
    fn reserve(&mut self, quantity: u32);

    /// Releases the reserved quantity of the item back to availability.
    ///
    /// # Arguments
    ///
    /// * `quantity` - The quantity to release.
    ///
    /// # Notes
    ///
    /// This method should increase the available quantity of the item.
    fn release(&mut self, quantity: u32);
}

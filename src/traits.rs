//! A trait representing an entity that can be reserved or booked.

/// The `Bookable` trait defines the common interface for any entity (e.g., rooms, vehicles, events)
/// that can be booked or reserved. Types implementing this trait should provide specific logic for 
/// reserving the entity and checking availability.
pub trait Bookable {
    fn id(&self) -> &str;
    fn is_available(&self) -> bool;
}

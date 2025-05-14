use serde::{Deserialize, Serialize};
use booking_kit::manager::BookingManager;
use booking_kit::traits::Bookable;

#[derive(Debug, Deserialize, Serialize)]
struct Room { id: &'static str }
impl Bookable for Room {
    fn id(&self) -> &str { self.id }
    fn is_available(&self) -> bool { true }
}

#[derive(Debug, Deserialize, Serialize)]
struct HotelRoom { id: &'static str, occupied: bool }
impl Bookable for HotelRoom {
    fn id(&self) -> &str { self.id }
    fn is_available(&self) -> bool { !self.occupied }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let available_room = Room { id: "R101" };
    let booking = BookingManager::create(
        "booking-001", None, available_room, "now", None, None::<()>
    );
    
    let json = serde_json::to_string_pretty(&booking)?;
    println!("Booking : \n{}\n", json);
    println!("Booking created for: {}", booking.item_id());

    let occupied_room = HotelRoom { id: "R404", occupied: true };
    match BookingManager::try_create(
        "booking-002", Some("user-456"), occupied_room, "now", None, None::<()>
    ) {
        Ok(bk) => println!("Booking successful.{:?}", bk),
        Err(e) => println!("Booking failed: {}", e),
    }
    
    Ok(())
}

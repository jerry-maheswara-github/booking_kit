use serde::{Deserialize, Serialize};
use booking_kit::manager::BookingManager;
use booking_kit::traits::Bookable;
use booking_kit::model::booking::Booking;

#[derive(Debug, Clone)]
struct Room {
    id: &'static str,
}

impl Bookable for Room {
    fn id(&self) -> &str {
        self.id
    }

    fn is_available(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HotelBookingMeta {
    special_request: Option<String>,
    breakfast_included: bool,
    voucher_code: Option<String>
}

fn main() {
    let room = Room { id: "room-888" };

    let metadata = HotelBookingMeta {
        special_request: Some("Late check-in".into()),
        breakfast_included: true,
        voucher_code: Some("PROMO2025".into()),
    };

    let booking: Booking<Room, &str, &str, HotelBookingMeta> =
        BookingManager::create(
            "booking-xyz",
            Some("user-123"),
            room,
            "2025-05-14T10:00:00Z",
            None,
            Some(metadata),
        );

    if let Some(meta) = &booking.metadata {
        println!("Special request: {:?}", meta.special_request);
    }
}

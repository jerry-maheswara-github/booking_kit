use serde::{Deserialize, Serialize};
use booking_kit::error::BookingError;
use booking_kit::manager::BookingManager;
use booking_kit::traits::Bookable;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
struct FlightTicket {
    flight_number: &'static str,
    departure: &'static str,
    arrival: &'static str,
    seat_class: &'static str,
}

impl Bookable for FlightTicket {
    fn id(&self) -> &str {
        self.flight_number
    }

    fn is_available(&self) -> bool {
        true
    }
}

#[derive(Debug)]
struct HotelRoom {
    id: String,
    is_occupied: bool,
    is_under_maintenance: bool,
}

impl Bookable for HotelRoom {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_available(&self) -> bool {
        !self.is_occupied && !self.is_under_maintenance
    }
}

#[derive(Debug)]
struct FlightSegment {
    id: String,
    seat_quota: u32,
    status: String, // "Scheduled", "Cancelled", "Delayed", etc.
}

impl Bookable for FlightSegment {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_available(&self) -> bool {
        self.seat_quota > 0 && self.status == "Scheduled"
    }
}

#[derive(Debug)]
struct EventTicket {
    id: String,
    event_start: i64, // unix timestamp
    current_time: i64,
    remaining_quota: u32,
}

impl Bookable for EventTicket {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_available(&self) -> bool {
        self.remaining_quota > 0 && self.current_time < self.event_start
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== HotelRoom (try_create with unavailable item) ===");
    let hotel_room = HotelRoom {
        id: "room-1".into(),
        is_occupied: true,
        is_under_maintenance: false,
    };

    match BookingManager::try_create(
        "booking-1",
        "user-1",
        hotel_room,
        "2025-05-14T00:00:00Z",
        None,
        None::<()>,
    ) {
        Ok(booking) => println!("Booking success: {:?}", booking),
        Err(BookingError::ItemUnavailable(item_id)) => {
            println!("Booking failed: item {} is not available", item_id);
        }
        Err(e) => println!("Booking failed: {}", e),
    }

    println!("\n=== Room (create with always available item) ===");
    let room = Room { id: "room-101" };
    let booking = BookingManager::create(
        "booking_002",
        "user-002",
        room,
        "now",
        None,
        None::<()>,
    );
    println!("Booking created with status: {:?}", booking.status);
    println!("Item ID: {}", booking.item_id());

    println!("\n=== FlightTicket (basic test) ===");
    let ticket = FlightTicket {
        flight_number: "GA123",
        departure: "CGK",
        arrival: "DPS",
        seat_class: "Economy",
    };

    let booking_flight = BookingManager::create(
        "booking_003",
        "user-003",
        ticket,
        "2025-05-14T10:00:00Z",
        None,
        None::<()>,
    );

    println!(
        "Flight booking created for flight number: {}",
        booking_flight.item_id()
    );

    println!("\n=== FlightSegment (try_create with unavailable seat/status) ===");
    let segment = FlightSegment {
        id: "segment-001".into(),
        seat_quota: 0,
        status: "Scheduled".into(),
    };

    match BookingManager::try_create(
        "booking-004",
        "user-004",
        segment,
        "now",
        None,
        None::<()>,
    ) {
        Ok(_) => println!("Flight segment booking success"),
        Err(BookingError::ItemUnavailable(item_id)) => {
            println!("Flight segment not available: {}", item_id)
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== EventTicket (try_create with expired or sold out event) ===");
    let event = EventTicket {
        id: "event-123".into(),
        event_start: 1_600_000_000,     // di masa lalu
        current_time: 1_700_000_000,    // sekarang (lebih besar)
        remaining_quota: 0,
    };

    match BookingManager::try_create(
        "booking-005",
        "user-005",
        event,
        "now",
        None,
        None::<()>,
    ) {
        Ok(_) => println!("Event ticket booking success"),
        Err(BookingError::ItemUnavailable(item_id)) => {
            println!("Event ticket not available: {}", item_id)
        }
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}

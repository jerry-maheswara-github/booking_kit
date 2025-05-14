//! # 🧳 booking_kit
//!
//! ✨ Extensible booking toolkit for Rust with core types, status logic, events, and validation rules.
//!
//! `booking_kit` is a generic and extensible toolkit designed to handle the core logic for booking systems in Rust.  
//! This crate provides structures, status management, events, validation rules, and error handling 🛠️ that are essential 
//! for creating a reservation or booking system. It is designed to be modular, flexible, and reusable across different 
//! applications.
//!
//!
//! This toolkit is designed to be extensible, allowing developers to build custom booking logic and integrate it into 
//! various types of applications — travel, hotels, events, or anything that needs reservation logic.
//!
//! It provides the flexibility to manage bookings without relying on specific external services, making it ideal for a variety of use cases.
//!
//! ---
//!
//! ## 🚀 Features:
//!
//! - 📦 **Core Data Structures** — Defines `Booking`, `BookingItem`, and `BookingStatus` for handling bookings and items.
//! - 🔄 **Status Management** — Offers predefined booking statuses such as `Pending`, `Confirmed`, `Canceled`, and more.
//! - 📡 **Domain Events** — Includes events like `BookingCreated` and `BookingCanceled` for tracking state transitions.
//! - ✅ **Rule Validation** — Allows custom validation rules for bookings, such as ensuring item availability or valid status transitions.
//! - ⚠️ **Error Handling** — Provides a set of errors like `ItemUnavailable`, `QuantityExceeded`, and `InvalidStatus` to handle common booking-related issues.
//! - 🛠️ **Metadata Support** — Customize your bookings with additional metadata for each item.
//!
//! ## 🧰 Core Components:
//!
//! - **BookingManager** — Handles your booking lifecycle—create, confirm, cancel, or expire your bookings.
//! - **Booking** — A flexible struct to hold booking details, like ID, status, and the item you're booking.
//! - **BookingStatus** — A friendly enum to manage all the possible states your booking can be in.
//! - **Error Types** — Robust error handling for any booking mishaps.
//! ---
//! 
//! ## 💡 Usage Example:
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use booking_kit::manager::BookingManager;
//! use booking_kit::traits::Bookable;
//! 
//! #[derive(Debug, Deserialize, Serialize)]
//! struct Room { id: &'static str }
//! impl Bookable for Room {
//!     fn id(&self) -> &str { self.id }
//!     fn is_available(&self) -> bool { true }
//! }
//! 
//! #[derive(Debug, Deserialize, Serialize)]
//! struct HotelRoom { id: &'static str, occupied: bool }
//! impl Bookable for HotelRoom {
//!     fn id(&self) -> &str { self.id }
//!     fn is_available(&self) -> bool { !self.occupied }
//! }
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let available_room = Room { id: "R101" };
//!     let booking = BookingManager::create(
//!         "booking-001", None, available_room, "now", None, None::<()>
//!     );
//!     
//!     let json = serde_json::to_string_pretty(&booking)?;
//!     println!("Booking : \n{}\n", json);
//!     println!("Booking created for: {}", booking.item_id());
//! 
//!     let occupied_room = HotelRoom { id: "R404", occupied: true };
//!     match BookingManager::try_create(
//!         "booking-002", Some("user-456"), occupied_room, "now", None, None::<()>
//!     ) {
//!         Ok(bk) => println!("Booking successful.{:?}", bk),
//!         Err(e) => println!("Booking failed: {}", e),
//!     }
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ---
//!
//! ## 📄 License
//!
//! Licensed under the [Apache-2.0 license](http://www.apache.org/licenses/LICENSE-2.0.txt) 📝
//!
//! ---
//!
//! ## 👨 Author
//!
//! Jerry Maheswara <jerrymaheswara@gmail.com>
//!
//! ---
//!
//! ## ❤️ Built with Love in Rust
//!
//! This project is built with ❤️ using **Rust** — a systems programming language that is safe, fast, and concurrent.  
//! Rust is the perfect choice for building reliable and efficient applications.
//!
//! ---
//!
//! ## 🤝 Contributing
//!
//! Pull requests, issues, and feedback are welcome!  
//! If you find this crate useful, give it a ⭐ and share it with others in the Rustacean community.
//!
//! ---

///! Core data model definitions for booking operations.
pub mod model;
pub mod manager;
pub mod traits;
pub mod error;


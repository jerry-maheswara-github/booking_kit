//! # booking_kit
//!
//! Extensible booking toolkit for Rust with core types, status logic, events, and validation rules.
//! 
//! `booking_kit` is a generic and extensible toolkit designed to handle the core logic for booking systems in Rust.
//! This crate provides structures, status management, events, validation rules, and error handling that are essential 
//! for creating a reservation or booking system. It is designed to be modular, flexible, and reusable across different 
//! applications.
//!
//! ## Features:
//!
//! - **Core Data Structures**: Defines `Booking`, `BookingItem`, and `BookingStatus` for handling bookings and items.
//! - **Status Management**: Offers predefined booking statuses such as `Pending`, `Confirmed`, `Canceled`, and more.
//! - **Domain Events**: Includes events like `BookingCreated` and `BookingCanceled` for tracking state transitions.
//! - **Rule Validation**: Allows custom validation rules for bookings, such as ensuring item availability or valid status transitions.
//! - **Error Handling**: Provides a set of errors like `ItemUnavailable`, `QuantityExceeded`, and `InvalidStatus` to handle common booking-related issues.
//!
//! This crate is highly reusable and adaptable to various contexts.
//!
//! ## Modules:
//!
//! - `booking`: Defines core entities like `Booking`, and related data structures.
//! - `status`: Contains the various statuses a booking can have, such as `Pending`, `Confirmed`, and `Canceled`.
//! - `traits`: Defines traits such as `Reservable` for entities that can be booked.
//! - `error`: Handles error types related to bookings, such as invalid status or unavailable items.
//!
//! ## Usage Example:
//!
//! ```code
//! 
//! ```
//!
//! This toolkit is designed to be extensible, allowing developers to build custom booking logic and integrate it into 
//! various types of applications. It provides the flexibility to manage bookings without needing to rely on specific 
//! external services, making it ideal for a variety of use cases.
//!
//! ---
//! 
//! ## License
//!
//! This project is licensed under the Apache-2.0 license. [LICENSE](http://www.apache.org/licenses/LICENSE-2.0.txt)
//!
//! ---
//!
//! ## Author
//! Jerry Maheswara <jerrymaheswara@gmail.com>
//!
//! ---
//!
//! ## Built with Love in Rust
//!
//! This project is built with ❤️ using **Rust** — a systems programming language that is safe, fast, and concurrent. Rust is the perfect choice for building reliable and efficient applications.
//!
//! ---
//!
//! ## Contributing
//!
//! Pull requests, issues, and feedback are welcome!
//! If you find this crate useful, give it a ⭐ and share it with others in the Rust community.
//!
//! ---

/// Core data model definitions for booking operations.
pub mod model;
pub mod manager;
pub mod traits;
pub mod error;


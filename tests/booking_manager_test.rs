#[cfg(test)]
mod tests {
    use booking_kit::error::BookingError;
    use booking_kit::manager::BookingManager;
    use booking_kit::model::status::BookingStatus;
    use booking_kit::traits::Bookable;

    // MockBookable simulates a bookable item for testing
    #[derive(Clone)]
    struct MockBookable {
        pub id: &'static str,
        pub available: bool,
    }

    impl Bookable for MockBookable {
        fn id(&self) -> &str {
            &self.id
        }

        fn is_available(&self) -> bool {
            self.available
        }
    }

    #[test]
    fn test_create_booking_success() {
        let item = MockBookable { id: "1", available: false };
        let booking = BookingManager::create(
            42,
            Some(100),
            item.clone(),
            123456,
            Some(123999),
            Some("notes".to_string()),
        );

        assert_eq!(booking.id, 42);
        assert_eq!(booking.status, BookingStatus::Pending);
        assert_eq!(booking.user_id, Some(100));
        assert_eq!(booking.item.id(), item.id());
    }

    #[test]
    fn test_try_create_success_when_available() {
        let item = MockBookable { id: "2", available: true };
        let result = BookingManager::try_create(
            99,
            Some(200),
            item.clone(),
            555555,
            None,
            None::<()>,
        );

        assert!(result.is_ok());
        let booking = result.unwrap();
        assert_eq!(booking.status, BookingStatus::Pending);
        assert_eq!(booking.item.id(), "2");
    }

    #[test]
    fn test_try_create_fails_when_unavailable() {
        let item = MockBookable { id: "3", available: false };
        let result = BookingManager::try_create(
            88,
            Some(300),
            item.clone(),
            777777,
            None,
            None::<()>,
        );

        assert!(result.is_err());
        if let Err(BookingError::ItemUnavailable(item_id)) = result {
            assert_eq!(item_id, "3");
        } else {
            panic!("Expected ItemUnavailable error");
        }
    }

    #[test]
    fn test_confirm_valid_transition() {
        let item = MockBookable { id: "4", available: true };
        let mut booking = BookingManager::create(1, None, item, 0, None, None::<()>);
        let result = BookingManager::confirm(&mut booking);

        assert!(result.is_ok());
        assert_eq!(booking.status, BookingStatus::Confirmed);
    }

    #[test]
    fn test_cancel_sets_status_to_canceled() {
        let item = MockBookable { id: "6", available: true };
        let mut booking = BookingManager::create(1, None, item, 0, None, None::<()>);
        BookingManager::cancel(&mut booking);
        assert_eq!(booking.status, BookingStatus::Canceled);
    }

    #[test]
    fn test_expire_sets_status_to_expired() {
        let item = MockBookable { id: "7", available: true };
        let mut booking = BookingManager::create(1, None, item, 0, None, None::<()>);
        BookingManager::expire(&mut booking);
        assert_eq!(booking.status, BookingStatus::Expired);
    }
}

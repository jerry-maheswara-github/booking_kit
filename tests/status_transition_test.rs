#[cfg(test)]
mod tests {
    use booking_kit::error::BookingError;
    use booking_kit::model::status::BookingStatus;

    #[test]
    fn test_invalid_status_transition() {
        // Trying to transition from 'Pending' to 'Expired' (which is invalid)
        let from_status = BookingStatus::Pending;
        let to_status = BookingStatus::Expired;

        // Create the InvalidStatusTransition error
        let error = BookingError::new_invalid_transition(from_status.clone(), to_status.clone());

        // Assert that the error is of type InvalidStatusTransition
        match error {
            BookingError::InvalidStatusTransition { from, to } => {
                assert_eq!(from, from_status);
                assert_eq!(to, to_status);
            }
            _ => panic!("Expected InvalidStatusTransition error"),
        }
    }

    #[test]
    fn test_invalid_status_transition_string() {
        // Create error and check the output string format
        let from_status = BookingStatus::Pending;
        let to_status = BookingStatus::Expired;

        let error = BookingError::new_invalid_transition(from_status.clone(), to_status.clone());

        // Assert the expected string
        let error_message = format!("{}", error);
        assert_eq!(error_message, "Invalid status transition from Pending to Expired");
    }
}

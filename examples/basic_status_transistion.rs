use booking_kit::model::status::BookingStatus;

fn main() {
    // Define some sample booking statuses
    let from = BookingStatus::Pending;
    let to = BookingStatus::Confirmed;

    // Check if the transition from `from` to `to` is valid
    if from.can_transition_to(&to) {
        println!("Valid transition from {:?} to {:?}", from, to);
    } else {
        println!("Invalid transition from {:?} to {:?}", from, to);
    }

    // Another example: Confirming a booking and then completing it
    let from = BookingStatus::Confirmed;
    let to = BookingStatus::Completed;

    if from.can_transition_to(&to) {
        println!("Valid transition from {:?} to {:?}", from, to);
    } else {
        println!("Invalid transition from {:?} to {:?}", from, to);
    }

    // Example of an invalid transition (Pending -> Expired)
    let from = BookingStatus::Pending;
    let to = BookingStatus::Expired;

    if from.can_transition_to(&to) {
        println!("Valid transition from {:?} to {:?}", from, to);
    } else {
        println!("Invalid transition from {:?} to {:?}", from, to);
    }
}

pub trait Bookable {
    fn id(&self) -> &str;
    fn is_available(&self) -> bool;
}

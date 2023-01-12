#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RentalStatus {
    Available,
    Rented(String),
}

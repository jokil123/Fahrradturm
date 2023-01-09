#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RentalStatus {
    Available,
    Rented(i32),
}

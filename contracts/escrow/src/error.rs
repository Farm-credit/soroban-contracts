use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EscrowError {
    InvalidAmount = 1,
    OfferCancelled = 2,
    InsufficientRemaining = 3,
    Unauthorized = 4,
    OfferNotFound = 5,
    AlreadyInitialized = 6,
    AmountMustBePositive = 7,
}
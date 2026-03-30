use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// The provided amount is negative.
    NegativeAmount = 1,
    /// The account does not have enough balance.
    InsufficientBalance = 2,
    /// The allowance is not enough for the transfer.
    InsufficientAllowance = 3,
    /// The contract is already initialized.
    AlreadyInitialized = 4,
    /// The provided expiration ledger is invalid (in the past).
    InvalidExpirationLedger = 5,
    /// The address is blacklisted.
    Blacklisted = 6,
    /// Caller is not authorized.
    Unauthorized = 7,
    /// Retirement amount must be greater than zero.
    ZeroRetirementAmount = 8,
    /// The successor address for super admin is invalid.
    InvalidSuccessor = 9,
    /// Only the SuperAdmin can blacklist/unblacklist themselves.
    CannotBlacklistSelf = 10,
    /// The report hash has already been used.
    ReportHashUsed = 11,
    /// The contract is paused.
    ContractPaused = 12,
}

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// The contract has already been initialized.
    AlreadyInitialized = 1,
    /// The contract has not been initialized yet.
    NotInitialized = 2,
    /// Caller is not the SuperAdmin.
    Unauthorized = 3,
    /// The address is already assigned to this role.
    RoleAlreadyAssigned = 4,
    /// The address does not have this role.
    RoleNotAssigned = 5,
    /// Cannot remove the SuperAdmin role from the SuperAdmin address.
    CannotRemoveSuperAdmin = 6,
    /// Invalid role type provided.
    InvalidRole = 7,
    /// The address already has a different role.
    AddressHasDifferentRole = 8,
    /// Threshold must be greater than 0 and less than or equal to the number of admins.
    InvalidThreshold = 9,
    /// Proposal not found.
    ProposalNotFound = 10,
    /// Proposal has already been executed.
    ProposalAlreadyExecuted = 11,
    /// Proposal has been rejected/cancelled.
    ProposalRejected = 12,
    /// Approver has already approved this proposal.
    AlreadyApproved = 13,
    /// The timelock delay has not yet expired.
    TimelockNotExpired = 14,
    /// Insufficient approvals to execute the proposal.
    InsufficientApprovals = 15,
    /// Empty admin list.
    EmptyAdmins = 16,
}


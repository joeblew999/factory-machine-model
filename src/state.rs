//! Machine state — the **OPC UA for Machinery** `MachineryItemState` state
//! machine (OPC 40001-1, building block for Machine Monitoring & KPIs).
//!
//! Maps to `Machines/<id>/MachineryBuildingBlocks/MachineryItemState` in the
//! gateway address space. Two orthogonal regions: the current condition
//! ([`MachineryItemState`]) and the operating mode ([`OperationMode`]).

use serde::{Deserialize, Serialize};

/// Current operational condition. The four states defined by the
/// `MachineryItemState_StateMachineType` (OPC 40001-1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MachineryItemState {
    /// Not ready to be used — e.g. powered off, not commissioned, offline.
    NotAvailable,
    /// Deliberately taken out of service (maintenance, fault hold).
    OutOfService,
    /// Available and idle — ready but not currently doing work.
    NotExecuting,
    /// Actively performing its function (running a job).
    Executing,
}

impl MachineryItemState {
    /// The spec BrowseName for this state.
    pub fn as_str(&self) -> &'static str {
        match self {
            MachineryItemState::NotAvailable => "NotAvailable",
            MachineryItemState::OutOfService => "OutOfService",
            MachineryItemState::NotExecuting => "NotExecuting",
            MachineryItemState::Executing => "Executing",
        }
    }
}

/// Operating mode — the `MachineryOperationModeStateMachineType` (OPC 40001-1),
/// orthogonal to [`MachineryItemState`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationMode {
    /// Mode not reported / not applicable.
    None,
    /// Normal production.
    Processing,
    /// Set-up / changeover.
    Setup,
    /// Maintenance.
    Maintenance,
}

impl OperationMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            OperationMode::None => "None",
            OperationMode::Processing => "Processing",
            OperationMode::Setup => "Setup",
            OperationMode::Maintenance => "Maintenance",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_browsenames() {
        assert_eq!(MachineryItemState::Executing.as_str(), "Executing");
        assert_eq!(OperationMode::Processing.as_str(), "Processing");
    }
}

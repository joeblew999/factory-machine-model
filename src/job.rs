//! Job dispatch — the **OPC UA for ISA-95 Job Control** model (OPC 10031-4 v2.0).
//!
//! A [`JobOrder`] is a request for a unit of work. The gateway exposes a
//! JobOrderReceiver (methods Store / StoreAndStart / Start / Stop / Cancel /
//! Pause / Resume / Abort / Clear) and tracks each order through its [`JobState`]
//! lifecycle; the assigned driver executes it via [`crate::MachineDriver::run_job`].
//!
//! The actual machine payload (e.g. a Howick cut-list CSV) rides as a
//! [`JobParameter`] — opaque to the gateway, meaningful only to the driver and
//! the upstream producer (e.g. `howick-rs`).

use serde::{Deserialize, Serialize};

/// An ISA-95 Job Order (`ISA95JobOrderDataType`, subset).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobOrder {
    /// `JobOrderID` — unique identifier of this order.
    pub job_order_id: String,
    /// `Description` — human-readable description.
    pub description: Option<String>,
    /// `WorkMasterID` — the recipe / definition this order instantiates.
    pub work_master_id: Option<String>,
    /// `JobOrderParameters` — the inputs, including the machine payload.
    pub parameters: Vec<JobParameter>,
}

impl JobOrder {
    /// Build a job carrying a single payload parameter (the common case).
    pub fn with_payload(
        job_order_id: impl Into<String>,
        payload_id: impl Into<String>,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            job_order_id: job_order_id.into(),
            description: None,
            work_master_id: None,
            parameters: vec![JobParameter {
                id: payload_id.into(),
                value: ParameterValue::Bytes(payload),
            }],
        }
    }

    /// First parameter whose value is bytes — the conventional machine payload.
    pub fn payload(&self) -> Option<&[u8]> {
        self.parameters.iter().find_map(|p| match &p.value {
            ParameterValue::Bytes(b) => Some(b.as_slice()),
            _ => None,
        })
    }
}

/// One `ISA95ParameterDataType`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobParameter {
    /// `ID` — parameter key, e.g. `"CutListCsv"`.
    pub id: String,
    pub value: ParameterValue,
}

/// A job-parameter value. Bytes carry opaque machine payloads (CSV, G-code, …).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParameterValue {
    Text(String),
    Number(f64),
    Bytes(Vec<u8>),
}

/// Job-order lifecycle state, using the ISA-95 Job Control state names
/// (OPC 10031-4): `Store` lands an order in `NotAllowedToStart`; `Start` /
/// `StoreAndStart` move it to `AllowedToStart`; execution begins → `Running`;
/// `Pause` → `Interrupted`; it ends `Completed` or `Aborted`.
///
/// In a fully-conformant server these are carried as `ISA95StateDataType`
/// (BrowseName + StateText + StateNumber); here we model the lifecycle directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobState {
    /// Stored in the receiver, not yet permitted to run.
    NotAllowedToStart,
    /// Permitted to run; will execute when the machine is ready.
    AllowedToStart,
    /// Currently executing on the machine.
    Running,
    /// Paused (after `Pause`), resumable via `Resume`.
    Interrupted,
    /// Finished successfully.
    Completed,
    /// Aborted / cancelled / stopped before completion.
    Aborted,
}

impl JobState {
    pub fn as_str(&self) -> &'static str {
        match self {
            JobState::NotAllowedToStart => "NotAllowedToStart",
            JobState::AllowedToStart => "AllowedToStart",
            JobState::Running => "Running",
            JobState::Interrupted => "Interrupted",
            JobState::Completed => "Completed",
            JobState::Aborted => "Aborted",
        }
    }

    /// Terminal states no longer eligible for execution.
    pub fn is_terminal(&self) -> bool {
        matches!(self, JobState::Completed | JobState::Aborted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_roundtrips() {
        let j = JobOrder::with_payload("T1-1", "CutListCsv", b"UNIT,MILLIMETRE".to_vec());
        assert_eq!(j.payload(), Some(&b"UNIT,MILLIMETRE"[..]));
        assert_eq!(j.job_order_id, "T1-1");
    }

    #[test]
    fn terminal_states() {
        assert!(JobState::Completed.is_terminal());
        assert!(!JobState::Running.is_terminal());
    }
}

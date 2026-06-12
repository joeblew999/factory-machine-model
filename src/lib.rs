//! # factory-machine-model
//!
//! The **gateway⇄driver contract** for factory-floor automation, modelled
//! directly on two OPC-UA companion specifications so the gateway exposes a
//! *standard* address space (any conformant SCADA / MES / historian interoperates):
//!
//! - **OPC UA for Machinery — OPC 40001-1** → machine identity & state:
//!   the [`Identification`] nameplate and the [`MachineryItemState`] state machine,
//!   organised under a well-known `Machines/` folder.
//!   <https://reference.opcfoundation.org/Machinery/v103/docs/>
//! - **OPC UA for ISA-95 Job Control — OPC 10031-4** → job dispatch:
//!   the [`JobOrder`] and its [`JobState`] lifecycle, handled by a JobOrderReceiver.
//!   <https://reference.opcfoundation.org/ISA95JOBCONTROL/docs/>
//!
//! A machine type is integrated by implementing [`MachineDriver`]. The gateway
//! depends only on this trait — never on a concrete driver — which is what lets
//! each driver be an independently-versioned crate composed per factory.

pub mod identification;
pub mod job;
pub mod state;
pub mod telemetry;

pub use identification::Identification;
pub use job::{JobOrder, JobParameter, JobState};
pub use state::{MachineryItemState, OperationMode};
pub use telemetry::{Telemetry, TelemetryField, Value, ValueKind};

/// Static description of a machine instance — what the gateway needs to build
/// this machine's subtree under the OPC-UA `Machines/` folder.
///
/// Returned by [`MachineDriver::descriptor`]. The `identification` populates the
/// standard Machinery nameplate; `telemetry` declares the machine-specific
/// variables (everything beyond the standard state + jobs).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MachineDescriptor {
    /// Instance id — the `Machines/<machine_id>` browse key, e.g. `"howick-1"`.
    pub machine_id: String,
    /// Machine type / driver kind, e.g. `"howick-frama"`, `"kuka"`.
    pub kind: String,
    /// Standard Machinery nameplate (OPC 40001-1 `Identification`).
    pub identification: Identification,
    /// Machine-specific telemetry variables (`Telemetry/` subtree).
    pub telemetry: Vec<TelemetryField>,
}

/// Per-machine-type runtime behaviour, implemented once per machine and run at
/// the edge (e.g. on the Pi wired to the machine). Object-safety is not required;
/// the gateway selects a driver at startup via its registry.
pub trait MachineDriver: Send + Sync {
    /// Identity + telemetry schema. Drives `Machines/<id>` node-tree construction.
    fn descriptor(&self) -> MachineDescriptor;

    /// Current standard machine state (OPC 40001-1 `MachineryItemState`).
    fn state(&self) -> impl std::future::Future<Output = MachineryItemState> + Send;

    /// Execute one ISA-95 [`JobOrder`] on the physical machine. Resolves when the
    /// machine has accepted the work (e.g. Howick: CSV written to the USB gadget).
    fn run_job(
        &self,
        job: &JobOrder,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    /// Read current telemetry. Field names must match [`MachineDescriptor::telemetry`].
    fn poll_telemetry(&self)
        -> impl std::future::Future<Output = anyhow::Result<Telemetry>> + Send;
}

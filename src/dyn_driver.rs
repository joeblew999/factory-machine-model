//! Object-safe driver façade.
//!
//! [`MachineDriver`](crate::MachineDriver) uses `impl Future` return types
//! (RPITIT) for zero-cost static dispatch — which is *not* `dyn`-compatible. The
//! gateway, however, must hold a heterogeneous set of drivers (Howick, robot,
//! CNC…) behind one type. [`DynMachineDriver`] is the boxed-future counterpart;
//! a blanket impl gives it to **every** [`MachineDriver`] for free, so drivers
//! implement only the ergonomic trait and the gateway stores
//! `Box<dyn DynMachineDriver>`.

use crate::{JobOrder, MachineDescriptor, MachineDriver, MachineryItemState, Telemetry};

/// Object-safe version of [`MachineDriver`]. Don't implement this directly — the
/// blanket impl below derives it from any [`MachineDriver`].
#[async_trait::async_trait]
pub trait DynMachineDriver: Send + Sync {
    fn descriptor(&self) -> MachineDescriptor;
    async fn state(&self) -> MachineryItemState;
    async fn run_job(&self, job: &JobOrder) -> anyhow::Result<()>;
    async fn poll_telemetry(&self) -> anyhow::Result<Telemetry>;
}

#[async_trait::async_trait]
impl<T: MachineDriver> DynMachineDriver for T {
    fn descriptor(&self) -> MachineDescriptor {
        MachineDriver::descriptor(self)
    }
    async fn state(&self) -> MachineryItemState {
        MachineDriver::state(self).await
    }
    async fn run_job(&self, job: &JobOrder) -> anyhow::Result<()> {
        MachineDriver::run_job(self, job).await
    }
    async fn poll_telemetry(&self) -> anyhow::Result<Telemetry> {
        MachineDriver::poll_telemetry(self).await
    }
}

/// A boxed driver as stored by the gateway.
pub type BoxedDriver = Box<dyn DynMachineDriver>;

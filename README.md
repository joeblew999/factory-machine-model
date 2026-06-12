# factory-machine-model

<https://github.com/joeblew999/factory-machine-model>

The **gateway⇄driver contract** for factory-floor automation. It models two
official **OPC-UA companion specifications** in Rust, so the gateway built on it
exposes a *standard* address space that any conformant SCADA / MES / historian
already understands — no bespoke integration.

Part of the `factory-` family:

| Repo | Role |
|------|------|
| **factory-machine-model** (this) | the OPC-UA-grounded contract — identity, state, jobs, telemetry, the `MachineDriver` trait |
| [factory-gateway](https://github.com/joeblew999/factory-gateway) | OPC-UA server: builds the address space, runs the job queue, hosts the dashboard |
| [factory-howick-driver](https://github.com/joeblew999/factory-howick-driver) | Howick FRAMA driver — implements `MachineDriver` |
| [factory-floor](https://github.com/joeblew999/factory-floor) | umbrella workspace + docs/ADRs; develop the whole stack locally |
| `factory-<machine>-driver` | one per machine type |

## The OPC-UA structure we implement

Every factory runs one gateway whose address space follows **OPC UA for Machinery**
(OPC 40001-1): a well-known `Machines/` folder with one object per machine, each
carrying a standard `Identification` nameplate, a `MachineryItemState`, and a
machine-specific `Telemetry/` subtree. Jobs are dispatched per **OPC UA for ISA-95
Job Control** (OPC 10031-4) through a `JobOrderReceiver`:

```text
Objects/
└── Machines/                                    ← OPC 40001-1 well-known folder
    └── <machine-id>/            e.g. howick-1
        ├── Identification/                       ← OPC 40001-1 nameplate
        │     Manufacturer · Model · SerialNumber · ProductInstanceUri
        │     DeviceClass · HardwareRevision · SoftwareRevision · YearOfConstruction
        ├── MachineryItemState                    ← OPC 40001-1 state machine
        │     NotAvailable | OutOfService | NotExecuting | Executing
        │     (+ OperationMode: Processing | Setup | Maintenance)
        ├── Telemetry/                            ← machine-specific, driver-declared
        │     e.g. Howick → PiecesProduced, CoilRemaining
        └── JobOrderReceiver                      ← OPC 10031-4 ISA-95 Job Control
              methods: Store · StoreAndStart · Start · Stop · Cancel · Pause · Resume · Abort · Clear
              JobOrder{ JobOrderID, Description, WorkMasterID, JobOrderParameters[] }
              JobState: Stored → Queued → Running → Ended | Aborted | Interrupted
```

The machine payload (a Howick cut-list CSV, a robot program, …) rides inside a
`JobOrder` as a `JobParameter` — opaque to the gateway, meaningful only to the driver.

| Rust type (this crate) | OPC-UA standard |
|------------------------|-----------------|
| [`Identification`]      | OPC 40001-1 `MachineIdentificationType` nameplate |
| [`MachineryItemState`] / [`OperationMode`] | OPC 40001-1 `MachineryItemState` state machine |
| [`JobOrder`] / [`JobParameter`] / [`JobState`] | OPC 10031-4 `ISA95JobOrder` + Job Control lifecycle |
| [`TelemetryField`] / [`Value`] | per-machine variables under `Telemetry/` |
| [`MachineDriver`]       | the Rust seam each driver implements |

## Adding a machine type

Implement [`MachineDriver`] in a new `factory-<machine>-driver` crate:

```rust
use factory_machine_model::*;

impl MachineDriver for MyMachine {
    fn descriptor(&self) -> MachineDescriptor {
        MachineDescriptor {
            machine_id: "my-machine-1".into(),
            kind: "my-machine".into(),
            identification: Identification::new("Acme", "X1"),
            telemetry: vec![TelemetryField::new("Temperature", ValueKind::Double, Some("°C"))],
        }
    }
    async fn state(&self) -> MachineryItemState { MachineryItemState::NotExecuting }
    async fn run_job(&self, job: &JobOrder) -> anyhow::Result<()> { /* job.payload() */ Ok(()) }
    async fn poll_telemetry(&self) -> anyhow::Result<Telemetry> { Ok(Telemetry::new()) }
}
```

The gateway needs **zero** changes to take on a new machine type.

## Standards

- OPC UA for Machinery — OPC 40001-1: <https://reference.opcfoundation.org/Machinery/v103/docs/>
- OPC UA for ISA-95 Job Control — OPC 10031-4: <https://reference.opcfoundation.org/ISA95JOBCONTROL/docs/>

## Licence

MIT OR Apache-2.0.

# factory-machine-model

<https://github.com/joeblew999/factory-machine-model>

The **gateway⇄driver contract** for factory-floor automation — the small, stable
crate that lets one generic OPC-UA gateway serve many machine types (roll-formers,
robots, CNCs) and lets SCADA speak to all of them without knowing what any of
them is.

Part of the `factory-` family:

| Repo | Role |
|------|------|
| **factory-machine-model** (this) | the shared contract — `MachineDescriptor`, `MachineDriver`, `Telemetry` |
| [factory-gateway](https://github.com/joeblew999/factory-gateway) | OPC-UA server + job queue + dashboard (builds the node-tree from descriptors) |
| [factory-howick-driver](https://github.com/joeblew999/factory-howick-driver) | Howick FRAMA edge driver — implements `MachineDriver` |
| `factory-<machine>-driver` | one per machine type, same shape |

## The model

Every machine appears under `/Machines/<MachineId>`. `Identity`, `Status`, and
`Jobs` are **fixed** — written once in the gateway and reused for every machine.
The `Telemetry/` subtree is **built dynamically** from each driver's declared schema:

```text
/Machines/<MachineId>/
  Identity/   MachineId · Kind · Vendor · Model          ── fixed
  Status/     State · CurrentJobId · LastError           ── fixed
  Jobs/       QueueDepth · Pending/* · CompleteJob()      ── fixed
  Telemetry/  ‹driver-declared›                           ── per machine type
```

## Adding a machine type

Implement [`MachineDriver`] in a new `factory-<machine>-driver` repo:

```rust
use factory_machine_model::*;

struct MyMachine { /* ... */ }

impl MachineDriver for MyMachine {
    fn descriptor(&self) -> MachineDescriptor {
        MachineDescriptor {
            machine_id: "my-machine-1".into(),
            kind: "my-machine".into(),
            vendor: "Acme".into(),
            model: "X1".into(),
            job_format: JobFormat::Gcode,
            telemetry: vec![
                TelemetryField::new("Temperature", ValueKind::Double, Some("°C")),
            ],
        }
    }
    async fn run_job(&self, job: &JobPayload) -> anyhow::Result<()> { /* ... */ Ok(()) }
    async fn poll_telemetry(&self) -> anyhow::Result<Telemetry> { /* ... */ Ok(Telemetry::new()) }
}
```

The gateway needs **zero** changes to take on a new machine type — that is the
whole point of this crate.

## Design

See ADR-0006 (`docs/adr/0006-standard-machine-model.md` in the gateway repo).

## Licence

MIT OR Apache-2.0.

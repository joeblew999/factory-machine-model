//! Machine-specific telemetry — the variables a driver exposes *beyond* the
//! standard [`crate::state`] and [`crate::job`] models. Each becomes a variable
//! under `Machines/<id>/Telemetry/` in the gateway address space.
//!
//! This is the only part of a machine's node-tree that varies by machine type
//! (e.g. Howick → `CoilRemaining`, a robot → `TcpX`/`TcpY`/`TcpZ`).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// One driver-declared telemetry variable (becomes `Telemetry/<name>`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelemetryField {
    /// OPC-UA BrowseName, e.g. `"CoilRemaining"`.
    pub name: String,
    /// Value type — drives the OPC-UA DataType of the variable node.
    pub kind: ValueKind,
    /// Engineering unit for display, e.g. `Some("m")`. `None` for counts.
    pub unit: Option<String>,
}

impl TelemetryField {
    pub fn new(name: impl Into<String>, kind: ValueKind, unit: Option<&str>) -> Self {
        Self {
            name: name.into(),
            kind,
            unit: unit.map(str::to_owned),
        }
    }
}

/// Type tag for a telemetry field / [`Value`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueKind {
    Bool,
    Int,
    UInt,
    Double,
    String,
}

/// A telemetry reading.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Bool(bool),
    Int(i64),
    UInt(u64),
    Double(f64),
    String(String),
}

impl Value {
    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Bool(_) => ValueKind::Bool,
            Value::Int(_) => ValueKind::Int,
            Value::UInt(_) => ValueKind::UInt,
            Value::Double(_) => ValueKind::Double,
            Value::String(_) => ValueKind::String,
        }
    }

    /// Numeric coercion to f64 for dashboard / JSON output (`None` for strings/bools).
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Int(v) => Some(*v as f64),
            Value::UInt(v) => Some(*v as f64),
            Value::Double(v) => Some(*v),
            _ => None,
        }
    }
}

/// Telemetry snapshot — `field name → value`. Synced to the `Telemetry/` nodes.
pub type Telemetry = BTreeMap<String, Value>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_kind_roundtrips() {
        assert_eq!(Value::UInt(3).kind(), ValueKind::UInt);
        assert_eq!(Value::Double(1.5).as_f64(), Some(1.5));
        assert_eq!(Value::String("x".into()).as_f64(), None);
    }
}

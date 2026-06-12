//! Machine identity — the **OPC UA for Machinery** `Identification` nameplate
//! (OPC 40001-1, building block 2: `MachineIdentificationType`, derived from the
//! DI `IVendorNameplateType`).
//!
//! Maps to `Machines/<id>/Identification/*` in the gateway address space.

use serde::{Deserialize, Serialize};

/// Standard Machinery nameplate. Optional fields are `None` when the machine
/// doesn't report them; required-by-spec fields (`manufacturer`, `model`) are
/// plain `String`. Field names mirror the spec's BrowseNames.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Identification {
    /// `Manufacturer` — vendor display name. (Spec: LocalizedText.)
    pub manufacturer: String,
    /// `ManufacturerUri` — globally-unique vendor URI.
    pub manufacturer_uri: Option<String>,
    /// `Model` — model name. (Spec: LocalizedText.)
    pub model: String,
    /// `ProductCode` — vendor-specific product/order code.
    pub product_code: Option<String>,
    /// `ProductInstanceUri` — globally-unique id of this physical instance.
    pub product_instance_uri: Option<String>,
    /// `SerialNumber` — unique within the manufacturer's `ManufacturerUri`.
    pub serial_number: Option<String>,
    /// `DeviceClass` — class of device, e.g. `"RollFormer"`, `"Robot"`.
    pub device_class: Option<String>,
    /// `HardwareRevision`.
    pub hardware_revision: Option<String>,
    /// `SoftwareRevision`.
    pub software_revision: Option<String>,
    /// `YearOfConstruction` (4-digit).
    pub year_of_construction: Option<u16>,
    /// `Location` — human-readable installation location.
    pub location: Option<String>,
}

impl Identification {
    /// The two spec-required fields; everything else defaults to `None`.
    pub fn new(manufacturer: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            manufacturer: manufacturer.into(),
            model: model.into(),
            ..Default::default()
        }
    }
}

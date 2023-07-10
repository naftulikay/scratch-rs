#[cfg(test)]
mod tests;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::compat;

// FIXME idea here is to essentially create an *internal* private mapping direct to the XML via
//       serde, transform it to an actually functional model, and on the struct itself, provide
//       methods like `from_reader` and `from_str` to extract a public model which isn't totally
//       insane.

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename = "device", rename_all = "kebab-case")]
pub struct Device {
    #[serde(
        serialize_with = "compat::serialize_naive_datetime_timestamp_to_str",
        deserialize_with = "compat::deserialize_naive_date_timestamp"
    )]
    pub timestamp: NaiveDateTime,
    outputs: DeviceOutputs,
    record_outputs: RecordOutputs,
    inputs: DeviceInputs,
    pub settings: DeviceSettings,
    pub clocking: DeviceClocking,
    pub monitoring: DeviceMonitoring,
}

impl Device {
    pub fn outputs(&self) -> &[Output] {
        self.outputs.as_slice()
    }

    pub fn record_outputs(&self) -> &[RecordOutput] {
        self.record_outputs.as_slice()
    }

    pub fn inputs(&self) -> &[Input] {
        self.inputs.as_slice()
    }
}

pub type FocusriteProfile = Device;

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct DeviceOutputs {
    #[serde(rename = "$value")]
    pub v: Vec<Output>,
}

impl Deref for DeviceOutputs {
    type Target = Vec<Output>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Output {
    #[serde(rename = "analogue")]
    Analog(AnalogOutput),
    #[serde(rename = "spdif-rca")]
    SpdifRca(SpdifRcaOutput),
    #[serde(rename = "loopback")]
    Loopback(LoopbackOutput),
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AnalogOutput {
    pub stereo: bool,
    pub mute: bool,
    #[serde(default)]
    pub source: Option<String>,
    // FIXME parse empty strings as Option::None
    pub nickname: String,
    pub gain: i16,
    pub hardware_control: bool,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct SpdifRcaOutput {
    pub stereo: bool,
    pub mute: bool,
    #[serde(default)]
    pub source: Option<String>,
    // FIXME parse empty strings as Option::None
    pub nickname: String,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LoopbackOutput {
    pub stereo: bool,
    pub mute: bool,
    #[serde(default)]
    pub source: Option<String>,
    // FIXME parse empty strings as Option::None
    pub nickname: String,
}

/// A set of `<record/>` elements for defining record output settings.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct RecordOutputs {
    #[serde(rename = "$value")]
    v: Vec<RecordOutput>,
}

impl Deref for RecordOutputs {
    type Target = Vec<RecordOutput>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

/// Record outputs seem to be Windows-specific types, as essentially each application is on its own
/// for managing these things, as opposed to macOS and Linux having subsystems for these, e.g.
/// ALSA, Pipewire.
///
/// Some instances of these are simply blank and this appears to be the case when stereo is used,
/// things are picked up from the previous record output.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RecordOutput {
    /// This appears to reference a specific analog input.
    #[serde(default)]
    pub hardware_input: Option<u8>,
    /// Unsure exactly whot this means: does `m` mean `monitor`?
    #[serde(default)]
    pub hardware_input_m: Option<u8>,
    /// Unsure exactly what this means: does `h` mean `headphones`?
    #[serde(default)]
    pub hardware_input_h: Option<u8>,
}

/// A set of input configurations of various types.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct DeviceInputs {
    #[serde(rename = "$value")]
    v: Vec<Input>,
}

impl Deref for DeviceInputs {
    type Target = Vec<Input>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

/// Representation of an individual input of an arbitrary type.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Input {
    #[serde(rename = "analogue")]
    Analog(AnalogInput),
    #[serde(rename = "adat")]
    Adat(AdatInput),
    #[serde(rename = "playback")]
    Playback(PlaybackInput),
    #[serde(rename = "spdif-rca")]
    SpdifRca(SpdifRcaInput),
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AnalogInput {
    // FIXME replace with Option<String>
    pub nickname: String,
    #[serde(default)]
    pub air: Option<bool>,
    #[serde(default)]
    pub pad: Option<bool>,
    pub mode: Option<AnalogInputMode>,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum AnalogInputMode {
    #[serde(rename = "Line")]
    Line,
    #[serde(rename = "Inst")]
    Instrument,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct SpdifRcaInput {
    // FIXME should be Option<String>
    pub nickname: String,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct AdatInput {
    // FIXME should be Option<String>
    pub nickname: String,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct PlaybackInput {
    // FIXME should be Option<String>
    pub nickname: String,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeviceSettings {
    // NOTE not sure if this is optional but we're treating it like that
    pub spdif_mode: Option<String>,
    // NOTE I think that only certain devices have this option
    pub phantom_persistence: Option<bool>,
}

// NOTE I don't know what settings this might contain
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct DeviceClocking {}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct DeviceMonitoring {
    // NOTE not sure if this is optional
    pub preset: Option<String>,
}

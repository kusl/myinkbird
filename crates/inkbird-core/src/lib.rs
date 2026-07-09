//! Pure decoding of INKBIRD **ITH-13-B** Bluetooth Low Energy advertisements.
//!
//! This crate is deliberately tiny and free of I/O, async, Bluetooth, and
//! serialization concerns. It takes the raw bytes of a BLE advertisement's
//! reconstructed manufacturer message and returns a validated
//! [`SensorReading`], or a [`ParseError`] describing why the packet was
//! rejected. Keeping the decode logic pure means every branch can be covered
//! by fast, hardware-free unit tests (see the `parser` module tests).
//!
//! # What "reconstructed message" means
//!
//! On the wire, a BLE advertisement carries *manufacturer specific data* as a
//! 16-bit little-endian company identifier followed by a payload. Most BLE
//! stacks (including `btleplug`) hand you the payload already keyed by company
//! id, i.e. a `HashMap<u16, Vec<u8>>`. The INKBIRD parser works on the
//! *message*, which is the company id bytes glued back onto the front of the
//! payload:
//!
//! ```text
//! message = company_id.to_le_bytes() ++ payload
//! ```
//!
//! Use [`build_message`] to reconstruct it and [`parse_manufacturer_entry`]
//! to go straight from a `(company_id, payload)` pair to a [`SensorReading`].
//!
//! # ITH-13-B byte layout (18-byte model)
//!
//! ```text
//! offset  bytes  field
//! ------  -----  --------------------------------------------
//! 0..2    2      company id (little-endian)
//! 2..6    4      unknown / device-specific
//! 6..8    2      temperature, i16 little-endian, value / 10 = degrees C
//! 8..10   2      humidity,    u16 little-endian, value / 10 = %RH
//! 10      1      battery percent (0..=100)
//! 11..18  7      unknown / device-specific
//! ```
//!
//! Temperature is **signed** (a sensor in a freezer legitimately reads below
//! zero). Humidity is published only when its raw value is non-zero. A reading
//! whose humidity exceeds 100 %RH or whose battery exceeds 100 % is treated as
//! a corrupt packet and the **whole** reading is dropped (see
//! [`ParseError`]). This mirrors the reference behaviour of the
//! `Bluetooth-Devices/inkbird-ble` project.

mod model;
mod parser;

pub use model::{ParseError, SensorReading};
pub use parser::{
    INKBIRD_SERVICE_UUID_16, ITH_13_B_LOCAL_NAME, ITH_13_B_MESSAGE_LEN, MIN_MESSAGE_LEN,
    build_message, looks_like_ith_13_b, parse_ith_13_b, parse_manufacturer_entry,
};

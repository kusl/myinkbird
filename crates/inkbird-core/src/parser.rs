//! The ITH-13-B advertisement decoder.
//!
//! See the crate-level docs for the byte layout. This module is a direct,
//! test-covered port of the reference decode used by the
//! `Bluetooth-Devices/inkbird-ble` project for 18-byte INKBIRD sensors.

use crate::model::{ParseError, SensorReading};

/// Lower-cased advertised local name of the ITH-13-B.
pub const ITH_13_B_LOCAL_NAME: &str = "ith-13-b";

/// The 16-bit INKBIRD service UUID (`0000fff0-0000-1000-8000-00805f9b34fb`).
///
/// Present in the advertisement's service-UUID list; useful as a secondary
/// identification signal alongside the local name.
pub const INKBIRD_SERVICE_UUID_16: u16 = 0xfff0;

/// Total length in bytes of a complete ITH-13-B message
/// (`company_id` + payload).
pub const ITH_13_B_MESSAGE_LEN: usize = 18;

/// Minimum message length required to decode temperature, humidity and
/// battery. Anything shorter is rejected with [`ParseError::TooShort`].
pub const MIN_MESSAGE_LEN: usize = 11;

/// Relative humidity cannot exceed this value; higher marks a corrupt packet.
const MAX_HUMIDITY_PCT: f64 = 100.0;

/// Battery percentage cannot exceed this value; higher marks a corrupt packet.
const MAX_BATTERY_PCT: u8 = 100;

/// Reconstruct the full advertisement *message* from a company id and payload.
///
/// The message is the 2-byte little-endian company identifier followed by the
/// manufacturer payload, i.e. `company_id.to_le_bytes() ++ payload`. This is
/// the byte string the [`parse_ith_13_b`] decoder expects.
#[must_use]
pub fn build_message(company_id: u16, payload: &[u8]) -> Vec<u8> {
    let mut message = Vec::with_capacity(2 + payload.len());
    message.extend_from_slice(&company_id.to_le_bytes());
    message.extend_from_slice(payload);
    message
}

/// Decode a single manufacturer-data entry into a [`SensorReading`].
///
/// Convenience wrapper that reconstructs the message with [`build_message`]
/// and then calls [`parse_ith_13_b`].
///
/// # Errors
///
/// Returns a [`ParseError`] if the reconstructed message is too short or
/// carries an implausible humidity or battery value.
pub fn parse_manufacturer_entry(
    company_id: u16,
    payload: &[u8],
) -> Result<SensorReading, ParseError> {
    let message = build_message(company_id, payload);
    parse_ith_13_b(&message)
}

/// Decode a complete ITH-13-B advertisement message into a [`SensorReading`].
///
/// `data` must be the reconstructed message (company id bytes + payload). See
/// the crate-level docs for the exact byte layout.
///
/// # Errors
///
/// * [`ParseError::TooShort`] if `data.len() < MIN_MESSAGE_LEN`.
/// * [`ParseError::ImplausibleHumidity`] if humidity decodes above 100 %RH.
/// * [`ParseError::ImplausibleBattery`] if battery decodes above 100 %.
pub fn parse_ith_13_b(data: &[u8]) -> Result<SensorReading, ParseError> {
    if data.len() < MIN_MESSAGE_LEN {
        return Err(ParseError::TooShort {
            need: MIN_MESSAGE_LEN,
            got: data.len(),
        });
    }

    // Temperature is signed (negatives are valid); humidity is unsigned.
    let temp_raw = i16::from_le_bytes([data[6], data[7]]);
    let hum_raw = u16::from_le_bytes([data[8], data[9]]);
    let battery = data[10];

    let humidity = f64::from(hum_raw) / 10.0;
    if humidity > MAX_HUMIDITY_PCT {
        return Err(ParseError::ImplausibleHumidity(humidity));
    }
    if battery > MAX_BATTERY_PCT {
        return Err(ParseError::ImplausibleBattery(battery));
    }

    Ok(SensorReading {
        temperature_c: Some(f64::from(temp_raw) / 10.0),
        // Humidity is only meaningful when the sensor reported it.
        humidity_pct: if hum_raw != 0 { Some(humidity) } else { None },
        battery_pct: Some(battery),
    })
}

/// Heuristic: does this advertised local name look like an ITH-13-B?
///
/// Matches case-insensitively and by substring, so both `"ITH-13-B"` and a
/// vendor-prefixed name like `"Inkbird ITH-13-B"` are recognised. Returns
/// `false` for `None` (a passive scan may not carry the name at all - in that
/// case identify the device by its Bluetooth address instead).
#[must_use]
pub fn looks_like_ith_13_b(local_name: Option<&str>) -> bool {
    match local_name {
        Some(name) => name.to_ascii_lowercase().contains(ITH_13_B_LOCAL_NAME),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compare two floats within a small epsilon.
    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    /// A known-good 18-byte message: temp 28.9 C, humidity 45.5 %, battery 100.
    ///
    /// Layout: company id `0xCDAB`, 4 unknown, temp `0x0121` = 289, humidity
    /// `0x01C7` = 455, battery `0x64` = 100, then 7 trailing bytes.
    fn good_message() -> Vec<u8> {
        vec![
            0xAB, 0xCD, // company id (LE)
            0x11, 0x22, 0x33, 0x44, // unknown
            0x21, 0x01, // temperature 289 -> 28.9
            0xC7, 0x01, // humidity 455 -> 45.5
            0x64, // battery 100
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // trailing
        ]
    }

    #[test]
    fn good_message_is_eighteen_bytes() {
        assert_eq!(good_message().len(), ITH_13_B_MESSAGE_LEN);
    }

    #[test]
    fn decodes_temperature_humidity_battery() {
        let r = parse_ith_13_b(&good_message()).expect("should decode");
        assert!(approx(r.temperature_c.unwrap(), 28.9));
        assert!(approx(r.humidity_pct.unwrap(), 45.5));
        assert_eq!(r.battery_pct, Some(100));
    }

    #[test]
    fn floats_format_cleanly() {
        // Rust's shortest-round-trip Display (same family as serde_json's ryu)
        // yields tidy NDJSON values with no trailing noise.
        let r = parse_ith_13_b(&good_message()).unwrap();
        assert_eq!(format!("{}", r.temperature_c.unwrap()), "28.9");
        assert_eq!(format!("{}", r.humidity_pct.unwrap()), "45.5");
    }

    #[test]
    fn zero_humidity_is_omitted_but_temp_and_battery_remain() {
        let mut msg = good_message();
        msg[8] = 0x00;
        msg[9] = 0x00;
        let r = parse_ith_13_b(&msg).unwrap();
        assert_eq!(r.humidity_pct, None);
        assert!(approx(r.temperature_c.unwrap(), 28.9));
        assert_eq!(r.battery_pct, Some(100));
    }

    #[test]
    fn implausible_humidity_drops_whole_reading() {
        let mut msg = good_message();
        msg[8] = 0xFF;
        msg[9] = 0xFF; // 65535 -> 6553.5 %
        assert_eq!(
            parse_ith_13_b(&msg),
            Err(ParseError::ImplausibleHumidity(6553.5))
        );
    }

    #[test]
    fn implausible_battery_drops_whole_reading() {
        let mut msg = good_message();
        msg[10] = 0xFF; // 255 %
        assert_eq!(
            parse_ith_13_b(&msg),
            Err(ParseError::ImplausibleBattery(255))
        );
    }

    #[test]
    fn negative_temperature_decodes() {
        let mut msg = good_message();
        let le = (-55i16).to_le_bytes(); // -5.5 C
        msg[6] = le[0];
        msg[7] = le[1];
        let r = parse_ith_13_b(&msg).unwrap();
        assert!(approx(r.temperature_c.unwrap(), -5.5));
    }

    #[test]
    fn too_short_is_rejected() {
        let msg = good_message();
        assert_eq!(
            parse_ith_13_b(&msg[..10]),
            Err(ParseError::TooShort { need: 11, got: 10 })
        );
    }

    #[test]
    fn empty_slice_is_rejected() {
        assert_eq!(
            parse_ith_13_b(&[]),
            Err(ParseError::TooShort { need: 11, got: 0 })
        );
    }

    #[test]
    fn battery_boundary_values() {
        let mut ok = good_message();
        ok[10] = 100;
        assert!(parse_ith_13_b(&ok).is_ok());

        let mut bad = good_message();
        bad[10] = 101;
        assert_eq!(
            parse_ith_13_b(&bad),
            Err(ParseError::ImplausibleBattery(101))
        );
    }

    #[test]
    fn humidity_boundary_values() {
        let mut ok = good_message();
        let le = 1000u16.to_le_bytes(); // exactly 100.0 %
        ok[8] = le[0];
        ok[9] = le[1];
        assert!(approx(parse_ith_13_b(&ok).unwrap().humidity_pct.unwrap(), 100.0));

        let mut bad = good_message();
        let le = 1001u16.to_le_bytes(); // 100.1 %
        bad[8] = le[0];
        bad[9] = le[1];
        assert!(matches!(
            parse_ith_13_b(&bad),
            Err(ParseError::ImplausibleHumidity(_))
        ));
    }

    #[test]
    fn zero_degrees_decodes() {
        let mut msg = good_message();
        msg[6] = 0x00;
        msg[7] = 0x00;
        let r = parse_ith_13_b(&msg).unwrap();
        assert!(approx(r.temperature_c.unwrap(), 0.0));
    }

    #[test]
    fn build_message_prepends_company_id_little_endian() {
        let msg = build_message(0xCDAB, &[0x11, 0x22]);
        assert_eq!(msg, vec![0xAB, 0xCD, 0x11, 0x22]);
    }

    #[test]
    fn parse_manufacturer_entry_matches_manual_reconstruction() {
        // Payload is the message minus the leading 2 company-id bytes.
        let full = good_message();
        let payload = &full[2..];
        let company_id = u16::from_le_bytes([full[0], full[1]]);
        let via_entry = parse_manufacturer_entry(company_id, payload).unwrap();
        let via_message = parse_ith_13_b(&full).unwrap();
        assert_eq!(via_entry, via_message);
    }

    #[test]
    fn name_matching_is_case_insensitive_and_substring() {
        assert!(looks_like_ith_13_b(Some("ITH-13-B")));
        assert!(looks_like_ith_13_b(Some("ith-13-b")));
        assert!(looks_like_ith_13_b(Some("Inkbird ITH-13-B sensor")));
        assert!(!looks_like_ith_13_b(Some("IBS-TH2")));
        assert!(!looks_like_ith_13_b(Some("")));
        assert!(!looks_like_ith_13_b(None));
    }

    #[test]
    fn sensor_reading_empty_helpers() {
        let e = SensorReading::empty();
        assert!(e.is_empty());
        let r = parse_ith_13_b(&good_message()).unwrap();
        assert!(!r.is_empty());
    }

    #[test]
    fn error_messages_are_human_readable() {
        let e = ParseError::TooShort { need: 11, got: 3 };
        assert!(e.to_string().contains("too short"));
        let e = ParseError::ImplausibleHumidity(6553.5);
        assert!(e.to_string().contains("humidity"));
        let e = ParseError::ImplausibleBattery(255);
        assert!(e.to_string().contains("battery"));
    }

    #[test]
    fn service_uuid_constant_is_fff0() {
        assert_eq!(INKBIRD_SERVICE_UUID_16, 0xfff0);
    }
}

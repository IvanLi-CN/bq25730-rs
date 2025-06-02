#![cfg(feature = "binrw")]

use binrw::{BinRead, BinWrite, Endian, io::Cursor};
use bq25730_async_rs::data_types::*;
use bq25730_async_rs::registers::*;

/// Helper function to test binrw roundtrip serialization/deserialization.
///
/// S: Type to serialize/deserialize
///
/// This function serializes the `original` value to a buffer, then deserializes
/// it back into a new value, and asserts that the new value is equal to the
/// original.
fn test_binrw_roundtrip<S>(original: S)
where
    for<'a> S: BinRead<Args<'a> = ()> + BinWrite<Args<'a> = ()> + PartialEq + std::fmt::Debug,
{
    let mut buffer = Cursor::new(Vec::new());

    // Serialize the original value
    original
        .write_options(&mut buffer, Endian::Little, ())
        .expect("Failed to serialize");

    // Rewind the buffer to the beginning
    buffer.set_position(0);

    // Deserialize the value back
    let deserialized =
        S::read_options(&mut buffer, Endian::Little, ()).expect("Failed to deserialize");

    // Assert that the deserialized value is equal to the original
    assert_eq!(original, deserialized);
}

/// Test that `ChargerStatus` can be serialized and deserialized correctly.
///
/// This test creates a `ChargerStatus` with all possible flags set, then
/// serializes it to a buffer and deserializes it back. It then asserts that the
/// deserialized value is equal to the original.
#[test]
fn test_charger_status_binrw_roundtrip() {
    let original = ChargerStatus {
        status_flags: ChargerStatusFlags::from_bits_truncate(0b10101010),
        fault_flags: ChargerStatusFaultFlags::from_bits_truncate(0b01010101),
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_prochot_status_binrw_roundtrip() {
    let original = ProchotStatus {
        msb_flags: ProchotStatusMsbFlags::from_bits_truncate(0b11001100),
        lsb_flags: ProchotStatusFlags::from_bits_truncate(0b00110011),
        prochot_width: 0b10,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_charge_current_binrw_roundtrip() {
    let original = ChargeCurrentSetting {
        milliamps: 1024,
        rsns_bat: SenseResistorValue::R5mOhm,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_charge_voltage_binrw_roundtrip() {
    let original = bq25730_async_rs::data_types::ChargeVoltageSetting { millivolts: 16000 }; // Example value in mV
    test_binrw_roundtrip(original);
}

#[test]
fn test_otg_voltage_binrw_roundtrip() {
    let original = bq25730_async_rs::data_types::OtgVoltageSetting { millivolts: 5000 }; // Example value in mV
    test_binrw_roundtrip(original);
}

#[test]
fn test_otg_current_binrw_roundtrip() {
    let original = OtgCurrentSetting {
        milliamps: 2000,
        rsns_bat: SenseResistorValue::R5mOhm,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_input_voltage_binrw_roundtrip() {
    let original = bq25730_async_rs::data_types::InputVoltageSetting { millivolts: 11968 }; // Example value in mV (11968 - 3200 = 8768 = 64 * 137)
    test_binrw_roundtrip(original);
}

#[test]
fn test_vsys_min_binrw_roundtrip() {
    let original = bq25730_async_rs::data_types::VsysMinSetting { millivolts: 3500 }; // Example value in mV
    test_binrw_roundtrip(original);
}

#[test]
fn test_iin_host_binrw_roundtrip() {
    let original = IinHost {
        milliamps: 3000,
        rsns_ac: SenseResistorValue::R5mOhm,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_iin_dpm_binrw_roundtrip() {
    let original = IinDpm {
        milliamps: 2500,
        rsns_ac: SenseResistorValue::R5mOhm,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_adc_measurements_binrw_roundtrip() {
    let original = AdcMeasurements {
        vbat: AdcVbat(3776),
        vsys: AdcVsys(3968),
        ichg: AdcIchg {
            milliamps: 500,
            rsns_bat: SenseResistorValue::R5mOhm,
        },
        idchg: AdcIdchg {
            milliamps: 1000,
            rsns_bat: SenseResistorValue::R5mOhm,
        },
        iin: AdcIin {
            milliamps: 800,
            rsns_ac: SenseResistorValue::R5mOhm,
        }, // 8 * 100mA/LSB for 5mOhm
        psys: AdcPsys(600),
        vbus: AdcVbus(4992),
        cmpin: AdcCmpin(96),
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_adccmpin_binrw_roundtrip() {
    let original = AdcCmpin(96); // Example scaled value (96 = 12 * 8)
    test_binrw_roundtrip(original);
}

#[test]
fn test_adcichg_binrw_roundtrip() {
    let original = AdcIchg {
        milliamps: 500,
        rsns_bat: SenseResistorValue::R5mOhm,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_adcidchg_binrw_roundtrip() {
    let original = AdcIdchg {
        milliamps: 1000,
        rsns_bat: SenseResistorValue::R5mOhm,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_adciin_binrw_roundtrip() {
    let original = AdcIin {
        milliamps: 800,
        rsns_ac: SenseResistorValue::R5mOhm,
    }; // 8 * 100mA/LSB for 5mOhm
    test_binrw_roundtrip(original);
}

#[test]
fn test_adcpsys_binrw_roundtrip() {
    let original = AdcPsys(600); // Example scaled value
    test_binrw_roundtrip(original);
}

#[test]
fn test_adcvbus_binrw_roundtrip() {
    let original = AdcVbus(4992); // Example value in mV (4992 = 96 * 52)
    test_binrw_roundtrip(original);
}

#[test]
fn test_adcvbat_binrw_roundtrip() {
    let original = AdcVbat(3776); // Example value in mV (3776 = 64 * 59)
    test_binrw_roundtrip(original);
}

#[test]
fn test_adcvsys_binrw_roundtrip() {
    let original = AdcVsys(3968); // Example value in mV (3968 = 64 * 62)
    test_binrw_roundtrip(original);
}

#[test]
fn test_adc_option_binrw_roundtrip() {
    let original = AdcOption {
        msb_flags: AdcOptionMsbFlags::from_bits_truncate(0b11110000),
        lsb_flags: AdcOptionFlags::from_bits_truncate(0b00001111),
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_charge_option0_binrw_roundtrip() {
    let original = ChargeOption0 {
        msb_flags: ChargeOption0MsbFlags::from_bits_truncate(0b10101010),
        lsb_flags: ChargeOption0Flags::from_bits_truncate(0b01010101),
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_charge_option1_binrw_roundtrip() {
    let original = ChargeOption1 {
        msb_flags: ChargeOption1MsbFlags::from_bits_truncate(0b11001100),
        lsb_flags: ChargeOption1Flags::from_bits_truncate(0b00110011),
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_charge_option2_binrw_roundtrip() {
    let original = ChargeOption2 {
        msb_flags: ChargeOption2MsbFlags::from_bits_truncate(0b11110000),
        lsb_flags: ChargeOption2Flags::from_bits_truncate(0b00001111),
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_charge_option3_binrw_roundtrip() {
    let original = ChargeOption3 {
        msb_flags: ChargeOption3MsbFlags::from_bits_truncate(0b01010101),
        lsb_flags: ChargeOption3Flags::from_bits_truncate(0b10101010),
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_prochot_option0_binrw_roundtrip() {
    let original = ProchotOption0 {
        msb_flags: ProchotOption0MsbFlags::from_bits_truncate(0b00110011),
        lsb_flags: ProchotOption0Flags::from_bits_truncate(0b11001100),
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_prochot_option1_binrw_roundtrip() {
    let original = ProchotOption1 {
        msb_flags: ProchotOption1MsbFlags::from_bits_truncate(0b00001111),
        lsb_flags: ProchotOption1Flags::from_bits_truncate(0b11110000),
    };
    test_binrw_roundtrip(original);
}

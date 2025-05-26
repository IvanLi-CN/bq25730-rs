#[cfg(feature = "defmt")]
extern crate defmt; // Make defmt available for derive macros

#[cfg(feature = "defmt")]
use defmt::Format;

/// Represents potential errors when interacting with the chip.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(Format))] // Conditionally derive Format
pub enum Error<E> {
    /// An error occurred during I2C communication.
    I2c(E),
    /// Invalid data received from the chip.
    InvalidData,
    // Add other specific error types as needed later, e.g.:
    // UnsupportedFeature,
    /// CRC validation failed.
    CrcError,
}

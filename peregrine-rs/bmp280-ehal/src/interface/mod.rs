pub mod i2c;
pub mod spi;

/// A method of communicating with the sensor
pub trait SensorInterface {
    /// Interface error type
    type SensorError;

    /// Writes the bytes and populates the buffer from rec data
    /// Returns Result
    fn write_read(&mut self, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::SensorError>;

    fn requires_soft_reset(&self) -> bool;
}

use super::SensorInterface;

pub struct I2cInterface<I2C> {
    /// i2c port
    i2c_port: I2C,

    /// address for i2c communications with the sensor hub
    address: u8,
}

impl<I2C, CommE> I2cInterface<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = CommE>,
    CommE: core::fmt::Debug,
{
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self {
            i2c_port: i2c,
            address: addr,
        }
    }
}

impl<I2C, CommE> SensorInterface for I2cInterface<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = CommE>,
    CommE: core::fmt::Debug,
{
    type SensorError = crate::Error<CommE>;

    fn write_read(&mut self, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::SensorError> {
        let _ = self
            .i2c_port
            .write_read(self.address, bytes, buffer)
            .map_err(crate::Error::Comm)?;
        Ok(())
    }

    /// Does this interface require a soft reset after init?
    fn requires_soft_reset(&self) -> bool {
        todo!()
    }
}

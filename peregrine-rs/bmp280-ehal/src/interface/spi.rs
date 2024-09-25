use embedded_hal::spi::Operation;

use super::SensorInterface;
use crate::Error;

pub struct SpiInterface<SPI> {
    pub spi: SPI,
}

impl<SPI, CommE> SpiInterface<SPI>
where
    SPI: embedded_hal::spi::SpiDevice<Error = CommE>,
    CommE: core::fmt::Debug,
{
    pub fn new(spi_device: SPI) -> Self {
        Self { spi: spi_device }
    }
}

impl<SPI, CommE> SensorInterface for SpiInterface<SPI>
where
    SPI: embedded_hal::spi::SpiDevice<Error = CommE>,
    CommE: core::fmt::Debug,
{
    type SensorError = Error<CommE>;

    fn write_read(&mut self, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::SensorError> {
        let _ = self
            .spi
            .transaction(&mut [Operation::Write(bytes), Operation::Read(buffer)])
            .map_err(crate::Error::Comm)?;
        Ok(())
    }

    /// Does this interface require a soft reset after init?
    fn requires_soft_reset(&self) -> bool {
        todo!()
    }
}

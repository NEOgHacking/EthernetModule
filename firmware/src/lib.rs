#![no_std]

use xpanse_api::{
    bus::{allocator::BusAllocator, spi::SpiBusHandle},
    driver::{Driver, DriverError, DriverMeta},
    gpio_bank::{BankPins, GpioBank},
    metadata::{ModuleDetectResistor, ModuleID, ModuleSlot},
    reexports::embassy_rp::{
        gpio::{Level, Output},
        spi,
    },
    registry::Registry,
};

pub const SPI_SCK_GPIO: u8 = 2;
pub const SPI_MISO_GPIO: u8 = 3;
pub const SPI_MOSI_GPIO: u8 = 4;
pub const W5500_CS_GPIO: u8 = 0;

const COMMON_REG_BLOCK: u8 = 0;
const VERSION_REGISTER: u16 = 0x0039;
const EXPECTED_VERSION: u8 = 0x04;

pub struct EthernetDriver;

impl DriverMeta for EthernetDriver {
    const ID: ModuleID = ModuleID {
        md0: ModuleDetectResistor::R1K1,
        md1: ModuleDetectResistor::R1K5,
    };
}

pub struct W5500 {
    spi: SpiBusHandle,
    cs: Output<'static>,
}

impl W5500 {
    fn new(spi: SpiBusHandle, cs: Output<'static>) -> Self {
        Self { spi, cs }
    }

    pub async fn read_register(&mut self, block: u8, address: u16) -> Result<u8, W5500Error> {
        let mut value = [0];
        self.read(block, address, &mut value).await?;
        Ok(value[0])
    }

    pub async fn write_register(
        &mut self,
        block: u8,
        address: u16,
        value: u8,
    ) -> Result<(), W5500Error> {
        self.write(block, address, &[value]).await
    }

    pub async fn version(&mut self) -> Result<u8, W5500Error> {
        self.read_register(COMMON_REG_BLOCK, VERSION_REGISTER).await
    }

    pub async fn is_present(&mut self) -> Result<bool, W5500Error> {
        Ok(self.version().await? == EXPECTED_VERSION)
    }

    pub async fn read(
        &mut self,
        block: u8,
        address: u16,
        data: &mut [u8],
    ) -> Result<(), W5500Error> {
        let header = make_header(block, address, false)?;
        self.cs.set_low();
        let result = async {
            self.spi.write(&header).await.map_err(W5500Error::Spi)?;
            self.spi.read(data).await.map_err(W5500Error::Spi)
        }
        .await;
        self.cs.set_high();
        result
    }

    pub async fn write(&mut self, block: u8, address: u16, data: &[u8]) -> Result<(), W5500Error> {
        let header = make_header(block, address, true)?;
        self.cs.set_low();
        let result = async {
            self.spi.write(&header).await.map_err(W5500Error::Spi)?;
            self.spi.write(data).await.map_err(W5500Error::Spi)
        }
        .await;
        self.cs.set_high();
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum W5500Error {
    Spi(xpanse_api::bus::spi::SpiError),
    InvalidBlock(u8),
}

fn make_header(block: u8, address: u16, is_write: bool) -> Result<[u8; 3], W5500Error> {
    if block > 0x1f {
        return Err(W5500Error::InvalidBlock(block));
    }

    let control = (block << 3) | if is_write { 0x04 } else { 0 };
    Ok([(address >> 8) as u8, address as u8, control])
}

impl<G: BankPins> Driver<G> for EthernetDriver {
    async fn create(
        gpio_bank: GpioBank<G>,
        slot: ModuleSlot,
        registry: &mut Registry,
        bus_allocator: &mut BusAllocator,
    ) -> Result<(), DriverError> {
        let cs = Output::new(gpio_bank.gpio0.into(), Level::High);
        let mut spi_config = spi::Config::default();
        spi_config.frequency = 100_000;

        let spi = bus_allocator
            .create_spi_bitbang(
                gpio_bank.gpio2.into(),
                gpio_bank.gpio4.into(),
                gpio_bank.gpio3.into(),
                spi_config,
            )
            .map_err(|_| DriverError::InitFailed)?;

        let mut w5500 = W5500::new(spi, cs);
        if !w5500
            .is_present()
            .await
            .map_err(|_| DriverError::InitFailed)?
        {
            return Err(DriverError::InitFailed);
        }

        registry.register(slot, Self::ID, w5500);
        Ok(())
    }
}

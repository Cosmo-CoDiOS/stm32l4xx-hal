use super::*;

macro_rules! bus_enable {
    ($PER:ident => $en:ident) => {
        impl Enable for crate::pac::$PER {
            #[inline(always)]
            fn enable(bus: &mut Self::Bus) {
                bus.enr().modify(|_, w| w.$en().set_bit());
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb(); // TODO: check if needed
            }
            #[inline(always)]
            fn disable(bus: &mut Self::Bus) {
                bus.enr().modify(|_, w| w.$en().clear_bit());
            }
            #[inline(always)]
            fn is_enabled() -> bool {
                Self::Bus::new().enr().read().$en().bit_is_set()
            }
            #[inline(always)]
            fn is_disabled() -> bool {
                Self::Bus::new().enr().read().$en().bit_is_clear()
            }
            #[inline(always)]
            unsafe fn enable_unchecked() {
                Self::enable(&mut Self::Bus::new());
            }
            #[inline(always)]
            unsafe fn disable_unchecked() {
                Self::disable(&mut Self::Bus::new());
            }
        }
    };
}

macro_rules! bus_smenable {
    ($PER:ident => $smen:ident) => {
        impl SMEnable for crate::pac::$PER {
            #[inline(always)]
            fn enable_in_sleep_mode(bus: &mut Self::Bus) {
                bus.smenr().modify(|_, w| w.$smen().set_bit());
                // Stall the pipeline to work around erratum 2.1.13 (DM00037591)
                cortex_m::asm::dsb();
            }
            #[inline(always)]
            fn disable_in_sleep_mode(bus: &mut Self::Bus) {
                bus.smenr().modify(|_, w| w.$smen().clear_bit());
            }
            #[inline(always)]
            fn is_enabled_in_sleep_mode() -> bool {
                Self::Bus::new().smenr().read().$smen().bit_is_set()
            }
            #[inline(always)]
            fn is_disabled_in_sleep_mode() -> bool {
                Self::Bus::new().smenr().read().$smen().bit_is_clear()
            }
            #[inline(always)]
            unsafe fn enable_in_sleep_mode_unchecked() {
                Self::enable(&mut Self::Bus::new());
            }
            #[inline(always)]
            unsafe fn disable_in_sleep_mode_unchecked() {
                Self::disable(&mut Self::Bus::new());
            }
        }
    };
}
macro_rules! bus_reset {
    ($PER:ident => $rst:ident) => {
        impl Reset for crate::pac::$PER {
            #[inline(always)]
            fn reset(bus: &mut Self::Bus) {
                bus.rstr().modify(|_, w| w.$rst().set_bit());
                bus.rstr().modify(|_, w| w.$rst().clear_bit());
            }
            #[inline(always)]
            unsafe fn reset_unchecked() {
                Self::reset(&mut Self::Bus::new());
            }
        }
    };
}

macro_rules! bus {
    ($($PER:ident => ($busX:ty, $($en:ident)?, $($smen:ident)?, $($rst:ident)?),)+) => {
        $(
            impl crate::Sealed for crate::pac::$PER {}
            impl RccBus for crate::pac::$PER {
                type Bus = $busX;
            }
            $(bus_enable!($PER => $en);)?
            $(bus_smenable!($PER => $smen);)?
            $(bus_reset!($PER => $rst);)?
        )+
    };
}

bus! {
    DMA1 => (AHB1, dma1en, dma1smen, dma1rst), // 0
    DMA2 => (AHB1, dma2en, dma2smen, dma2rst), // 1
    FLASH => (AHB1, flashen, flashsmen, flashrst), // 8
    CRC => (AHB1, crcen, crcsmen, crcrst), // 12
    TSC => (AHB1, tscen, tscsmen, tscrst), // 16

    GPIOA => (AHB2, gpioaen, gpioasmen, gpioarst), // 0
    GPIOB => (AHB2, gpioben, gpiobsmen, gpiobrst), // 1
    GPIOC => (AHB2, gpiocen, gpiocsmen, gpiocrst), // 2
    GPIOD => (AHB2, gpioden, gpiodsmen, gpiodrst), // 3
    GPIOE => (AHB2, gpioeen, gpioesmen, gpioerst), // 4
    GPIOH => (AHB2, gpiohen, gpiohsmen, gpiohrst), // 7
    ADC1 => (AHB2, adcen, adcfssmen, adcrst), // 13
    AES => (AHB2, aesen, aessmen, aesrst), // 16
    RNG => (AHB2, rngen, rngsmen, rngrst), // 18

    TIM2 => (APB1R1, tim2en, tim2smen, tim2rst), // 0
    TIM6 => (APB1R1, tim6en, tim6smen, tim6rst), // 4
    TIM7 => (APB1R1, tim7en, tim7smen, tim7rst), // 5
    LCD => (APB1R1, lcden, lcdsmen, lcdrst), // 9
    WWDG => (APB1R1, wwdgen, wwdgsmen,), // 11
    SPI2 => (APB1R1, spi2en, spi2smen, spi2rst), // 14
    SPI3 => (APB1R1, spi3en, sp3smen, spi3rst), // 15 // TODO: fix typo
    USART2 => (APB1R1, usart2en, usart2smen, usart2rst), // 17
    USART3 => (APB1R1, usart3en, usart3smen, usart3rst), // 18
    I2C1 => (APB1R1, i2c1en, i2c1smen, i2c1rst), // 21
    I2C2 => (APB1R1, i2c2en, i2c2smen, i2c2rst), // 22
    I2C3 => (APB1R1, i2c3en, i2c3smen, i2c3rst), // 23
    CAN1 => (APB1R1, can1en, can1smen, can1rst), // 25
    PWR => (APB1R1, pwren, pwrsmen, pwrrst), // 28
    OPAMP => (APB1R1, opampen, opampsmen, opamprst), // 30
    LPTIM1 => (APB1R1, lptim1en, lptim1smen, lptim1rst), // 31

    LPUART1 => (APB1R2, lpuart1en, lpuart1smen, lpuart1rst), // 0
    SWPMI1 => (APB1R2, swpmi1en, swpmi1smen, swpmi1rst), // 2
    LPTIM2 => (APB1R2, lptim2en, lptim2smen, lptim2rst), // 5
    SYSCFG => (APB2, syscfgen, syscfgsmen, syscfgrst), // 0
    FIREWALL => (APB2, firewallen,,), // 7
    TIM1 => (APB2, tim1en, tim1smen, tim1rst), // 11
    SPI1 => (APB2, spi1en, spi1smen, spi1rst), // 12
    USART1 => (APB2, usart1en, usart1smen, usart1rst), // 14
    TIM15 => (APB2, tim15en, tim15smen, tim15rst), // 16
    TIM16 => (APB2, tim16en, tim16smen, tim16rst), // 17
    SAI1 => (APB2, sai1en, sai1smen, sai1rst), // 21
}

#[cfg(any(feature = "stm32l4x5", feature = "stm32l4x6"))]
bus! {
    GPIOF => (AHB2, gpiofen, gpiofsmen, gpiofrst), // 5
    GPIOG => (AHB2, gpiogen, gpiogsmen, gpiogrst), // 6

    FMC => (AHB3, fmcen, fmcsmen, fmcrst), // 0

    TIM3 => (APB1R1, tim3en, tim3smen, tim3rst), // 1
    TIM4 => (APB1R1, tim4en, tim4smen, tim4rst), // 2
    TIM5 => (APB1R1, tim5en, tim5smen, tim5rst), // 3
    UART4 => (APB1R1, uart4en, uart4smen, uart4rst), // 19
    UART5 => (APB1R1, uart5en, uart5smen, uart5rst), // 20

    TIM8 => (APB2, tim8en, tim8smen, tim8rst), // 13
    TIM17 => (APB2, tim17en, tim17smen, tim17rst), // 18
    SAI2 => (APB2, sai2en, sai2smen, sai2rst), // 22
}

#[cfg(any(feature = "stm32l4x1", feature = "stm32l4x2"))]
bus! {
    UART4 => (APB1R1, uart4en, uart4smen, usart4rst), // 19 // TODO: fix typo

    I2C4 => (APB1R2, i2c4en,, i2c4rst), // 1 // TODO: fix absent
}

#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x5"
))]
bus! {
    DAC1 => (APB1R1, dac1en, dac1smen, dac1rst), // 29

    SDMMC => (APB2, sdmmcen, sdmmcsmen, sdmmcrst), // 10
}

#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
bus! {
    ADC2 => (AHB2, adcen, adcfssmen, adcrst), // 13
    QUADSPI => (AHB3, qspien, qspismen, qspirst), // 8
}

#[cfg(any(
    feature = "stm32l4x1",
    feature = "stm32l4x2",
    feature = "stm32l4x3",
    feature = "stm32l4x6"
))]
bus! {
    CRS => (APB1R1, crsen,,), // 24 // TODO: fix absent
}

#[cfg(any(feature = "stm32l4x2", feature = "stm32l4x3"))]
bus! {
    USB => (APB1R1, usbfsen, usbfssmen, usbfsrst), // 26
}
#[cfg(feature = "stm32l4x1")]
bus! {
    TIM3 => (APB1R1, tim3en,,), // 1 // TODO: absent smen, rst
    USB_FS => (APB1R1, usbf, usbfssmen, usbfsrst), // 26 // TODO: fix typo
}

#[cfg(feature = "stm32l4x2")]
bus! {
    TIM3 => (APB1R1, tim3en,, tim3rst), // 1 // TODO: fix absent
}

#[cfg(feature = "stm32l4x5")]
bus! {
    DFSDM => (APB2, dfsdmen, dfsdmsmen, dfsdmrst), // 24
}

#[cfg(feature = "stm32l4x6")]
bus! {
    DMA2D => (AHB1, dma2den, dma2dsmen, dma2drst), // 17

    GPIOI => (AHB2, gpioien, gpioismen, gpioirst), // 8
    OTG_FS_GLOBAL => (AHB2, otgfsen, otgfssmen, otgfsrst), // 12 // TODO: absent in x5
    DCMI => (AHB2, dcmien, dcmismen, dcmirst), // 14
    HASH => (AHB2, hash1en, hash1smen, hash1rst), // 17

    CAN2 => (APB1R1, can2en, can2smen, can2rst), // 26
    DAC => (APB1R1, dac1en, dac1smen, dac1rst), // 29

    I2C4 => (APB1R2, i2c4en, i2c4smen, i2c4rst), // 1

    SDMMC1 => (APB2, sdmmcen, sdmmcsmen, sdmmcrst), // 10
    DFSDM1 => (APB2, dfsdmen, dfsdmsmen, dfsdmrst), // 24
}

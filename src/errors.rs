//! Error definitions

pub mod i2c {
    /// I2C error
    pub trait Error: core::fmt::Debug {
        /// Convert error to a generic I2C error kind
        ///
        /// By using this method I2C errors freely defined by HAL implementations
        /// can be converted to a set of generic I2C errors upon which generic
        /// code can act.
        fn kind(&self) -> ErrorKind;
    }

    /// I2C error kind
    ///
    /// This represents a common set of I2C operation errors. HAL implementations are
    /// free to define more specific or additional error types. However, by providing
    /// a mapping to these common I2C errors, generic code can still react to them.
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[non_exhaustive]
    pub enum ErrorKind {
        /// An unspecific bus error occurred
        Bus,
        /// The arbitration was lost, e.g. electrical problems with the clock signal
        ArbitrationLoss,
        /// A bus operation was not acknowledged, e.g. due to the addressed device not being available on
        /// the bus or the device not being ready to process requests at the moment
        NoAcknowledge,
        /// The peripheral receive buffer was overrun
        Overrun,
        /// The peripheral send buffer ran out of data
        Underrun,
        /// A different error occurred. The original error may contain more information.
        Other,
    }

    impl Error for ErrorKind {
        fn kind(&self) -> ErrorKind {
            *self
        }
    }
}

pub mod spi {
    /// SPI error
    pub trait Error: core::fmt::Debug {
        /// Convert error to a generic SPI error kind
        ///
        /// By using this method SPI errors freely defined by HAL implementations
        /// can be converted to a set of generic SPI errors upon which generic
        /// code can act.
        fn kind(&self) -> ErrorKind;
    }

    /// SPI error kind
    ///
    /// This represents a common set of SPI operation errors. HAL implementations are
    /// free to define more specific or additional error types. However, by providing
    /// a mapping to these common SPI errors, generic code can still react to them.
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[non_exhaustive]
    pub enum ErrorKind {
        /// An unspecific bus error occurred
        Bus,
        /// The peripheral receive buffer was overrun
        Overrun,
        /// Multiple devices on the SPI bus are trying across each other, e.g. in a multi-master setup
        ModeFault,
        /// CRC does not match the received data
        Crc,
        /// Received data does not conform to the peripheral configuration
        FrameFormat,
        /// A different error occurred. The original error may contain more information.
        Other,
    }

    impl Error for ErrorKind {
        fn kind(&self) -> ErrorKind {
            *self
        }
    }
}

pub mod serial {
    /// Serial error
    pub trait Error: core::fmt::Debug {
        /// Convert error to a generic serial error kind
        ///
        /// By using this method serial errors freely defined by HAL implementations
        /// can be converted to a set of generic serial errors upon which generic
        /// code can act.
        fn kind(&self) -> ErrorKind;
    }

    /// Serial error kind
    ///
    /// This represents a common set of serial operation errors. HAL implementations are
    /// free to define more specific or additional error types. However, by providing
    /// a mapping to these common serial errors, generic code can still react to them.
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    #[non_exhaustive]
    pub enum ErrorKind {
        /// The peripheral receive buffer was overrun.
        Overrun,
        /// Received data does not conform to the peripheral configuration.
        /// Can be caused by a misconfigured device on either end of the serial line.
        FrameFormat,
        /// Parity check failed.
        Parity,
        /// Serial line is too noisy to read valid data.
        Noise,
        /// A different error occurred. The original error may contain more information.
        Other,
    }

    impl Error for ErrorKind {
        fn kind(&self) -> ErrorKind {
            *self
        }
    }
}

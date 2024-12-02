
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Uart",
            extends: None,
            description: Some(
                "Universal asynchronous receiver transmitter",
            ),
            items: &[
                BlockItem {
                    name: "sr",
                    description: Some(
                        "Status register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "Data register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "brr",
                    description: Some(
                        "Baud rate register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Brr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "Control register 1",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "Control register 2",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr3",
                    description: Some(
                        "Control register 3",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr3",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Usart",
            extends: Some(
                "UART",
            ),
            description: Some(
                "Universal synchronous asynchronous receiver transmitter",
            ),
            items: &[
                BlockItem {
                    name: "sr",
                    description: Some(
                        "Status register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dr",
                    description: Some(
                        "Data register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "brr",
                    description: Some(
                        "Baud rate register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Brr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr1",
                    description: Some(
                        "Control register 1",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "Control register 2",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2Usart",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr3",
                    description: Some(
                        "Control register 3",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr3Usart",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gtpr",
                    description: Some(
                        "Guard time and prescaler register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gtpr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Brr",
            extends: None,
            description: Some(
                "Baud rate register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "brr",
                    description: Some(
                        "USARTDIV",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some(
                "Control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbk",
                    description: Some(
                        "Send break",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rwu",
                    description: Some(
                        "Receiver wakeup",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rwu",
                    ),
                },
                Field {
                    name: "re",
                    description: Some(
                        "Receiver enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "te",
                    description: Some(
                        "Transmitter enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idleie",
                    description: Some(
                        "IDLE interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxneie",
                    description: Some(
                        "RXNE interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcie",
                    description: Some(
                        "Transmission complete interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txeie",
                    description: Some(
                        "TXE interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "peie",
                    description: Some(
                        "PE interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ps",
                    description: Some(
                        "Parity selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ps",
                    ),
                },
                Field {
                    name: "pce",
                    description: Some(
                        "Parity control enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wake",
                    description: Some(
                        "Receiver wakeup method",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Wake",
                    ),
                },
                Field {
                    name: "m0",
                    description: Some(
                        "Word length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "M0",
                    ),
                },
                Field {
                    name: "ue",
                    description: Some(
                        "USART enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some(
                "Control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "add",
                    description: Some(
                        "Address of the USART node",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lbdl",
                    description: Some(
                        "Line break detection length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lbdl",
                    ),
                },
                Field {
                    name: "lbdie",
                    description: Some(
                        "LIN break detection interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stop",
                    description: Some(
                        "STOP bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Stop",
                    ),
                },
                Field {
                    name: "linen",
                    description: Some(
                        "LIN mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2Usart",
            extends: Some(
                "CR2",
            ),
            description: Some(
                "Control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "add",
                    description: Some(
                        "Address of the USART node",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lbdl",
                    description: Some(
                        "Line break detection length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lbdl",
                    ),
                },
                Field {
                    name: "lbdie",
                    description: Some(
                        "LIN break detection interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lbcl",
                    description: Some(
                        "Last bit clock pulse",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpha",
                    description: Some(
                        "Clock phase",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cpha",
                    ),
                },
                Field {
                    name: "cpol",
                    description: Some(
                        "Clock polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cpol",
                    ),
                },
                Field {
                    name: "clken",
                    description: Some(
                        "Clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stop",
                    description: Some(
                        "STOP bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Stop",
                    ),
                },
                Field {
                    name: "linen",
                    description: Some(
                        "LIN mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr3",
            extends: None,
            description: Some(
                "Control register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eie",
                    description: Some(
                        "Error interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iren",
                    description: Some(
                        "IrDA mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "irlp",
                    description: Some(
                        "IrDA low-power",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Irlp",
                    ),
                },
                Field {
                    name: "hdsel",
                    description: Some(
                        "Half-duplex selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmar",
                    description: Some(
                        "DMA enable receiver",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmat",
                    description: Some(
                        "DMA enable transmitter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr3Usart",
            extends: Some(
                "CR3",
            ),
            description: Some(
                "Control register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eie",
                    description: Some(
                        "Error interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iren",
                    description: Some(
                        "IrDA mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "irlp",
                    description: Some(
                        "IrDA low-power",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Irlp",
                    ),
                },
                Field {
                    name: "hdsel",
                    description: Some(
                        "Half-duplex selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nack",
                    description: Some(
                        "Smartcard NACK enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scen",
                    description: Some(
                        "Smartcard mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmar",
                    description: Some(
                        "DMA enable receiver",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmat",
                    description: Some(
                        "DMA enable transmitter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtse",
                    description: Some(
                        "RTS enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ctse",
                    description: Some(
                        "CTS enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ctsie",
                    description: Some(
                        "CTS interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some(
                "Data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dr",
                    description: Some(
                        "Data value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gtpr",
            extends: None,
            description: Some(
                "Guard time and prescaler register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "psc",
                    description: Some(
                        "Prescaler value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gt",
                    description: Some(
                        "Guard time value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pe",
                    description: Some(
                        "Parity error",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fe",
                    description: Some(
                        "Framing error",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ne",
                    description: Some(
                        "Noise error flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ore",
                    description: Some(
                        "Overrun error",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idle",
                    description: Some(
                        "Idle line detected",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxne",
                    description: Some(
                        "Read data register not empty",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tc",
                    description: Some(
                        "Transmission complete",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txe",
                    description: Some(
                        "Transmit data register empty",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lbd",
                    description: Some(
                        "LIN break detection flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cts",
                    description: Some(
                        "CTS flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Cpha",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "FIRST",
                    description: Some(
                        "The first clock transition is the first data capture edge",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SECOND",
                    description: Some(
                        "The second clock transition is the first data capture edge",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cpol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Steady low value on CK pin outside transmission window",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "Steady high value on CK pin outside transmission window",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Irlp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "Normal mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LOWPOWER",
                    description: Some(
                        "Low-power mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lbdl",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BIT10",
                    description: Some(
                        "10-bit break detection",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BIT11",
                    description: Some(
                        "11-bit break detection",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "M0",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "BIT8",
                    description: Some(
                        "1 start bit, 8 data bits, n stop bits",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "BIT9",
                    description: Some(
                        "1 start bit, 9 data bits, n stop bits",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ps",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "EVEN",
                    description: Some(
                        "Even parity",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ODD",
                    description: Some(
                        "Odd parity",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rwu",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVE",
                    description: Some(
                        "Receiver in active mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MUTE",
                    description: Some(
                        "Receiver in mute mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Stop",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "STOP1",
                    description: Some(
                        "1 stop bit",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "STOP0P5",
                    description: Some(
                        "0.5 stop bits",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "STOP2",
                    description: Some(
                        "2 stop bits",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "STOP1P5",
                    description: Some(
                        "1.5 stop bits",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Wake",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "IDLELINE",
                    description: Some(
                        "USART wakeup on idle line",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ADDRESSMARK",
                    description: Some(
                        "USART wakeup on address mark",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                
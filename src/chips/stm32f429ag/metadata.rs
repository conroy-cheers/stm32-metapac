include!("../metadata_0194.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "STM32F429AG",
                family: "STM32F4",
                line: "STM32F429/439",
                memory: &[
    MemoryRegion {
        name: "BANK_1_REGION_1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 65536,
        settings: Some(
            FlashSettings {
                erase_size: 16384,
                write_size: 4,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "BANK_1_REGION_2",
        kind: MemoryRegionKind::Flash,
        address: 0x8010000,
        size: 65536,
        settings: Some(
            FlashSettings {
                erase_size: 65536,
                write_size: 4,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "BANK_1_REGION_3",
        kind: MemoryRegionKind::Flash,
        address: 0x8020000,
        size: 917504,
        settings: Some(
            FlashSettings {
                erase_size: 131072,
                write_size: 4,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "CCMRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x10000000,
        size: 65536,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 196608,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(4),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };
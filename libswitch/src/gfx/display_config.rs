use core::ptr::write_volatile;

use register::mmio::ReadWrite;

/// Wrapper around a display configuration value.
#[derive(Clone, Copy, Debug)]
struct ConfigTable {
    /// The offset to write the value to.
    offset: u32,
    /// The actual configuration value.
    value: u32,
}

#[derive(Clone, Debug)]
pub struct Config<'a> {
    tables: &'a [ConfigTable],
}

impl Config {
    pub const CLOCK_CONFIG: Self = Config {
        tables: &[
            ConfigTable {
                offset: 0x4E,
                value: 0x40000000,
            },
            ConfigTable {
                offset: 0x34,
                value: 0x4830A001,
            },
            ConfigTable {
                offset: 0x36,
                value: 0x20,
            },
            ConfigTable {
                offset: 0x37,
                value: 0x2D0AAA,
            },
        ],
    };

    pub const DISPLAY_A: Self = Config {
        tables: &[
            Config {
                offset: 0x40,
                value: 0,
            },
            Config {
                offset: 0x41,
                value: 0x100,
            },
            Config {
                offset: 0x41,
                value: 1,
            },
            Config {
                offset: 0x043,
                value: 0x54,
            },
            Config {
                offset: 0x41,
                value: 0x100,
            },
            Config {
                offset: 0x41,
                value: 1,
            },
            Config {
                offset: 0x42,
                value: 0x10,
            },
            Config {
                offset: 0x42,
                value: 0x20,
            },
            Config {
                offset: 0x42,
                value: 0x40,
            },
            Config {
                offset: 0x480,
                value: 0,
            },
            Config {
                offset: 0x403,
                value: 0,
            },
            Config {
                offset: 0x404,
                value: 0,
            },
            Config {
                offset: 0x36,
                value: 0x50155,
            },
            Config {
                offset: 0x01,
                value: 0x100,
            },
            Config {
                offset: 0x28,
                value: 0x109,
            },
            Config {
                offset: 0x41,
                value: 0xF00,
            },
            Config {
                offset: 0x41,
                value: 0xF,
            },
            Config {
                offset: 0x40,
                value: 0,
            },
            Config {
                offset: 0x42,
                value: 0x10,
            },
            Config {
                offset: 0x700,
                value: 0,
            },
            Config {
                offset: 0x42,
                value: 0x10,
            },
            Config {
                offset: 0x70E,
                value: 0,
            },
            Config {
                offset: 0x700,
                value: 0,
            },
            Config {
                offset: 0x42,
                value: 0x10,
            },
            Config {
                offset: 0x42,
                value: 0x10,
            },
            Config {
                offset: 0x611,
                value: 0xF0,
            },
            Config {
                offset: 0x612,
                value: 0x12A,
            },
            Config {
                offset: 0x613,
                value: 0,
            },
            Config {
                offset: 0x614,
                value: 0x198,
            },
            Config {
                offset: 0x615,
                value: 0x39B,
            },
            Config {
                offset: 0x616,
                value: 0x32F,
            },
            Config {
                offset: 0x617,
                value: 0x204,
            },
            Config {
                offset: 0x618,
                value: 0,
            },
            Config {
                offset: 0x42,
                value: 0x10,
            },
            Config {
                offset: 0x700,
                value: 0,
            },
            Config {
                offset: 0x42,
                value: 0x20,
            },
            Config {
                offset: 0x700,
                value: 0,
            },
            Config {
                offset: 0x42,
                value: 0x40,
            },
            Config {
                offset: 0x700,
                value: 0,
            },
            Config {
                offset: 0x430,
                value: 0x8,
            },
            Config {
                offset: 0x42F,
                value: 0,
            },
            Config {
                offset: 0x307,
                value: 0x1000000,
            },
            Config {
                offset: 0x309,
                value: 0,
            },
            Config {
                offset: 0x4E4,
                value: 0,
            },
            Config {
                offset: 0x300,
                value: 0,
            },
            Config {
                offset: 0x41,
                value: 0xF00,
            },
            Config {
                offset: 0x41,
                value: 0xF0,
            },
            Config {
                offset: 0x42,
                value: 0x10,
            },
            Config {
                offset: 0x716,
                value: 0x10000FF,
            },
            Config {
                offset: 0x42,
                value: 0x20,
            },
            Config {
                offset: 0x716,
                value: 0x10000FF,
            },
            Config {
                offset: 0x42,
                value: 0x40,
            },
            Config {
                offset: 0x716,
                value: 0x10000FF,
            },
            Config {
                offset: 0x031,
                value: 0,
            },
            Config {
                offset: 0x42,
                value: 0x10,
            },
            Config {
                offset: 0x700,
                value: 0,
            },
            Config {
                offset: 0x42,
                value: 0x20,
            },
            Config {
                offset: 0x700,
                value: 0,
            },
            Config {
                offset: 0x42,
                value: 0x40,
            },
            Config {
                offset: 0x700,
                value: 0,
            },
            Config {
                offset: 0x402,
                value: 0,
            },
            Config {
                offset: 0x32,
                value: 0,
            },
            Config {
                offset: 0x41,
                value: 0xF00,
            },
            Config {
                offset: 0x41,
                value: 0xF,
            },
        ],
    };

    pub const ONE_COLOR: Self = Config {
        tables: &[
            ConfigTable {
                offset: 0x42,
                value: 0x10,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x20,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x40,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x402,
                value: 0x2000_0000,
            },
            ConfigTable {
                offset: 0x32,
                value: 0x20,
            },
        ],
    };

    pub const FRAMEBUFFER: Self = Config {
        tables: &[
            ConfigTable {
                offset: 0x42,
                value: 0x40,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x20,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x42,
                value: 0x10,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x402,
                value: 0x2000_0000,
            },
            ConfigTable {
                offset: 0x703,
                value: 0xC,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x704,
                value: 0,
            },
            ConfigTable {
                offset: 0x707,
                value: 0,
            },
            ConfigTable {
                offset: 0x708,
                value: 0,
            },
            ConfigTable {
                offset: 0x706,
                value: 0x5000_B40,
            },
            ConfigTable {
                offset: 0x709,
                value: 0x1000_1000,
            },
            ConfigTable {
                offset: 0x705,
                value: 0x5000_2D0,
            },
            ConfigTable {
                offset: 0x70A,
                value: 0x5A00_B40,
            },
            ConfigTable {
                offset: 0x702,
                value: 0,
            },
            ConfigTable {
                offset: 0x80B,
                value: 0,
            },
            ConfigTable {
                offset: 0x800,
                value: 0xC000_0000,
            },
            ConfigTable {
                offset: 0x806,
                value: 0,
            },
            ConfigTable {
                offset: 0x808,
                value: 0,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x402,
                value: 0x2000_0000,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x402,
                value: 0x2000_0000,
            },
            ConfigTable {
                offset: 0x700,
                value: 0,
            },
            ConfigTable {
                offset: 0x402,
                value: 0x2000_0000,
            },
            ConfigTable {
                offset: 0x700,
                value: 0x4000_0000,
            },
            ConfigTable {
                offset: 0x32,
                value: 0x20,
            },
            ConfigTable {
                offset: 0x41,
                value: 0x300,
            },
            ConfigTable {
                offset: 0x41,
                value: 0x3,
            },
        ],
    };
}

impl Config {
    pub fn execute(&self, base: *const u32) {
        for table in self.tables {
            unsafe {
                write_volatile(base.offset(table.offset as isize), table.value);
            }
        }
    }
}

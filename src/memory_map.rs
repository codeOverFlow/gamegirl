use std::ops::{Range, RangeInclusive};

const ROM_BANK_RANGE: Range<u16> = 0x0000..0x4000;
const SWITCHABLE_ROM_BANK_RANGE: Range<u16> = 0x4000..0x8000;
const VIDEO_RAM_RANGE: Range<u16> = 0x8000..0xA000;
const SWITCHABLE_RAM_BANK_RANGE: Range<u16> = 0xA000..0xC000;
const K8_INTERNAL_RAM_RANGE: Range<u16> = 0xC000..0xE000;
const ECHO_INTERNAL_RAM_RANGE: Range<u16> = 0xE000..0xFE00;
const SPRITE_ATTRIB_RANGE: Range<u16> = 0xFE00..0xFEA0;
const EMPTY_RANGE: Range<u16> = 0xFEA0..0xFF00;
const IO_PORT_RANGE: Range<u16> = 0xFF00..0xFF4C;
const EMPTY2_RANGE: Range<u16> = 0xFF4C..0xFF80;
const INTERNAL_RAM_RANGE: Range<u16> = 0xFF80..0xFFFF;
const INTERUPT_ENABLE_REGISTER_INDEX: u16 = 0xFFFF;

const RESTART_00_INDEX: u16 = 0x0000;
const RESTART_08_INDEX: u16 = 0x0008;
const RESTART_10_INDEX: u16 = 0x0010;
const RESTART_18_INDEX: u16 = 0x0018;
const RESTART_20_INDEX: u16 = 0x0020;
const RESTART_28_INDEX: u16 = 0x0028;
const RESTART_30_INDEX: u16 = 0x0030;
const RESTART_38_INDEX: u16 = 0x0038;
const VERTICAL_BLANK_INTERUPT_START_INDEX: u16 = 0x0040;
const LCDC_STATUS_INTERUPT_START_INDEX: u16 = 0x0048;
const TIMER_OVERFLOW_INTERUPT_START_INDEX: u16 = 0x0050;
const SERIAL_TRANSFER_COMPLETION_INTERUPT_START_INDEX: u16 = 0x0058;
const HIGH_TO_LOW_INTERUPT_START_INDEX: u16 = 0x0060;

const EXECUTION_START_INDEX: RangeInclusive<u16> = 0x0100..=0x0103;
const NINTENDO_SCROLL_INDEX: RangeInclusive<u16> = 0x0104..=0x0133;
const GAME_TITLE_INDEX: RangeInclusive<u16> = 0x0134..=0x0142;
const IS_CGB_INDEX: u16 = 0x0143;
const HIGH_NIB_LICENCE_INDEX: u16 = 0x0144;
const LOW_NIB_LICENCE_INDEX: u16 = 0x0145;
const IS_SGB_INDEX: u16 = 0x0146;
const CARTRIDGE_TYPE_INDEX: u16 = 0x0147;
const ROM_SIZE_INDEX: u16 = 0x0148;
const RAM_SIZE_INDEX: u16 = 0x0149;
const DESTINATION_CODE_INDEX: u16 = 0x014a;
const LICENCE_CODE_INDEX: u16 = 0x014b;
const MASK_ROM_VERSION_INDEX: u16 = 0x014c;
const COMPLEMENT_CHECK_INDEX: u16 = 0x014d;
const CHECKSUM_INDEX: RangeInclusive<u16> = 0x014e..=0x014f;

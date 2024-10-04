pub type Byte = u8;
pub type Bit = u8;

pub const BIT_1: Bit = 0b_0000_0001;
// pub const BIT_2: Bit = 0b_0000_0010;
// pub const BIT_3: Bit = 0b_0000_0100;
// pub const BIT_4: Bit = 0b_0000_1000;
// pub const BIT_5: Bit = 0b_0001_0000;
// pub const BIT_6: Bit = 0b_0010_0000;
// pub const BIT_7: Bit = 0b_0100_0000;
// pub const BIT_8: Bit = 0b_1000_0000;

pub trait BitArray {
    fn bit(&self) -> Bit;

    fn has(&self, flags: Byte) -> bool {
        (self.bit() & flags) != 0
    }
}

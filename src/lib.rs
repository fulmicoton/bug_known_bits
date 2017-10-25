
#[repr(u32)]
pub enum BugEnum {
    One = 1,
    Two = 2,
}

#[inline(always)]
pub fn from_u32(v: u32) -> BugEnum {
    match v {
        1u32 => BugEnum::One,
        2u32 => BugEnum::Two,
        _ => { panic!("") }
    }
}

pub fn f(input: BugEnum) -> u32 {
    let neg_input = !(input as u32);
    let nval = from_u32(neg_input);
    (nval as u32) | 1u32
}


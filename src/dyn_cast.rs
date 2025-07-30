use num_enum::{IntoPrimitive, TryFromPrimitive};
use rand::Rng;
use std::os::raw::c_void;

#[repr(i32)]
#[derive(Debug, Clone, Copy, Default, IntoPrimitive, TryFromPrimitive)]
pub enum E1 {
    #[default]
    A = 0,
    B = 2,
    C = 3,
}
impl E1 {
    fn random() -> Self {
        let variants = [Self::A, Self::B, Self::C];
        let mut rng = rand::rng();
        variants[rng.random_range(0..variants.len())]
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Default, IntoPrimitive, TryFromPrimitive)]
pub enum E2 {
    #[default]
    A = 0,
    B = 2,
    C = 3,
}

impl E2 {
    fn random() -> Self {
        let variants = [Self::A, Self::B, Self::C];
        let mut rng = rand::rng();
        variants[rng.random_range(0..variants.len())]
    }
}

pub enum DataType {
    I32,
    U32,
}

pub enum PropId {
    PropE1,
    PropE2,
}

// ============ Fonction du'une sdk en C opaque ============

pub fn get_type(t: &PropId) -> DataType {
    match t {
        PropId::PropE1 => DataType::I32,
        PropId::PropE2 => DataType::U32,
    }
}

pub fn get_data(t: &PropId) -> *const c_void {
    match t {
        PropId::PropE1 => {
            let a = E1::random();
            let b: i32 = a.into();
            b as *const c_void
        }
        PropId::PropE2 => {
            let a = E2::random();
            let b: u32 = a.into();
            b as *const c_void
        }
    }
}

// ===================================================

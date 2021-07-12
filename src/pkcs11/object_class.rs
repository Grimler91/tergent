use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[repr(u64)]
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ObjectClass {
    Data = 0,
    Certificate = 1,
    PublicKey = 2,
    PrivateKey = 3,
    SecretKey = 4,
    HwFeature = 5,
    DomainParameters = 6,
    Mechanism = 7,
    OtpKey = 8,
    VendorDefined = 0x80000000,
}

impl TryFrom<c_ulong> for ObjectClass {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        ObjectClass::from_u64(value.into()).ok_or(())
    }
}

impl TryFrom<ObjectClass> for u64 {
    type Error = ();
    fn try_from(value: ObjectClass) -> Result<Self, Self::Error> {
        ObjectClass::to_u64(&value).ok_or(())
    }
}

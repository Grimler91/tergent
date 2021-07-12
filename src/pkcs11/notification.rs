use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Notification {
    Surrender = 0,
    OtpChanged = 1,
}

impl TryFrom<c_ulong> for Notification {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        Notification::from_u64(value.into()).ok_or(())
    }
}

impl TryFrom<Notification> for u64 {
    type Error = ();
    fn try_from(value: Notification) -> Result<Self, Self::Error> {
        Notification::to_u64(&value).ok_or(())
    }
}

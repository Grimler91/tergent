use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CertificateType {
    X509 = 0,
    X509AttrCert = 1,
    Wtls = 2,
    VendorDefined = 0x80000000,
}

impl TryFrom<c_ulong> for CertificateType {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        CertificateType::from_u64(value.into()).ok_or(())
    }
}

impl TryFrom<CertificateType> for u64 {
    type Error = ();
    fn try_from(value: CertificateType) -> Result<Self, Self::Error> {
        CertificateType::to_u64(&value).ok_or(())
    }
}

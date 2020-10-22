pub mod error;

use std::convert::TryFrom;

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
#[repr(u16)]
pub enum Status {
    Ok = 200,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
}

impl TryFrom<u16> for Status {
    type Error = u16;

    fn try_from(code: u16) -> Result<Self, u16> {
        match code {
            200 => Ok(Self::Ok),
            401 => Ok(Self::Unauthorized),
            403 => Ok(Self::Forbidden),
            404 => Ok(Self::NotFound),
            _ => Err(code),
        }
    }
}

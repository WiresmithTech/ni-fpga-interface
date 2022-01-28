//! Error support for the FPGA Interface.

pub enum FPGAError {
    InternalError(i32)
}

pub type Result<T> = core::result::Result<T, FPGAError>;

impl FPGAError {
    pub fn from_code(fpga_return_code: i32) -> Self {
        FPGAError::InternalError(fpga_return_code)
    }
}

pub fn to_fpga_result<T>(value: T, return_code: i32) -> Result<T> {
    if return_code == 0 {
        Ok(value)
    }
    else {
        Err(FPGAError::from_code(return_code))
    }
}

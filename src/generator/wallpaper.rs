#[cfg(windows)]
use std::ffi::CString;
use winapi::ctypes::c_void;
use std::io;
use winapi::um::winuser::{SystemParametersInfoA, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER};
use crate::generator::Result;

pub(crate) fn set_wallpaper(path: impl Into<Vec<u8>>) -> Result<()> {
    let image_path = CString::new(path)?;

    unsafe {
        let success = SystemParametersInfoA(
            SPI_SETDESKWALLPAPER,
            0,
            image_path.as_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE
        ) == 1;

        if success {
            Ok(())
        } else {
            Err(io::Error::last_os_error().into())
        }
    }

}
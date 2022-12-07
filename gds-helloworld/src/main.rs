use cufile_sys as cufile;
use cust::prelude::*;
use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::mem;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::IntoRawFd;

fn main() -> Result<(), Box<dyn Error>> {
    let _ctx = cust::quick_init()?;

    let testfn = env::var("TESTFILE")?;

    println!("Opening File {}", testfn);
    let f = OpenOptions::new()
        .write(true)
        .create(true)
        .custom_flags(libc::O_DIRECT)
        .open(&testfn)?;

    let raw_fd = f.into_raw_fd();

    println!("Opening cuFileDriver.");
    unsafe {
        let status = cufile::cuFileDriverOpen();
        assert_eq!(status.err, cufile::CUfileOpError::CU_FILE_SUCCESS);
    }

    let mut cf_handle: cufile::CUfileHandle_t = ::std::ptr::null_mut();

    let mut descr = cufile::CUfileDescr_t::default();
    descr.handle.fd = raw_fd;
    descr.type_ = cufile::CUfileFileHandleType::CU_FILE_HANDLE_TYPE_OPAQUE_FD;

    println!("Registering cuFile handle to {}.", testfn);
    unsafe {
        let status = cufile::cuFileHandleRegister(&mut cf_handle, &mut descr);
        assert_eq!(status.err, cufile::CUfileOpError::CU_FILE_SUCCESS);
    }

    let io_size = 1 << 24;
    let buff_size = io_size + 0x1000;

    println!("Allocating CUDA buffer of {} bytes.", buff_size);
    let mut buf = unsafe { DeviceBuffer::<u8>::uninitialized(buff_size)? };

    println!("Registering Buffer of {} bytes.", buff_size);
    unsafe {
        let status = cufile::cuFileBufRegister(
            buf.as_device_ptr().as_raw() as *mut ::std::ffi::c_void,
            mem::size_of::<u8>() * buf.len(),
            0,
        );
        assert_eq!(status.err, cufile::CUfileOpError::CU_FILE_SUCCESS);
    }

    println!("Filling memory.");
    buf.set_8(0xab)?;

    let dev_ptr_offset = 0x1000;
    let file_offset = 0x2000;

    println!("Writing buffer to file.");
    unsafe {
        let ret = cufile::cuFileWrite(
            cf_handle,
            buf.as_device_ptr().as_raw() as *mut ::std::ffi::c_void,
            io_size,
            file_offset,
            dev_ptr_offset,
        );
        assert!(ret >= 0 && ret as usize == io_size);
    }

    println!("Releasing cuFile buffer.");
    unsafe {
        let status =
            cufile::cuFileBufDeregister(buf.as_device_ptr().as_raw() as *mut ::std::ffi::c_void);
        assert_eq!(status.err, cufile::CUfileOpError::CU_FILE_SUCCESS);
    }

    println!("Releasing file handle.");
    unsafe {
        cufile::cuFileHandleDeregister(cf_handle);
    }

    println!("Closing File Driver.");
    unsafe {
        let status = cufile::cuFileDriverClose();
        assert_eq!(status.err, cufile::CUfileOpError::CU_FILE_SUCCESS);
    }

    Ok(())
}

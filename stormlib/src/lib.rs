#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

mod bindings;

use std::ffi::CString;
use std::ptr;
use std::os::raw::c_uint;
use std::os::raw::*;
use std::fmt::{Debug, Formatter};
use std::iter::FromIterator;

pub struct MpqArchive {
    handle: bindings::HANDLE,
}

pub struct MpqFile {
    handle: bindings::HANDLE,
}

#[derive(Debug)]
pub enum MpqErr {
    // errors from stormlib
    FileNotFound,
    AccessDenied,
    InvalidHandle,
    NotEnoughMemory,
    NotSupported,
    InvalidParameter,
    NegativeSeek,
    DiskFull,
    AlreadyExists,
    InsufficientBuffer,
    BadFormat,
    NoMoreFiles,
    HandleEof,
    CanNotComplete,
    FileCorrupt,
    AviFile,
    UnknownFileKey,
    ChecksumError,
    InternalFile,
    BaseFileMissing,
    MarkedForDelete,
    FileIncomplete,
    UnknownFileNames,
    CantFindPatchPrefix,
    FakeMpqHeader,

}

impl MpqArchive {
    pub fn from_path(mpq_path: &str) -> Result<MpqArchive, MpqErr> {
        let c_mpq_path = CString::new(mpq_path).unwrap();
        let mut ptr: bindings::HANDLE = ptr::null_mut();
        let result = unsafe {
            bindings::SFileOpenArchive(
                c_mpq_path.as_ptr(),
                0,
                0,
                &mut ptr,
            )
        };
        let mut last_error: bindings::DWORD = bindings::ERROR_SUCCESS;
        if !result {
            last_error = unsafe { bindings::GetLastError() };
        }
        if last_error != bindings::ERROR_SUCCESS {
            return Err(last_error.into());
        }
        let mpq = MpqArchive { handle: ptr };
        return Ok(mpq);
    }

    pub fn get_file_list(&self) -> Result<Vec<String>, MpqErr> {
        let list_file = self.get_file("(listfile)")?;
        let bytes = list_file.read_as_vec()?;
        let lines = bytes.split(|&byte| byte == 0x0a)
            .filter(|line| line.len() > 0)
            .filter_map(|line| String::from_utf8(line[0..line.len() - 1].to_vec()).ok())
            .collect();
        Ok(lines)
    }

    pub fn get_file(
        &self,
        file_name: &str,
    ) -> Result<MpqFile, MpqErr> {
        let c_file_path = CString::new(file_name).unwrap();
        let mut file_handle: bindings::HANDLE = ptr::null_mut();
        let result = unsafe {
            bindings::SFileOpenFileEx(self.handle, c_file_path.as_ptr(), 0, &mut file_handle)
        };
        let mut last_error: bindings::DWORD = bindings::ERROR_SUCCESS;
        if !result {
            last_error = unsafe { bindings::GetLastError() };
        }
        if last_error != bindings::ERROR_SUCCESS {
            return Err(last_error.into());
        }
        return Ok(MpqFile { handle: file_handle });
    }
}

impl MpqFile {
    pub fn read_as_vec(self) -> Result<Vec<u8>, MpqErr> {
        let mut result = Vec::new();
        let mut buf = [0 as u8; 0x1000];
        let mut read_bytes: bindings::DWORD = 1;
        while read_bytes > 0 {
            let read_result = unsafe {
                bindings::SFileReadFile(
                    self.handle,
                    buf.as_mut_ptr().cast(),
                    0x1000,
                    &mut read_bytes,
                    ptr::null_mut(),
                )
            };
            if !read_result {
                let mut last_error: bindings::DWORD = bindings::ERROR_SUCCESS;
                last_error = unsafe { bindings::GetLastError() };
                if last_error != bindings::ERROR_SUCCESS && last_error != bindings::ERROR_HANDLE_EOF {
                    return Err(last_error.into());
                }
            }
            if read_bytes > 0 {
                result.extend_from_slice(&buf[0..(read_bytes as usize)]);
            }
        }
        return Ok(result);
    }
}

impl Drop for MpqArchive {
    fn drop(&mut self) {
        unsafe {
            bindings::SFileCloseArchive(self.handle);
        }
    }
}

impl Drop for MpqFile {
    fn drop(&mut self) {
        unsafe {
            bindings::SFileCloseFile(self.handle);
        }
    }
}

impl Into<MpqErr> for bindings::DWORD {
    fn into(self) -> MpqErr {
        match self {
            bindings::ERROR_SUCCESS => panic!("stormlib binding error: tried to map bindings::ERROR_SUCCESS"),
            bindings::ERROR_FILE_NOT_FOUND => MpqErr::FileNotFound,
            bindings::ERROR_ACCESS_DENIED => MpqErr::AccessDenied,
            bindings::ERROR_INVALID_HANDLE => MpqErr::InvalidHandle,
            bindings::ERROR_NOT_ENOUGH_MEMORY => MpqErr::NotEnoughMemory,
            bindings::ERROR_NOT_SUPPORTED => MpqErr::NotSupported,
            bindings::ERROR_INVALID_PARAMETER => MpqErr::InvalidParameter,
            bindings::ERROR_NEGATIVE_SEEK => MpqErr::NegativeSeek,
            bindings::ERROR_DISK_FULL => MpqErr::DiskFull,
            bindings::ERROR_ALREADY_EXISTS => MpqErr::AlreadyExists,
            bindings::ERROR_INSUFFICIENT_BUFFER => MpqErr::InsufficientBuffer,
            bindings::ERROR_BAD_FORMAT => MpqErr::BadFormat,
            bindings::ERROR_NO_MORE_FILES => MpqErr::NoMoreFiles,
            bindings::ERROR_HANDLE_EOF => MpqErr::HandleEof,
            bindings::ERROR_CAN_NOT_COMPLETE => MpqErr::CanNotComplete,
            bindings::ERROR_FILE_CORRUPT => MpqErr::FileCorrupt,
            bindings::ERROR_AVI_FILE => MpqErr::AviFile,
            bindings::ERROR_UNKNOWN_FILE_KEY => MpqErr::UnknownFileKey,
            bindings::ERROR_CHECKSUM_ERROR => MpqErr::ChecksumError,
            bindings::ERROR_INTERNAL_FILE => MpqErr::InternalFile,
            bindings::ERROR_BASE_FILE_MISSING => MpqErr::BaseFileMissing,
            bindings::ERROR_MARKED_FOR_DELETE => MpqErr::MarkedForDelete,
            bindings::ERROR_FILE_INCOMPLETE => MpqErr::FileIncomplete,
            bindings::ERROR_UNKNOWN_FILE_NAMES => MpqErr::UnknownFileNames,
            bindings::ERROR_CANT_FIND_PATCH_PREFIX => MpqErr::CantFindPatchPrefix,
            bindings::ERROR_FAKE_MPQ_HEADER => MpqErr::FakeMpqHeader,
            _ => panic!("Received unsupported stormlib error code - {}", self)
        }
    }
}
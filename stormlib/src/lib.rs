#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

mod bindings;

use std::error::Error;
use std::ffi::CString;
use std::fmt::{Debug, Formatter};
use std::iter::FromIterator;
use std::os::raw::c_uint;
use std::os::raw::*;
use std::ptr;

use bindings::SFileAddFileEx;

pub struct MpqArchive {
    handle: bindings::HANDLE,
    open_files: Vec<MpqFile>,
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
        return Self::from_path_internal(mpq_path, 0);
    }

    pub fn new(mpq_path: &str) -> Result<MpqArchive, MpqErr> {
        let c_mpq_path = CString::new(mpq_path).unwrap();
        let mut ptr: bindings::HANDLE = ptr::null_mut();
        unsafe {
            bindings::SFileCreateArchive(
                c_mpq_path.as_ptr(),
                bindings::MPQ_CREATE_LISTFILE,
                bindings::HASH_TABLE_SIZE_MAX - 1,
                &mut ptr,
            )
        };

        return Ok(MpqArchive {
            handle: ptr,
            open_files: vec![],
        });
    }

    pub fn from_path_readonly(mpq_path: &str) -> Result<MpqArchive, MpqErr> {
        return Self::from_path_internal(mpq_path, bindings::MPQ_FLAG_READ_ONLY);
    }

    pub fn get_file_list(&mut self) -> Result<Vec<String>, MpqErr> {
        let list_file = self.get_file("(listfile)")?;
        let bytes = list_file.read_as_vec()?;
        let lines = bytes
            .split(|&byte| byte == 0x0a)
            .filter(|line| line.len() > 0)
            .filter_map(|line| String::from_utf8(line[0..line.len() - 1].to_vec()).ok())
            .collect();
        Ok(lines)
    }

    pub fn get_file(&mut self, file_name: &str) -> Result<&MpqFile, MpqErr> {
        let c_file_path = CString::new(file_name).unwrap();

        // check if file is present
        unsafe {
            let has_file = bindings::SFileHasFile(self.handle, c_file_path.as_ptr());
            if !has_file {
                let err = bindings::GetLastError();
                return Err(err.into());
            }
        }

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
        let file = MpqFile {
            handle: file_handle,
        };
        self.open_files.push(file);

        return Ok(self.open_files.last().unwrap());
    }

    pub fn write_file(&mut self, file_name: &str, bytes: &[u8]) -> Result<(), MpqErr> {
        let c_file_path = CString::new(file_name).unwrap();
        let mut file_handle: bindings::HANDLE = ptr::null_mut();

        println!("bytes.len(): {}", bytes.len());

        let fileCreationResult = unsafe {
            bindings::SFileCreateFile(
                self.handle,
                c_file_path.as_ptr(),
                0,
                bytes.len() as u32,
                0,
                bindings::MPQ_FILE_REPLACEEXISTING,
                &mut file_handle,
            )
        };

        println!("fileCreationResult: {}", fileCreationResult);

        if !fileCreationResult {
            let last_err = unsafe { bindings::GetLastError() };
            return Err(last_err.into());
        }

        let write_result = unsafe {
            bindings::SFileWriteFile(
                file_handle.cast(),
                bytes.as_ptr().cast(),
                bytes.len() as u32,
                bindings::MPQ_COMPRESSION_ZLIB,
            )
        };

        println!("write_result: {}", write_result);

        if !write_result {
            let last_err = unsafe { bindings::GetLastError() };
            return Err(last_err.into());
        }

        return Ok(());
    }

    pub fn add_file(
        &mut self,
        file_path: &std::path::PathBuf,
        mpq_save_as_path: &String,
    ) -> Result<(), MpqErr> {
        let file_path = CString::new(file_path.to_string_lossy().as_bytes())
            .expect("Couldn't create a CString from given file_path");
        let save_at = CString::new(mpq_save_as_path.clone())
            .expect("Couldn't create a CString from given mpq_save_as_path");
        let result = unsafe {
            bindings::SFileAddFileEx(
                self.handle,
                file_path.as_ptr(),
                save_at.as_ptr(),
                bindings::MPQ_FILE_COMPRESS | bindings::MPQ_FILE_REPLACEEXISTING,
                bindings::MPQ_COMPRESSION_HUFFMANN,
                bindings::MPQ_COMPRESSION_HUFFMANN,
            )
        };
        if !result {
            let last_err = unsafe { bindings::GetLastError() };
            return Err(last_err.into());
        }
        Ok(())
    }

    fn from_path_internal(mpq_path: &str, flags: bindings::DWORD) -> Result<MpqArchive, MpqErr> {
        let c_mpq_path = CString::new(mpq_path).unwrap();
        let mut ptr: bindings::HANDLE = ptr::null_mut();
        let result = unsafe { bindings::SFileOpenArchive(c_mpq_path.as_ptr(), 0, flags, &mut ptr) };
        let mut last_error: bindings::DWORD = bindings::ERROR_SUCCESS;
        if !result {
            last_error = unsafe { bindings::GetLastError() };
        }
        if last_error != bindings::ERROR_SUCCESS {
            return Err(last_error.into());
        }
        let mpq = MpqArchive {
            handle: ptr,
            open_files: vec![],
        };
        return Ok(mpq);
    }
}

impl MpqFile {
    pub fn get_full_file_name(&self) -> Result<String, MpqErr> {
        let mut file_name_buf = [0 as c_char; bindings::MAX_PATH];
        let file_name = unsafe {
            let result = bindings::SFileGetFileName(self.handle, file_name_buf.as_mut_ptr().cast());
            if !result {
                let err_code = bindings::GetLastError();
                return Err(err_code.into());
            }
        };
        let file_name_raw = file_name_buf
            .iter()
            .take_while(|&&it| it != 0)
            .map(|&it| it as u8)
            .collect();
        Ok(unsafe { String::from_utf8_unchecked(file_name_raw) })
    }

    pub fn get_file_name(&self) -> Result<String, MpqErr> {
        let full = self.get_full_file_name()?;
        let f_name = full.split("\\").last().unwrap();
        Ok(f_name.to_string())
    }

    pub fn size_in_bytes(&self) -> Result<usize, MpqErr> {
        let mut size: bindings::DWORD = 0;
        let result = unsafe { bindings::SFileGetFileSize(self.handle, &mut size) };
        if result == bindings::SFILE_INVALID_SIZE {
            let err = unsafe { bindings::GetLastError() };
            return Err(err.into());
        }
        return Ok(size as usize);
    }

    pub fn read_as_vec(&self) -> Result<Vec<u8>, MpqErr> {
        let size = self.size_in_bytes()?;
        let mut result = Vec::with_capacity(size as usize);
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
                if last_error != bindings::ERROR_SUCCESS && last_error != bindings::ERROR_HANDLE_EOF
                {
                    return Err(last_error.into());
                }
            }
            if read_bytes > 0 {
                result.extend_from_slice(&buf[0..(read_bytes as usize)]);
            }
        }

        // reset file pointer for future reuse
        unsafe {
            let res = bindings::SFileSetFilePointer(
                self.handle,
                0,
                ptr::null_mut(),
                bindings::FILE_BEGIN,
            );
            if res == bindings::SFILE_INVALID_SIZE {
                let err = bindings::GetLastError();
                return Err(err.into());
            }
        }

        return Ok(result);
    }
}

impl Drop for MpqArchive {
    fn drop(&mut self) {
        self.open_files.clear();
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
            bindings::ERROR_SUCCESS => {
                panic!("stormlib binding error: tried to map bindings::ERROR_SUCCESS")
            }
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
            _ => panic!("Received unsupported stormlib error code - {}", self),
        }
    }
}

impl std::fmt::Display for MpqErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for MpqErr {}

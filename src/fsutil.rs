// Copyright © 2022 The Aldar Authors
//
// Use of this source code is governed by an BSD-style
// license that can be found in the LICENSE file.

use std::fs::DirEntry;
#[cfg(unix)]
use std::os::unix::prelude::{MetadataExt, PermissionsExt};

pub trait AldarExt {
    fn is_hidden(&self) -> bool;
    fn is_executable(&self) -> bool;
    fn is_dir(&self) -> bool;
    fn size(&self) -> u64;
    fn full_rel_path(&self, base: &str) -> String;
}

#[cfg(unix)]
impl AldarExt for DirEntry {
    fn is_hidden(&self) -> bool {
        match self.file_name().to_str() {
            Some(n) => n.starts_with("."),
            _ => false,
        }
    }

    fn is_executable(&self) -> bool {
        match self.file_type() {
            Ok(f) if f.is_dir() => false,
            Ok(_) => {
                if let Ok(meta) = self.path().metadata() {
                    return meta.permissions().mode() & 0o111 != 0;
                }
                false
            }
            _ => false,
        }
    }

    fn size(&self) -> u64 {
        let metadata = match self.metadata() {
            Ok(m) => m,
            Err(_) => return 0,
        };

        return metadata.size();
    }

    fn full_rel_path(&self, base: &str) -> String {
       get_full_rel_path(self, base)
    }

    fn is_dir(&self) -> bool {
        match self.file_type() {
            Ok(f) => f.is_dir(),
            _ => false,
        }
    }
}


fn get_full_rel_path(entry: &DirEntry, base: &str) -> String {
    let fp = match entry.path().canonicalize() {
        Ok(p) => p,
        _ => match entry.file_name().to_str() {
            Some(s) => return s.to_string(),
            _ => return "?".to_owned(),
        },
    };

    let rel_str = match fp.to_str() {
        Some(p) => match p.strip_prefix(base) {
            Some(s) => s.to_string(),
            _ => return "?".to_owned(),
        },
        _ => return "?".to_owned(),
    };

    match rel_str.strip_prefix(std::path::MAIN_SEPARATOR) {
        Some(s) => s.to_string(),
        _ => rel_str,
    }
}


#[cfg(windows)]
use std::os::windows::prelude::*;

#[cfg(windows)]
impl AldarExt for DirEntry {
    fn is_hidden(&self) -> bool {
        if let Ok(m) = self.metadata() {
            let attrs = m.file_attributes();

            
            return attrs&win32::FILE_ATTRIBUTE_HIDDEN == win32::FILE_ATTRIBUTE_HIDDEN;
        }
        false
    }

    fn is_executable(&self) -> bool {    
        let path_result = self.path().canonicalize();

        if path_result.is_err() {
            return false;
        }

        let lp_application_name: Vec<u16> = match path_result.unwrap().to_str() {
            Some(s) => s.encode_utf16().chain(Some(0)).collect(),
            _ => return false
        };
        
        let success: win32::BOOL;
        let mut lp_binary_type: win32::DWORD = 0;
        unsafe {
            success = win32::GetBinaryTypeW(lp_application_name.as_ptr(), &mut lp_binary_type);
        }

        success == win32::TRUE
    }

    fn size(&self) -> u64 {
        let metadata = match self.metadata() {
            Ok(m) => m,
            Err(_) => return 0,
        };

        return metadata.file_size();
    }

    fn full_rel_path(&self, base: &str) -> String {
        get_full_rel_path(self, base)
    }

    fn is_dir(&self) -> bool {
        match self.file_type() {
            Ok(f) => f.is_dir(),
            _ => false,
        }
    }
}


#[cfg(windows)]
#[allow(dead_code)]
mod win32 {

    pub const FILE_ATTRIBUTE_HIDDEN: u32 = 0x00000002;
    pub const TRUE: BOOL = 1;
    pub const FALSE: BOOL = 0;


    #[allow(non_camel_case_types)]
    pub type c_uint = u32;
    #[allow(non_camel_case_types)]
    pub type c_int = i32;
    pub type DWORD = c_uint;
    pub type LPDWORD = *mut DWORD;
    pub type BOOL = c_int;

    pub type LPCWSTR = *const WCHAR;
    pub type WCHAR = wchar_t;
    #[allow(non_camel_case_types)]
    pub type wchar_t = u16;

    #[allow(non_camel_case_types)]
    pub enum LpBinaryType {
        // A 32-bit Windows-based application
        SCS_32BIT_BINARY = 0,

        // 	A 64-bit Windows-based application.
        SCS_64BIT_BINARY = 6,

        // 	An MS-DOS – based application
        SCS_DOS_BINARY = 1,

        // 	A 16-bit OS/2-based application
        SCS_OS216_BINARY = 5,

        // A PIF file that executes an MS-DOS – based application
        SCS_PIF_BINARY = 3,

        // 	A POSIX – based application
        SCS_POSIX_BINARY = 4,

        // A 16-bit Windows-based application
        SCS_WOW_BINARY = 2,
    }

    #[link(name = "Kernel32")]
    extern "system" {
        /// [`GetBinaryTypeW`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getbinarytypew)
        pub fn GetBinaryTypeW(lpApplicationName: LPCWSTR, lpBinaryType: LPDWORD) -> BOOL;
    }
}

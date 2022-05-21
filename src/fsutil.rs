// Copyright © 2022 The Aldar Authors
//
// Use of this source code is governed by an BSD-style
// license that can be found in the LICENSE file.

use std::{fs::DirEntry, os::unix::prelude::{PermissionsExt, MetadataExt}};


pub trait AldarExt {
    fn  is_hidden(&self) -> bool;
    fn  is_executable(&self) -> bool;
    fn  is_dir(&self) -> bool;
    fn  size(&self) -> u64;
    fn  full_rel_path(&self, base: &str) -> String;
}

#[cfg(unix)]
impl AldarExt for DirEntry {
    fn  is_hidden(&self) -> bool {
        match self.file_name().to_str() {
            Some(n) => n.starts_with("."),
            _ => false
        }
    }

    fn  is_executable(&self) -> bool {
        match self.file_type() {
            Ok(f) if f.is_dir() => false,
            Ok(_) => {
                if let Ok(meta) = self.path().metadata() {
                    return meta.permissions().mode()&0o111 != 0                
                }
                false
            },
            _ => false
        }
    }

    fn  size(&self) -> u64 {        
        let metadata = match self.metadata() {
            Ok(m) => m,
            Err(_) => return 0,
            
        };
              
        return metadata.size();        
    }

    fn  full_rel_path(&self, base: &str) -> String {
        let fp = match self.path().canonicalize() {
            Ok(p) => p,
            _ => match self.file_name().to_str() {
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

    fn  is_dir(&self) -> bool {
        match self.file_type() {
            Ok(f) => f.is_dir(),          
            _ => false
        }
    }
}


#[cfg(windows)]
impl AldarExt for DirEntry {
    fn  is_hidden(&self) -> bool {
        todo!()
    }

    fn  is_executable(&self) -> bool {
        todo!()
    }

    fn  size(&self) -> u64 {
        todo!()
    }

    fn  full_rel_path(&self, base: &str) -> String {
        todo!()
    }

    fn  is_dir(&self) -> bool {
        todo!()
    }
}
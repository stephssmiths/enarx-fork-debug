//const VERSIONS: &str = env!("fewaMJEFGWIOJwef");
//const VERSIONS: &str = &*version_file_read(); 
static VERSIONS: &'static str = include_str!(concat!("temp_crate_info.txt"));


use std::fs::File;
use std::io::prelude::*;

use colorful::core::StrMarker;

pub fn crate_versions() -> &'static str {
  VERSIONS
}


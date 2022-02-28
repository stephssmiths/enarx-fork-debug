static VERSIONS: &'static str = include_str!(concat!("temp_crate_info.txt"));

pub fn crate_versions() -> &'static str {
    VERSIONS
}

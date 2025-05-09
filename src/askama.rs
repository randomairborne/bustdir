use crate::BustDir;

/// This filter takes one argument of type `BustDir`.
pub fn bust_dir<T: std::fmt::Display>(
    path: T,
    _: &dyn askama::Values,
    bustdir: &BustDir,
) -> askama::Result<String> {
    let path = path.to_string();
    Ok(bustdir.get_or_random(&path).to_hex().to_string())
}

use std::path::PathBuf;

/// Mutate a str in-place to have uppercase first letter.
pub fn first_letter_uppercase(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

/// Convert a PathBuf to a string, optionally canonicalizing it along the way.
pub fn pathbuf_to_string(p: &PathBuf, canonicalize: bool) -> String {
    return (|| { (if canonicalize  {p.canonicalize().ok()?} else {p.to_path_buf()}).into_os_string().into_string().ok() }) ().unwrap()
}
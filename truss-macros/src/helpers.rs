pub struct VariantInfo<'a> {
    pub name: &'a syn::Ident,
    pub method: String,
    pub path: String,
    pub is_unit: bool,
}

pub fn capitalize(value: &str) -> String {
    let mut c = value.chars();

    match c.next() {
        Some(ch) => ch.to_ascii_uppercase().to_string() + c.as_str(),
        None => String::new(),
    }
}

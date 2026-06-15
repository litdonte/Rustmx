pub trait IntoQueryParam {
    fn key(&self) -> &str;
    fn value(&self) -> &str;
}

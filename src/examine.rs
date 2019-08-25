pub trait Examine {
    fn examine(&self) -> &'static str;
}

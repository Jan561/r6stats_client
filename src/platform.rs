#[derive(Clone, Copy, Debug)]
pub enum Platform {
    Pc,
    Xbox,
    Playstation,
}

impl From<Platform> for &str {
    fn from(val: Platform) -> &'static str {
        match val {
            Platform::Pc => "pc",
            Platform::Xbox => "xbox",
            Platform::Playstation => "ps4",
        }
    }
}

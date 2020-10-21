#[derive(Clone, Copy, Debug)]
pub enum Platform {
    Pc,
    Xbox,
    Playstation,
}

impl Platform {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pc => "pc",
            Self::Xbox => "xbox",
            Self::Playstation => "ps4",
        }
    }
}

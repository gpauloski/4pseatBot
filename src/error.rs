use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    // Modules
    #[from]
    Config(crate::config::Error),

    // External
    #[from]
    Serenity(serenity::Error),
}

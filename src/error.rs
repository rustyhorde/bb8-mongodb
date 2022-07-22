#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("There was an error connecting to the database!")]
    MongoDB(#[from] mongodb::error::Error),
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid image")]
    InvalidImage,

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

pub fn handle_error(err: AppError) {
    match err {
        AppError::Unexpected(e) => {
            println!("{}", e.to_string())
        }
        expected => println!("{}", expected.to_string()),
    }
}

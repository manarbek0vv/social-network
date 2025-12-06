use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("user service internal error")]
    UserServiceInternal,

    // #[error("Password hashing error")]
    // PasswordHashingError,
}

// pub fn map_users_status_to_auth_error(status: Status) -> AuthError {
//     match status.code() {
//         Code::NotFound => AuthError::InvalidCredentials,
//         Code::AlreadyExists => AuthError::UserAlreadyExists,
//         _ => AuthError::UserServiceInternal,
//     }
// }

// pub fn map_auth_error_to_status(error: AuthError) -> Status {
//     match error {
//         AuthError::InvalidCredentials => Status::unauthenticated("invalid credentials"),
//         AuthError::UserAlreadyExists => Status::already_exists("User with this email exists"),
//         AuthError::UserServiceInternal => Status::internal("Auth service internal error"),
//     }
// }
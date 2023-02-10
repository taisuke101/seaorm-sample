use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use std::{env::VarError, error::Error as StdError, net::AddrParseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error(transparent)]
    Error(#[from] Box<dyn StdError>),
    #[error(transparent)]
    DbError(#[from] DbErr),
    #[error("Error")]
    NoneError(i32),
}

#[derive(Debug, Error)]
pub enum ServerBuildError {
    #[error(transparent)]
    VarError(#[from] VarError),
    #[error(transparent)]
    DbErr(#[from] DbErr),
    #[error(transparent)]
    AddrParseError(#[from] AddrParseError),
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let body = match self {
            CustomError::Error(err) => (StatusCode::INTERNAL_SERVER_ERROR, eprintln!("{:?}", err)),
            CustomError::DbError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, eprintln!("{:?}", err))
            }
            CustomError::NoneError(err) => (
                StatusCode::NOT_FOUND,
                eprintln!("Error: requested for id {:?}, but got None", err),
            ),
        };

        body.into_response()
    }
}

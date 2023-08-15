use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError};
use diesel::result::Error as DieselError;
use std::{
    fmt,
    io::{Error as IOError, ErrorKind},
};

pub type DatabaseError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub enum EntityError {
    NotFound,
    BadData(String),
    Unknown(String),
    Other(String),
}

#[derive(Debug)]
pub enum Error {
    EntityError(EntityError),
    DatabaseTimeout,
}

impl fmt::Display for EntityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "Not found"),
            Self::BadData(e) => write!(f, "BadData {}", e),
            Self::Unknown(e) => write!(f, "Unknown error {}", e),
            Self::Other(e) => write!(f, "Error {}", e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EntityError(e) => write!(f, "{}", e),
            Self::DatabaseTimeout => write!(f, "Database connection timeout"),
        }
    }
}

impl From<DieselError> for EntityError {
    fn from(value: DieselError) -> Self {
        match value {
            DieselError::NotFound => Self::NotFound,
            DieselError::QueryBuilderError(e) => Self::BadData(e.to_string()),
            DieselError::DatabaseError(_, e) => Self::Other(e.message().to_owned()),
            _ => Self::Unknown("Unknown database error".to_owned()),
        }
    }
}

impl From<DieselError> for Error {
    fn from(value: DieselError) -> Self {
        Error::EntityError(EntityError::from(value))
    }
}

impl From<EntityError> for Error {
    fn from(val: EntityError) -> Self {
        Error::EntityError(val)
    }
}

impl<T> From<EntityError> for Result<T, Error> {
    fn from(val: EntityError) -> Self {
        Err(val.into())
    }
}

impl From<EntityError> for IOError {
    fn from(val: EntityError) -> Self {
        match val {
            EntityError::NotFound => IOError::new(ErrorKind::NotFound, val.to_string()),
            EntityError::BadData(e) => IOError::new(ErrorKind::InvalidData, e),
            EntityError::Unknown(e) => IOError::new(ErrorKind::Other, e),
            EntityError::Other(e) => IOError::new(ErrorKind::Other, e),
        }
    }
}

impl From<Error> for IOError {
    fn from(val: Error) -> Self {
        match val {
            Error::EntityError(e) => e.into(),
            Error::DatabaseTimeout => IOError::new(ErrorKind::TimedOut, val.to_string()),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::EntityError(EntityError::NotFound) => StatusCode::NOT_FOUND,
            Self::EntityError(EntityError::BadData(_)) => StatusCode::BAD_REQUEST,
            Self::EntityError(EntityError::Unknown(_)) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EntityError(EntityError::Other(_)) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DatabaseTimeout => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(self.to_string())
    }
}

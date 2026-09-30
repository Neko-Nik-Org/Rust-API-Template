use deadpool_postgres::PoolError;
use std::fmt;


pub type PgResult<T> = Result<T, PgError>;
pub type PgPool = deadpool_postgres::Pool;  // Just an alias for convenience


#[derive(Debug)]
pub enum PgError {
    Pool(PoolError),
    Postgres(tokio_postgres::Error),
}


// ------- Implementations ------- //


#[inline]
fn pg_error(db_error: &tokio_postgres::Error, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(db_err) = db_error.as_db_error() {
        write!(
            f,
            "PostgreSQL error: {} | code={} | severity={} | detail={} | hint={} | table={} | column={} | constraint={}",
            db_err.message(),
            db_err.code().code(),
            db_err.severity(),
            db_err.detail().unwrap_or("-"),
            db_err.hint().unwrap_or("-"),
            db_err.table().unwrap_or("-"),
            db_err.column().unwrap_or("-"),
            db_err.constraint().unwrap_or("-"),
        )
    } else {
        write!(f, "PostgreSQL client error: {}", db_error)
    }
}


#[inline]
fn pool_error(db_error: &PoolError, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        f,
        "PostgreSQL pool error: {}",
        db_error
    )
}


impl fmt::Display for PgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PgError::Pool(e) => pool_error(e, f),
            PgError::Postgres(e) => pg_error(e, f),
        }
    }
}


impl From<PoolError> for PgError {
    fn from(e: PoolError) -> Self {
        PgError::Pool(e)
    }
}


impl From<tokio_postgres::Error> for PgError {
    fn from(e: tokio_postgres::Error) -> Self {
        PgError::Postgres(e)
    }
}


impl std::error::Error for PgError {}

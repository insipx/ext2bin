

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Not found {}", _0)]
    NotFound(String),
    #[fail(display = "Tried Casting {}", _0)]
    IncorrectCast(String)
}

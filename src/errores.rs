//INVALID_TABLE: relacionado a problemas con el procesamiento de las tablas.
//INVALID_COLUMN: relacionado a problemas con el procesamiento de columnas.
//INVALID_SYNTAX: relacionado a problemas con el procesamiento de consultas.
//ERROR: tipo genérico para otros posibles errores detectados.
// pub enum SqlErrors {
//     INVALID_TABLE,
//     INVALID_COLUMN,
//     INVALID_SYNTAX,
// }

//pub fn funcion_invalida()

use std::error;
use std::fmt;

/// Define a custom error type for MQTT errors.
#[derive(Debug)]
pub struct SQLError {
    message: String,
}

impl SQLError {
    /// Creates a new SQLError with the given message.
    pub fn new(message: &str) -> SQLError {
        SQLError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for SQLError {
    /// Formats the SQLError for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for SQLError {
    /// Provides a description of the SQLError.
    fn description(&self) -> &str {
        &self.message
    }
}












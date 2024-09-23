use crate::errores::SQLError;
use std::path::Path;


///Recibe el nombre del archivo y el path del archivo.
/// Devuelve el path completo del archivo.
pub fn archivo(nombre_archivo: &String, direccion_archivo: &String) -> Result<String, SQLError> {

    let mut tabla: String = direccion_archivo.to_string();
    tabla.push_str("/");
    tabla.push_str(&nombre_archivo.replace(";", ""));
    tabla.push_str(".csv");
    if !Path::new(&tabla).exists() {
        println!("No existe la tabla: {}", tabla);

        return Err(SQLError::new("INVALID_TABLE"));
    }
    return Ok(tabla)
}

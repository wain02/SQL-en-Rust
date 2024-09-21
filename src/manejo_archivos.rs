use crate::errores::SQLError;
use std::path::Path;



pub fn archivo(nombre_archivo: &String, direccion_archivo: &String) -> Result<String, SQLError> {

    let mut tabla: String = direccion_archivo.to_string();
    tabla.push_str("/");
    tabla.push_str(&nombre_archivo.replace(";", ""));
    tabla.push_str(".csv");
    if !Path::new(&tabla).exists() {
        println!("No existe la tabla");
        return Err(SQLError::new("INVALID_TABLE"));
    }
    return Ok(tabla)
}

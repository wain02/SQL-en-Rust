

pub fn regex_casero(sql: &str, claves: Vec<&str>) -> Vec<String> {
    //let claves = ["DELETE", "FROM", "WHERE", "AND", "OR"];
    let mut partes = Vec::new();
    let mut partes_actuales = String::new();

    for palabra in sql.split_whitespace() {
        if claves.contains(&palabra.to_uppercase().as_str()) {
            if !partes_actuales.is_empty() {
                partes.push(partes_actuales.trim().to_string());
                partes_actuales.clear();
            }
        } else {
            partes_actuales.push_str(palabra);
            partes_actuales.push(' ');
        }
    }

    if !partes_actuales.is_empty() {
        partes.push(partes_actuales.trim().to_string());
    }

    partes
}


/*pub fn parse_operadores(condicion: &str) -> Option<(String, String, String)> {
    let operadores = ['>', '<', '='];

    for operador in &operadores {
        if let Some(pos) = condicion.find(*operador) {
            //divide de un lado la columna y del otro el valor con su operador respectivo.
            let columna_filtro = condicion[..pos].trim().to_string();
            let valor_filtro = condicion[pos + 1..].trim().to_string();
           
            let operador = match operador {
                '>' => "mayor".to_string(),
                '<' => "menor".to_string(),
                '=' => "igual".to_string(),
                _ => return None, // Error
            };
            
            return Some((columna_filtro, operador, valor_filtro));
        }
    }

    // si el operador seleccionado no esta.
    

    None
} */



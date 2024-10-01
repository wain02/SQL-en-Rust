//#![cfg(test)]
//use super::*;
#[test]
fn test_insert_simple() {
    let consulta = "INSERT INTO ordenes (id, id_cliente, producto, cantidad) VALUES (111, 6, 'Laptop', 3);";
    let consulta_ref = consulta.to_string();
    let direccion_archivo = "src/archivosCSV".to_string();
    let result = super::insert::comando_insert(consulta_ref, direccion_archivo);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_insert_complejo() {
    let consulta = "INSERT INTO ordenes (id, id_cliente, producto, cantidad) VALUES (111, 6, 'Laptop', 3),
    (101, 7, 'momito', 3),(13, 6, 'laso', 8),(001, 8, 'juan ', 0),(123, 2, 'Laso', 4);";
    let consulta_ref = consulta.to_string();
    let direccion_archivo = "src/archivosCSV".to_string();
    let result = super::insert::comando_insert(consulta_ref, direccion_archivo);
    assert_eq!(result.is_ok(), true);
}


#[test]
fn test_update_simple() {
    let consulta = "UPDATE clientes SET email = 'mrodriguez@hotmail.com', nombre = 'Sol'  WHERE id = 4;";
    let consulta_ref = consulta.to_string();
    let direccion_archivo = "src/archivosCSV".to_string();
    let result = super::update::comando_update(consulta_ref, direccion_archivo);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_delete_simple() {
    let consulta = "DELETE FROM ordenes WHERE producto = 'Laptop';";
    let consulta_ref = consulta.to_string();
    let direccion_archivo = "src/archivosCSV".to_string();
    let result = super::delete::comando_delete(consulta_ref, direccion_archivo);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_select() {
    let consulta = "SELECT * FROM ordenes;";
    let consulta_ref = consulta.to_string();
    let direccion_archivo = "src/archivosCSV".to_string();
    let result = super::select::comando_select(consulta_ref, direccion_archivo);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_select_simple() {
    let consulta: String = "SELECT id, producto, id_cliente FROM ordenes WHERE cantidad > 1;".to_string();
    let direccion_archivo: String = "src/archivosCSV".to_string();
    let result = super::select::comando_select(consulta, direccion_archivo);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_select_complejo() {
    let consulta: String = "SELECT id, producto, id_cliente FROM ordenes WHERE (id = 106 AND producto = 'Monitor' AND id_cliente = 21) OR id = 109;".to_string();
    let direccion_archivo: String = "src/archivosCSV".to_string();
    let result = super::select::comando_select(consulta, direccion_archivo);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_select_order_by() {
    let consulta = "SELECT * FROM ordenes ORDER BY id_cliente DESC;";
    let consulta_ref = consulta.to_string();
    let direccion_archivo = "src/archivosCSV".to_string();
    let result = super::select::comando_select(consulta_ref, direccion_archivo);
    assert_eq!(result.is_ok(), true);
}
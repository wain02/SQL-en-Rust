mod insert;
mod delete;
mod update;
mod parciar;
mod select;
mod sql_predicate;
mod sql_conditions;
mod errores;
mod main_tests;
mod manejo_archivos;


fn main() {
    let terminal: Vec<String> = std::env::args().collect();

    let consulta_terminal = &terminal[2]; //consulta completa de SQL
    let direccion_archivo = &terminal[1]; //direccion del archivo CSV
    //let consulta = consulta_terminal.replace(",", "").replace("\n", " "); 
    let consulta = consulta_terminal.replace("\n", " "); 


    let mut partes_consulta: Vec<&str> = consulta.trim().split_whitespace().collect(); // Divide la cadena en partes
    let instruccion = partes_consulta[0];
    
    partes_consulta.remove(0);


    match instruccion.to_uppercase().as_str() {
        "INSERT" => {
            //INSERT INTO ordenes (id, id_cliente, producto, cantidad) VALUES (111, 6, 'Laptop', 3);
            let consulta_ref = &consulta;
            let _ = insert::comando_insert(consulta_ref.to_string(), direccion_archivo.to_string()); 

        },
        "UPDATE" => {
            let consulta_ref = &consulta;
            let _ = update::comando_update(consulta_ref.to_string(), direccion_archivo.to_string());
        },
        "DELETE" => {
            //DELETE FROM contact WHERE person_id IN (SELECT id FROM person WHERE  place_of_birth = 'San Francisco');
            //DELETE FROM person WHERE lastname = 'Burton';
            let consulta_ref = &consulta;
            let _ = delete::comando_delete(consulta_ref.to_string(), direccion_archivo.to_string());
        },
        "SELECT" => {
            let consulta_ref = &consulta;
            let _ = select::comando_select(consulta_ref.to_string(), direccion_archivo.to_string());
        },
        _ => {
            println!("Instrucción no válida");
            return;
        }
    }

}



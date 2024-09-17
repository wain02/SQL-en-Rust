mod insert;
mod delete;
mod update;
mod parciar;
mod select;
mod sql_predicate;
mod sql_conditions;
mod errores;


fn main() {
    let terminal: Vec<String> = std::env::args().collect();
    let consulta_terminal = &terminal[2]; //consulta completa de SQL
    //let consulta = consulta_terminal.replace(",", "").replace("\n", " "); 
    let consulta = consulta_terminal.replace("\n", " "); 


    let mut partes_consulta: Vec<&str> = consulta.trim().split_whitespace().collect(); // Divide la cadena en partes
    let instruccion = partes_consulta[0];
    partes_consulta.remove(0);


    match instruccion.to_uppercase().as_str() {
        "INSERT" => {
            //INSERT INTO ordenes (id, id_cliente, producto, cantidad) VALUES (111, 6, 'Laptop', 3);
            let consulta_ref = &consulta;
            let _ = insert::comando_insert(consulta_ref.to_string()); 

        },
        "UPDATE" => {
            let consulta_ref = &consulta;
            let _ = update::comando_update(consulta_ref.to_string());
        },
        "DELETE" => {
            //DELETE FROM contact WHERE person_id IN (SELECT id FROM person WHERE  place_of_birth = 'San Francisco');
            //DELETE FROM person WHERE lastname = 'Burton';
            let consulta_ref = &consulta;
            let _ = delete::comando_delete(consulta_ref.to_string());
        },
        "SELECT" => {
            let consulta_ref = &consulta;
            let _ = select::comando_select(consulta_ref.to_string());
        },
        _ => {
            println!("Instrucción no válida");
            return;
        }
    }

}



mod mytest {
    #[test]
    fn test_insert_simple() {
        let consulta = "INSERT INTO ordenes (id, id_cliente, producto, cantidad) VALUES (111, 6, 'Laptop', 3);";
        let consulta_ref = consulta.to_string();
        let result = super::insert::comando_insert(consulta_ref);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_insert_complejo() {
        let consulta = "INSERT INTO ordenes (id, id_cliente, producto, cantidad) VALUES (111, 6, 'Laptop', 3),
        (101, 7, 'momito', 3),(13, 6, 'laso', 8),(001, 8, 'juan ', 0),(123, 2, 'Laso', 4);";
        let consulta_ref = consulta.to_string();
        let result = super::insert::comando_insert(consulta_ref);
        assert_eq!(result.is_ok(), true);
    }


    #[test]
    fn test_update_simple() {
        let consulta = "UPDATE clientes SET email = 'mrodriguez@hotmail.com', nombre = 'Sol'  WHERE id = 4;";
        let consulta_ref = consulta.to_string();
        let result = super::update::comando_update(consulta_ref);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_delete_simple() {
        let consulta = "DELETE FROM ordenes WHERE producto = 'Laptop';";
        let consulta_ref = consulta.to_string();
        let result = super::delete::comando_delete(consulta_ref);
        assert_eq!(result.is_ok(), true);
    }
    
    #[test]
    fn test_select_simple() {
        let consulta = "SELECT id, producto, id_cliente FROM ordenes WHERE cantidad > 1;";
        let consulta_ref = consulta.to_string();
        let result = super::select::comando_select(consulta_ref);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_select_order_by() {
        let consulta = "SELECT * FROM ordenes ORDER BY id_cliente DEC;";
        let consulta_ref = consulta.to_string();
        let result = super::select::comando_select(consulta_ref);
        assert_eq!(result.is_ok(), true);
    }
}
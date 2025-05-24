use std::io::{self, Write};

// Ejercicio 1: MTF con lista fija de [0,1,2,3,4] y secuencia repetida
fn ejercicio1() {
    let mut list = vec![0, 1, 2, 3, 4];
    let requests = vec![
        0,1,2,3,4,
        0,1,2,3,4,
        0,1,2,3,4,
        0,1,2,3,4,
    ];
    let mut total_cost = 0;
    println!("=========================\n");
    println!("Ejercicio 1: Move-To-Front costos para secuencia fija\n");
    for req in requests {
        println!("Lista antes:    {:?}", list);
        let cost = access_cost(&mut list, req);
        println!("Solicitud: {:>2} | Costo: {:>2} | Lista después: {:?}\n", req, cost, list);
        total_cost += cost;
    }
    println!("Costo total de todas las accesos: {}\n", total_cost);
    println!("=========================\n");

}

fn ejercicio2() {
    println!("Ejercicio 2 aún no implementado.\n");
}
fn ejercicio3() {
    println!("Ejercicio 3 aún no implementado.\n");
}
fn ejercicio4() {
    println!("Ejercicio 4 aún no implementado.\n");
}
fn ejercicio5() {
    println!("Ejercicio 5 aún no implementado.\n");
}
fn ejercicio6() {
    println!("Ejercicio 6 aún no implementado.\n");
}

// Función de costo y MTF reusada
fn access_cost(list: &mut Vec<usize>, request: usize) -> usize {
    if let Some(pos) = list.iter().position(|&x| x == request) {
        let cost = pos + 1;
        list.remove(pos);
        list.insert(0, request);
        cost
    } else {
        panic!("Solicitud {} no encontrada en la lista", request);
    }
}

fn main() {
    loop {
        println!("Seleccione una opción:");
        println!("1) Ejercicio 1");
        println!("2) Ejercicio 2");
        println!("3) Ejercicio 3");
        println!("4) Ejercicio 4");
        println!("5) Ejercicio 5");
        println!("6) Ejercicio 6");
        println!("7) Salir\n");
        print!("Opción: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => ejercicio1(),
            2 => ejercicio2(),
            3 => ejercicio3(),
            4 => ejercicio4(),
            5 => ejercicio5(),
            6 => ejercicio6(),
            7 => {
                println!("Saliendo...");
                break;
            }
            _ => println!("Opción inválida, intente de nuevo.\n"),
        }
    }
}

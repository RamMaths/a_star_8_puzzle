use proyecto_2::Node;
use proyecto_2::imprimir_estado;
use std::collections::BinaryHeap;
use std::io;
use std::rc::Rc;
use std::cmp::Reverse;

fn main() {
    // ------------------------
    let mut buffer: String = String::new();
    let mut priority_queue: BinaryHeap<Reverse<Rc<Node>>> = BinaryHeap::new();
    let mut estado_inicial: Vec<i8> = Vec::with_capacity(9);
    let mut estado_meta: Vec<i8> = Vec::with_capacity(9);
    let mut resultado: Option<Rc<Node>> = None;

    // toma del estado inicial
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.split_whitespace().for_each(|x| { estado_inicial.push(x.parse::<i8>().unwrap())});
    buffer.clear();
    if estado_inicial.len() < 9 {
        panic!("Estado inicial mal formado");
    }
    // toma del estado meta
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.split_whitespace().for_each(|x| { estado_meta.push(x.parse::<i8>().unwrap())});
    buffer.clear();
    if estado_meta.len() < 9 {
        panic!("Estado meta mal formado");
    }

    let vacio = posicion_vacio(&estado_inicial);

    let root = Rc::new(
        Node::new(
            0,
            vacio,
            estado_inicial,
            &estado_meta,
            None
        )
    );
    
    priority_queue.push(Reverse(Rc::clone(&root)));

    while let Some(Reverse(nodo_actual)) = priority_queue.pop() {
        let h = nodo_actual.costo_funcion - nodo_actual.costo_operador;

        // si h es 0 significa que hemos llegado al estado meta
        if h == 0 { 
            resultado = Some(nodo_actual);
            break; 
        }

        println!("Explorando nodo al nivel: {} ----------------->", &nodo_actual.costo_operador + 1);
        imprimir_estado(&nodo_actual.estado_actual, true);
        println!("");

        // posibles movimientos que se derivan del estado acutal
        let posibles_movimientos: Vec<usize> = nodo_actual.posibles_movimientos().unwrap();

        //iteramos sobre cada uno de los posibles movimientos para crear nuevos nodos
        for index in &posibles_movimientos {
            let mut nuevo_estado: Vec<i8> = nuevo_estado(&nodo_actual.estado_actual);

            nuevo_estado.swap(*index, nodo_actual.posicion_vacio);

            priority_queue.push(
                Reverse(
                    Rc::new(
                        Node::new(
                            nodo_actual.costo_operador + 1,
                            *index,
                            nuevo_estado,
                            &estado_meta,
                            Some(Rc::clone(&nodo_actual))
                        )
                    )
                )
            )
        }
    }

    match resultado {
        Some(nodo) => {
            println!("Resultado ---------------------------------->\nFunción heurística final: {}\nPasos hacia la solución: {}\n", &nodo.costo_funcion, &nodo.costo_operador);
            imprimir_estado(&nodo.estado_actual, true);
        },
        None => println!("Hubo un error y no se llegó a la solución")
    }

}

fn nuevo_estado(estado_actual: &Vec<i8>) -> Vec<i8> {
    let mut nuevo_estado: Vec<i8> = Vec::from(vec![0; 9]);

    for i in 0..9 {
        nuevo_estado[i] = estado_actual[i];
    }

    nuevo_estado
}

// funciones
fn posicion_vacio(arr: &Vec<i8>) -> usize {
    for i in 0..arr.len() {
        if arr[i] == -1 {
            return i;
        }
    }

    0
}

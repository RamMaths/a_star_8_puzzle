use prettytable::Table;
use std::rc::Rc;
// Node
#[derive(Debug)]
pub struct Node<'a> {
    pub costo_funcion: u8,
    pub costo_operador: u8,
    pub posicion_vacio: usize,
    pub nodo_padre: Option<Rc<Node<'a>>>,
    pub estado_actual: Vec<i8>,
    pub estado_meta: &'a Vec<i8>
}

impl<'a> Node<'a> {
    pub fn new(costo_operador: u8, posicion_vacio: usize, estado_actual: Vec<i8>, estado_meta: &'a Vec<i8>, nodo_padre: Option<Rc<Node<'a>>>) -> Self {
        let h = Node::h(&estado_actual, estado_meta);
        imprimir_estado(&estado_actual, false);
        println!("\tf' = {}", &costo_operador + &h);
        println!("");

        Node {
            costo_funcion: costo_operador + h,
            costo_operador,
            posicion_vacio,
            estado_actual,
            estado_meta,
            nodo_padre
        }
    }

    fn h(estado_actual: &Vec<i8>, estado_meta: &Vec<i8>) -> u8 {
        let mut count: u8 = 0;

        for i in 0..estado_actual.len() {
            if estado_meta[i] != estado_actual[i] {
                count += 1;
            }
        }

        if count > 0 {
            return count - 1;
        }

        count
    }

    pub fn posibles_movimientos(&self) -> Option<Vec<usize>> {
        match self.posicion_vacio {
            0 => Some(vec![1, 3]),
            1 => Some(vec![0, 2, 4]),
            2 => Some(vec![1, 5]),
            3 => Some(vec![0, 4, 6]),
            4 => Some(vec![1, 3, 5, 7]),
            5 => Some(vec![2, 4, 8]),
            6 => Some(vec![3, 7]),
            7 => Some(vec![4, 6, 8]),
            8 => Some(vec![7, 5]),
            _ => None
        }
    }
}

// Traits para que la cola de prioridad acepté el nuevo tipo "Node"

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.costo_funcion.eq(&other.costo_funcion)
    }

    fn ne(&self, other: &Self) -> bool {
        self.costo_funcion.ne(&other.costo_funcion)
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.costo_funcion.cmp(&other.costo_funcion))
    }
}

impl<'a> Eq for Node<'a> {}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.costo_funcion.cmp(&other.costo_funcion)
    }
}

// funciones
pub fn imprimir_estado(arr: &Vec<i8>, parent: bool) {
    let mut table = Table::new();

    if parent {
        table.add_row(prettytable::row![FbBrbc => arr[0], arr[1], arr[2]]);
        table.add_row(prettytable::row![FbBrbc => arr[3], arr[4], arr[5]]);
        table.add_row(prettytable::row![FbBrbc => arr[6], arr[7], arr[8]]);
    } else {
        table.add_row(prettytable::row![FdBybc => arr[0], arr[1], arr[2]]);
        table.add_row(prettytable::row![FdBybc => arr[3], arr[4], arr[5]]);
        table.add_row(prettytable::row![FdBybc => arr[6], arr[7], arr[8]]);
    }


    table.printstd();
}


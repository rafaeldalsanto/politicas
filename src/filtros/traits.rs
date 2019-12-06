use crate::pedido::Pedido;

pub trait Filtro {
    fn eh_satisfeito_por(&self, indice: usize, pedido: &Pedido) -> bool;
}

use crate::pedido::Pedido;

pub trait Filtro {
    fn eh_satisfeito_por(&self, indice_do_item: u32, pedido: Pedido) -> bool;
}

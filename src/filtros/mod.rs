mod intervalo;
mod produto;
mod valor_do_pedido;

use crate::pedido::Pedido;

pub trait Filtro {
    fn eh_satisfeito_por(&self, indice: usize, pedido: &Pedido) -> bool;
}

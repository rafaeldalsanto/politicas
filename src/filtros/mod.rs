mod efiltro;
mod intervalo;
mod produto;
mod valor_do_pedido;

use crate::pedido::Pedido;

pub trait Filtro {
    fn avaliar(&self, indice: usize, pedido: &Pedido) -> bool;
}

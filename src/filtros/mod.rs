mod efiltro;
mod intervalo;
mod produto;
mod valor_do_pedido;

use crate::pedido::Pedido;

pub trait Filtro: FiltroClone {
    fn avaliar(&self, indice: usize, pedido: &Pedido) -> bool;
}

trait FiltroClone {
    fn clone_box(&self) -> Box<dyn Filtro>;
}

impl<T> FiltroClone for T
    where T: 'static + Filtro + Clone
{
    fn clone_box(&self) -> Box<dyn Filtro> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Filtro> {
    fn clone(&self) -> Box<dyn Filtro> {
        self.clone_box()
    }
}

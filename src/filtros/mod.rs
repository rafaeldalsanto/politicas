pub mod efiltro;
pub mod intervalo;
pub mod produto;
pub mod valor_do_pedido;

use crate::pedido::Pedido;
use std::fmt::{Debug, Formatter, Result};

pub trait Filtro: FiltroClone + FiltroDebug {
    fn avaliar(&self, indice: usize, pedido: &Pedido) -> bool;
}

pub trait FiltroClone {
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

pub trait FiltroDebug {
    fn debug_box(&self, f: &mut Formatter<'_>) -> Result;
}

impl<T> FiltroDebug for T
    where T: 'static + Filtro + Debug
{
    fn debug_box(&self, f: &mut Formatter<'_>) -> Result {
        self.fmt(f)
    }
}

impl Debug for Box<dyn Filtro> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.debug_box(f)
    }
}

impl Default for Box<dyn Filtro> {
    fn default() -> Self {
        Box::new(FiltroFalso {})
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct FiltroFalso {}

impl Filtro for FiltroFalso {
    fn avaliar(&self, _: usize, _: &Pedido) -> bool {
        false
    }
}

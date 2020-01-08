use crate::filtros::Filtro;
use rust_decimal::Decimal;

#[derive(Clone, Debug, Default)]
pub struct Politica {
    pub id: u32,
    pub nome: String,
    pub regras: Vec<Regra>,
}

#[derive(Clone, Debug, Default)]
pub struct Regra {
    pub id: u32,
    pub editavel: bool,
    pub aplicavel_a_promocoes: bool,
    pub filtro: Box<dyn Filtro>,
    pub desconto_maximo: Decimal,
    pub desconto_sugerido: Decimal,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct RegraItemPedido {
    pub regra_id: u32,
    pub item_id: u32,
    pub desconto: Decimal,
}

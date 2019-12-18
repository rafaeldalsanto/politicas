use crate::filtros::Filtro;

#[derive(Clone, Debug)]
pub struct Politica {
    pub nome: String,
    pub regras: Vec<Regra>,
}

#[derive(Clone, Debug)]
pub struct Regra {
    pub editavel: bool,
    pub aplicavel_a_promocoes: bool,
    pub filtro: Box<dyn Filtro>,
    pub desconto_maximo: f64,
    pub desconto_sugerido: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct RegraItemPedido {
    pub regra_id: u32,
    pub item_id: u32,
    pub desconto: f64,
}

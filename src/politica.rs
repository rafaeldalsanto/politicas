use crate::filtros::Filtro;

pub struct Politica {
    pub nome: String,
    pub regras: Vec<Regra>,
}

pub struct Regra {
    pub editavel: bool,
    pub aplicavel_a_promocoes: bool,
    pub filtro: Box<dyn Filtro>,
    pub desconto_maximo: f64,
    pub desconto_sugerido: f64,
}

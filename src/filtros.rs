use crate::{Pedido, ItemDePedido};

pub trait Filtro {
    fn eh_satisfeito_por(&self, indice_do_item: u32, pedido: Pedido) -> bool;
}

pub struct ValorDoPedido {
    minimo: f64,
    maximo: f64,
}

impl Filtro for ValorDoPedido {
    fn eh_satisfeito_por(&self, indice_do_item: u32, pedido: Pedido) -> bool {
        let total = pedido.total();
        total >= self.minimo && total <= self.maximo
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pedido;

    #[test]
    fn filtro_total_do_pedido() {
        let filtro = ValorDoPedido { minimo: 1.0, maximo: 10.0};
        let mut pedido = Pedido {itens: vec![]};
        let item1 = ItemDePedido {
            quantidade: 2.0,
            preco_tabela: 5.0,
            descontos_do_vendedor: vec![10.0],
        };
        pedido.itens.push(item1);

        assert!(filtro.eh_satisfeito_por(0, pedido));
    }
}

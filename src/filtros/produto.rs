use std::collections::HashSet;
use crate::filtros::Filtro;
use crate::pedido::Pedido;

#[derive(Clone, Debug, Default)]
pub struct Produto {
    pub ids: HashSet<u32>,
}

impl Filtro for Produto {
    fn avaliar(&self, indice: usize, pedido: &Pedido) -> bool {
        self.ids.contains(&(pedido.itens)[indice].produto_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item_de_pedido::ItemDePedido;

    #[test]
    fn eh_satisfeito_quando_o_id_do_produto_existe() {
        let ids: HashSet<u32> = [1, 2, 3].iter().cloned().collect();
        let filtro = Produto { ids };
        let mut pedido = Pedido::new();
        pedido.adicionar_item(ItemDePedido { produto_id: 1, ..Default::default() });

        assert!(filtro.avaliar(0, &pedido));
    }

    #[test]
    fn nao_eh_satisfeito_quando_o_id_do_produto_nao_existe() {
        let ids: HashSet<u32> = [1, 2, 3].iter().cloned().collect();
        let filtro = Produto { ids };
        let mut pedido = Pedido::new();
        pedido.adicionar_item(ItemDePedido { produto_id: 4, ..Default::default() });

        assert!(!filtro.avaliar(0, &pedido));
    }
}
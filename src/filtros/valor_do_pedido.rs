use crate::pedido::Pedido;
use crate::filtros::intervalo::Intervalo;
use crate::filtros::Filtro;

#[derive(Copy, Clone, Debug)]
pub struct ValorDoPedido {
    pub intervalo: Intervalo,
}

impl Filtro for ValorDoPedido {
    fn avaliar(&self, _: usize, pedido: &Pedido) -> bool {
        self.intervalo.contem(pedido.total())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item_de_pedido::ItemDePedido;
    use crate::filtros::intervalo::Intervalo;

    #[test]
    fn eh_satisfeito_quando_o_total_do_pedido_esta_entre_o_minimo_e_o_maximo() {
        let intervalo = Intervalo { minimo: Some(1.0), maximo: Some(10.0) };
        let filtro = ValorDoPedido { intervalo };
        let mut pedido = Pedido::new();
        pedido.adicionar_item(ItemDePedido {
            quantidade: 2.0,
            preco_de_tabela: 4.0,
            ..Default::default()
        });

        assert!(filtro.avaliar(0, &pedido));
    }

    #[test]
    fn nao_eh_satisfeito_quando_o_total_do_pedido_nao_esta_entre_o_minimo_e_o_maximo() {
        let intervalo = Intervalo { minimo: Some(1.0), maximo: Some(10.0) };
        let filtro = ValorDoPedido { intervalo };
        let mut pedido = Pedido::new();
        pedido.adicionar_item(ItemDePedido {
            quantidade: 3.0,
            preco_de_tabela: 5.0,
            ..Default::default()
        });

        assert!(!filtro.avaliar(0, &pedido));
    }
}

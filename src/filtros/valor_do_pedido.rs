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
    use rust_decimal_macros::*;

    #[test]
    fn eh_satisfeito_quando_o_total_do_pedido_esta_entre_o_minimo_e_o_maximo() {
        let intervalo = Intervalo { minimo: Some(dec!(1)), maximo: Some(dec!(10)) };
        let filtro = ValorDoPedido { intervalo };
        let mut pedido = Pedido::new();
        pedido.adicionar_item(ItemDePedido {
            quantidade: dec!(2),
            preco_de_tabela: dec!(4),
            ..Default::default()
        });

        assert!(filtro.avaliar(0, &pedido));
    }

    #[test]
    fn nao_eh_satisfeito_quando_o_total_do_pedido_nao_esta_entre_o_minimo_e_o_maximo() {
        let intervalo = Intervalo { minimo: Some(dec!(1)), maximo: Some(dec!(10)) };
        let filtro = ValorDoPedido { intervalo };
        let mut pedido = Pedido::new();
        pedido.adicionar_item(ItemDePedido {
            quantidade: dec!(3),
            preco_de_tabela: dec!(5),
            ..Default::default()
        });

        assert!(!filtro.avaliar(0, &pedido));
    }
}

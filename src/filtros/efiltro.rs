use crate::filtros::Filtro;
use crate::pedido::Pedido;

#[derive(Clone, Debug)]
pub struct EFiltro {
    esquerda: Box<dyn Filtro>,
    direita: Box<dyn Filtro>,
}

impl Filtro for EFiltro {
    fn avaliar(&self, indice: usize, pedido: &Pedido) -> bool {
        self.esquerda.avaliar(indice, pedido) && self.direita.avaliar(indice, pedido)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::item_de_pedido::ItemDePedido;
    use crate::filtros::intervalo::Intervalo;
    use std::collections::HashSet;
    use crate::filtros::produto::Produto;
    use crate::filtros::valor_do_pedido::ValorDoPedido;
    use rust_decimal_macros::*;

    #[test]
    fn eh_satisfeito_quando_os_dois_filtros_sao_satisfeitos() {
        let ids: HashSet<u32> = [1, 2, 3].iter().cloned().collect();
        let filtro_esquerda = Produto { ids };
        let filtro_direita = ValorDoPedido { intervalo: Intervalo { minimo: Some(dec!(0)), maximo: Some(dec!(20)) } };
        let efiltro = EFiltro { esquerda: Box::new(filtro_esquerda), direita: Box::new(filtro_direita) };
        let mut pedido = Pedido::new();
        pedido.adicionar_item(ItemDePedido {
            produto_id: 1,
            quantidade: dec!(3),
            preco_de_tabela: dec!(5),
            ..Default::default()
        });

        assert!(efiltro.avaliar(0, &pedido));
    }

    #[test]
    fn nao_eh_satisfeito_quando_os_dois_filtros_nao_sao_satisfeitos() {
        let ids: HashSet<u32> = [1, 2, 3].iter().cloned().collect();
        let filtro_esquerda = Produto { ids };
        let filtro_direita = ValorDoPedido { intervalo: Intervalo { minimo: Some(dec!(0)), maximo: Some(dec!(5)) } };
        let efiltro = EFiltro { esquerda: Box::new(filtro_esquerda), direita: Box::new(filtro_direita) };
        let mut pedido = Pedido::new();
        pedido.adicionar_item(ItemDePedido {
            produto_id: 99,
            quantidade: dec!(3),
            preco_de_tabela: dec!(5),
            ..Default::default()
        });

        assert!(!efiltro.avaliar(0, &pedido));
    }

    #[test]
    fn nao_eh_satisfeito_quando_o_filtro_esquerda_nao_eh_satisfeito() {
        let ids: HashSet<u32> = [1, 2, 3].iter().cloned().collect();
        let filtro_esquerda = Produto { ids };
        let filtro_direita = ValorDoPedido { intervalo: Intervalo { minimo: Some(dec!(0)), maximo: Some(dec!(20)) } };
        let efiltro = EFiltro { esquerda: Box::new(filtro_esquerda), direita: Box::new(filtro_direita) };
        let mut pedido = Pedido::new();
        pedido.adicionar_item(ItemDePedido {
            produto_id: 99,
            quantidade: dec!(3),
            preco_de_tabela: dec!(5),
            ..Default::default()
        });

        assert!(!efiltro.avaliar(0, &pedido));
    }

    #[test]
    fn nao_eh_satisfeito_quando_o_filtro_direita_nao_eh_satisfeito() {
        let ids: HashSet<u32> = [1, 2, 3].iter().cloned().collect();
        let filtro_esquerda = Produto { ids };
        let filtro_direita = ValorDoPedido { intervalo: Intervalo { minimo: Some(dec!(0)), maximo: Some(dec!(10)) } };
        let efiltro = EFiltro { esquerda: Box::new(filtro_esquerda), direita: Box::new(filtro_direita) };
        let mut pedido = Pedido::new();
        pedido.adicionar_item(ItemDePedido {
            produto_id: 1,
            quantidade: dec!(3),
            preco_de_tabela: dec!(5),
            ..Default::default()
        });

        assert!(!efiltro.avaliar(0, &pedido));
    }
}

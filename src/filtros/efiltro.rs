use crate::filtros::Filtro;
use crate::pedido::Pedido;

pub struct EFiltro {
    esquerda: Box<dyn Filtro>,
    direita: Box<dyn Filtro>,
}

impl Filtro for EFiltro {
    fn eh_satisfeito_por(&self, indice: usize, pedido: &Pedido) -> bool {
        self.esquerda.eh_satisfeito_por(indice, pedido) &&
            self.direita.eh_satisfeito_por(indice, pedido)
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

    #[test]
    fn eh_satisfeito_quando_os_dois_filtros_sao_satisfeitos() {
        let ids: HashSet<u32> = [1, 2, 3].iter().cloned().collect();
        let filtro_esquerda = Produto { ids };
        let filtro_direita = ValorDoPedido {intervalo: Intervalo {minimo: Some(0.0), maximo: Some(20.0)}};
        let efiltro = EFiltro { esquerda: Box::new(filtro_esquerda), direita: Box::new(filtro_direita) };
        let mut pedido = Pedido::new();
        let item1 = ItemDePedido::new(1,3.0, 5.0, vec![10.0]);
        pedido.adicionar_item(item1);

        assert!(efiltro.eh_satisfeito_por(0, &pedido));
    }

    #[test]
    fn nao_eh_satisfeito_quando_os_dois_filtros_nao_sao_satisfeitos() {
        let ids: HashSet<u32> = [1, 2, 3].iter().cloned().collect();
        let filtro_esquerda = Produto { ids };
        let filtro_direita = ValorDoPedido {intervalo: Intervalo {minimo: Some(0.0), maximo: Some(5.0)}};
        let efiltro = EFiltro { esquerda: Box::new(filtro_esquerda), direita: Box::new(filtro_direita) };
        let mut pedido = Pedido::new();
        let item1 = ItemDePedido::new(99,3.0, 5.0, vec![10.0]);
        pedido.adicionar_item(item1);

        assert!(!efiltro.eh_satisfeito_por(0, &pedido));
    }

    #[test]
    fn nao_eh_satisfeito_quando_o_filtro_esquerda_nao_eh_satisfeito() {
        let ids: HashSet<u32> = [1, 2, 3].iter().cloned().collect();
        let filtro_esquerda = Produto { ids };
        let filtro_direita = ValorDoPedido {intervalo: Intervalo {minimo: Some(0.0), maximo: Some(20.0)}};
        let efiltro = EFiltro { esquerda: Box::new(filtro_esquerda), direita: Box::new(filtro_direita) };
        let mut pedido = Pedido::new();
        let item1 = ItemDePedido::new(99,3.0, 5.0, vec![10.0]);
        pedido.adicionar_item(item1);

        assert!(!efiltro.eh_satisfeito_por(0, &pedido));
    }

    #[test]
    fn nao_eh_satisfeito_quando_o_filtro_direita_nao_eh_satisfeito() {
        let ids: HashSet<u32> = [1, 2, 3].iter().cloned().collect();
        let filtro_esquerda = Produto { ids };
        let filtro_direita = ValorDoPedido {intervalo: Intervalo {minimo: Some(0.0), maximo: Some(10.0)}};
        let efiltro = EFiltro { esquerda: Box::new(filtro_esquerda), direita: Box::new(filtro_direita) };
        let mut pedido = Pedido::new();
        let item1 = ItemDePedido::new(1,3.0, 5.0, vec![10.0]);
        pedido.adicionar_item(item1);

        assert!(!efiltro.eh_satisfeito_por(0, &pedido));
    }
}
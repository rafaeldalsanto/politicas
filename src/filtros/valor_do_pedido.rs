use crate::pedido::Pedido;
use crate::filtros::intervalo::Intervalo;
use crate::filtros::traits::Filtro;

pub struct ValorDoPedido {
    intervalo: Intervalo,
}

impl Filtro for ValorDoPedido {
    fn eh_satisfeito_por(&self, _: u32, pedido: Pedido) -> bool {
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
        let item1 = ItemDePedido::new(2.0, 5.0, vec![10.0]);
        pedido.adicionar_item(item1);

        assert!(filtro.eh_satisfeito_por(0, pedido));
    }

    #[test]
    fn nao_eh_satisfeito_quando_o_total_do_pedido_nao_esta_entre_o_minimo_e_o_maximo() {
        let intervalo = Intervalo { minimo: Some(1.0), maximo: Some(10.0) };
        let filtro = ValorDoPedido { intervalo };
        let mut pedido = Pedido::new();
        let item1 = ItemDePedido::new(3.0, 5.0, vec![10.0]);
        pedido.adicionar_item(item1);

        assert!(!filtro.eh_satisfeito_por(0, pedido));
    }
}
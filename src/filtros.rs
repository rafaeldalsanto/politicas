use crate::pedido::Pedido;

pub trait Filtro {
    fn eh_satisfeito_por(&self, indice_do_item: u32, pedido: Pedido) -> bool;
}

pub struct ValorDoPedido {
    intervalo: Intervalo,
}

pub struct Intervalo {
    minimo: Option<f64>,
    maximo: Option<f64>,
}

impl Intervalo {
    pub fn contem(&self, valor: f64) -> bool {
        let minimo_ok = match self.minimo {
            None => true,
            Some(min) => valor >= min
        };
        let maximo_ok = match self.maximo {
            None => true,
            Some(max) => valor <= max
        };
        minimo_ok && maximo_ok
    }
}

impl Filtro for ValorDoPedido {
    fn eh_satisfeito_por(&self, indice_do_item: u32, pedido: Pedido) -> bool {
        self.intervalo.contem(pedido.total())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item_de_pedido::ItemDePedido;

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
    fn eh_satisfeito_quando_o_total_do_pedido_eh_menor_que_o_maximo() {
        let intervalo = Intervalo { minimo: None, maximo: Some(10.0) };
        let filtro = ValorDoPedido { intervalo };
        let mut pedido = Pedido::new();
        let item1 = ItemDePedido::new(2.0, 5.0, vec![10.0]);
        pedido.adicionar_item(item1);

        assert!(filtro.eh_satisfeito_por(0, pedido));
    }

    #[test]
    fn eh_satisfeito_quando_o_total_do_pedido_eh_maior_que_o_minimo() {
        let intervalo = Intervalo { minimo: Some(1.0), maximo: None };
        let filtro = ValorDoPedido { intervalo };
        let mut pedido = Pedido::new();
        let item1 = ItemDePedido::new(2.0, 5.0, vec![10.0]);
        pedido.adicionar_item(item1);

        assert!(filtro.eh_satisfeito_por(0, pedido));
    }

    #[test]
    fn nao_eh_satisfeito_quando_o_total_do_pedido_eh_maior_que_o_maximo() {
        let intervalo = Intervalo { minimo: None, maximo: Some(5.0) };
        let filtro = ValorDoPedido { intervalo };
        let mut pedido = Pedido::new();
        let item1 = ItemDePedido::new(2.0, 5.0, vec![10.0]);
        pedido.adicionar_item(item1);

        assert!(!filtro.eh_satisfeito_por(0, pedido));
    }

    #[test]
    fn nao_eh_satisfeito_quando_o_total_do_pedido_eh_menor_que_o_minimo() {
        let intervalo = Intervalo { minimo: Some(15.0), maximo: None };
        let filtro = ValorDoPedido { intervalo };
        let mut pedido = Pedido::new();
        let item1 = ItemDePedido::new(2.0, 5.0, vec![10.0]);
        pedido.adicionar_item(item1);

        assert!(!filtro.eh_satisfeito_por(0, pedido));
    }
}

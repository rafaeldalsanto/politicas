use crate::item_de_pedido::ItemDePedido;
use rust_decimal::Decimal;

#[derive(Clone, Debug, Default)]
pub struct Pedido {
    pub itens: Vec<ItemDePedido>,
}

impl Pedido {
    pub fn new() -> Pedido {
        Pedido {
            itens: Vec::new(),
        }
    }

    pub fn total(&self) -> Decimal {
        self.itens.iter().map(|item| item.total()).sum()
    }

    pub fn adicionar_item(&mut self, item: ItemDePedido) {
        self.itens.push(item);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::item_de_pedido::ItemDePedido;
    use crate::politica::RegraItemPedido;

    #[test]
    fn calcula_o_total_do_pedido() {
        let mut pedido = Pedido::new();
        pedido.adicionar_item(ItemDePedido {
            quantidade: Decimal::new(2, 0),
            preco_de_tabela: Decimal::new(5, 0),
            descontos_do_vendedor: vec![Decimal::new(2, 0)],
            promocoes: vec![RegraItemPedido { desconto: Decimal::new(3, 0), ..Default::default() }],
            ..Default::default()
        });
        pedido.adicionar_item(ItemDePedido {
            quantidade: Decimal::new(4, 0),
            preco_de_tabela: Decimal::new(5, 0),
            politicas: vec![RegraItemPedido { desconto: Decimal::new(5, 0), ..Default::default() }],
            ..Default::default()
        });

        assert_eq!(pedido.total(), Decimal::new(28506, 3));
    }
}

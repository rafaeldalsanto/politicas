use crate::item_de_pedido::ItemDePedido;
use rust_decimal::Decimal;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
    use rust_decimal_macros::*;

    #[test]
    fn calcula_o_total_do_pedido() {
        let mut pedido = Pedido::new();
        pedido.adicionar_item(ItemDePedido {
            quantidade: dec!(2),
            preco_de_tabela: dec!(5),
            descontos_do_vendedor: vec![dec!(2)],
            promocoes: vec![RegraItemPedido { desconto: dec!(3), ..Default::default() }],
            ..Default::default()
        });
        pedido.adicionar_item(ItemDePedido {
            quantidade: dec!(4),
            preco_de_tabela: dec!(5),
            politicas: vec![RegraItemPedido { desconto: dec!(5), ..Default::default() }],
            ..Default::default()
        });

        assert_eq!(pedido.total(), dec!(28.506));
    }
}

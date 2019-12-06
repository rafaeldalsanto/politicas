use crate::item_de_pedido::ItemDePedido;

pub struct Pedido {
    pub itens: Vec<ItemDePedido>,
}

impl Pedido {
    pub fn new() -> Pedido {
        Pedido {
            itens: Vec::new(),
        }
    }

    pub fn total(&self) -> f64 {
        self.itens.iter().map(|item| item.total()).sum()
    }

    pub fn adicionar_item(&mut self, item: ItemDePedido) {
        self.itens.push(item);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcula_o_total_do_pedido() {
        let item1 = ItemDePedido::new(1,2.0, 5.0, vec![10.0]);
        let item2 = ItemDePedido::new(1,4.0, 5.0, vec![10.0]);
        let mut pedido = Pedido::new();
        pedido.adicionar_item(item1);
        pedido.adicionar_item(item2);

        assert_eq!(pedido.total(), 27.0);
    }
}

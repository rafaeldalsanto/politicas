pub mod pedido {
    pub struct Pedido {
        itens: Vec<ItemDePedido>,
    }

    impl Pedido {
        fn total(&self) -> f64 {
            self.itens.iter().map(|item| item.total()).sum()
        }

        fn adicionar_item(&mut self, quantidade: f64, preco_tabela: f64, descontos_do_vendedor: Vec<f64>) {
            let item = ItemDePedido {
                quantidade,
                preco_tabela,
                descontos_do_vendedor,
            };
            self.itens.push(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_do_pedido() {
        let item1 = ItemDePedido {
            quantidade: 2.0,
            preco_tabela: 5.0,
            descontos_do_vendedor: vec![10.0]
        };

        let item2 = ItemDePedido {
            quantidade: 4.0,
            preco_tabela: 5.0,
            descontos_do_vendedor: vec![10.0]
        };

        let pedido = Pedido {itens: vec![item1, item2]};

        assert_eq!(pedido.total(), 27.0);
    }
}

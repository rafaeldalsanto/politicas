mod filtros;

fn main() {
    println!("Hello, world!");
}


struct Pedido {
    itens: Vec<ItemDePedido>,
}

impl Pedido {
    fn total(&self) -> f64 {
        self.itens.iter().fold(0.0, |total, item| total + item.total())
    };

    fn adicionar_item<'a>(&'a self, quantidade: f64, preco_tabela: f64, descontos_do_vendedor: Vec<f64>) -> ItemDePedido {
        let item = ItemDePedido {
            quantidade,
            preco_tabela,
            descontos_do_vendedor,
            pedido: self,
        };
        self.itens.push(item);
        item
    }
}

struct ItemDePedido {
    quantidade: f64,
    preco_tabela: f64,
    descontos_do_vendedor: Vec<f64>,
    pedido: Pedido,
}

impl ItemDePedido {
    fn preco_liquido(&self) -> f64 {
        aplicar_descontos(self.preco_tabela, &self.descontos_do_vendedor)
    }

    fn total(&self) -> f64 {
        self.quantidade * self.preco_liquido()
    }
}

fn aplicar_descontos(valor: f64, descontos: &Vec<f64>) -> f64 {
    descontos.iter().fold(valor, |desc_total, desc| desc_total * (1.0 - desc / 100.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aplicar_descontos() {
        let descontos = vec![10.0, 5.0];

        let resultado = aplicar_descontos(100.0, &descontos);
        
        assert_eq!(resultado, 85.5);
    }

    #[test]
    fn test_total_do_item() {
        let pedido = Pedido {itens: vec![]};
        let item = ItemDePedido {
            quantidade: 2.0,
            preco_tabela: 5.0,
            descontos_do_vendedor: vec![10.0],
            pedido
        };

        assert_eq!(item.total(), 9.0);
    }

    #[test]
    fn test_preco_liquido() {
        let item = ItemDePedido {
            quantidade: 2.0,
            preco_tabela: 5.0,
            descontos_do_vendedor: vec![10.0]
        };

        assert_eq!(item.preco_liquido(), 4.5)
    }

    #[test]
    fn test_total_do_pedido(){
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

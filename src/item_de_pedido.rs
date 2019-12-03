
struct ItemDePedido {
    quantidade: f64,
    preco_tabela: f64,
    descontos_do_vendedor: Vec<f64>,
}

impl ItemDePedido {
    fn new(quantidade: f64, preco_tabela: f64, descontos_do_vendedor: Vec<f64>) -> ItemDePedido {
        ItemDePedido {
            quantidade,
            preco_tabela,
            descontos_do_vendedor,
        }
    }

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
    fn aplica_descontos() {
        let descontos = vec![10.0, 5.0];

        let resultado = aplicar_descontos(100.0, &descontos);

        assert_eq!(resultado, 85.5);
    }

    #[test]
    fn calcula_o_total_do_item() {
        let item = ItemDePedido::new(2.0, 5.0, vec![10.0]);

        assert_eq!(item.total(), 9.0);
    }

    #[test]
    fn calcula_o_preco_liquido() {
        let item = ItemDePedido::new(2.0, 5.0, vec![10.0]);

        assert_eq!(item.preco_liquido(), 4.5)
    }
}

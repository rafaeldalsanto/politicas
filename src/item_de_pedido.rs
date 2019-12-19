use crate::politica::RegraItemPedido;

#[derive(Clone, Debug, Default)]
pub struct ItemDePedido {
    pub id: u32,
    pub pedido_id: u32,
    pub produto_id: u32,
    pub quantidade: f64,
    pub preco_de_tabela: f64,
    pub descontos_do_vendedor: Vec<f64>,
    pub promocoes: Vec<RegraItemPedido>,
    pub politicas: Vec<RegraItemPedido>,
}

impl ItemDePedido {
    pub fn descontos_de_promocoes(&self) -> Vec<f64> {
        self.promocoes.iter().map(|p| p.desconto).collect()
    }

    pub fn descontos_de_politicas(&self) -> Vec<f64> {
        self.politicas.iter().map(|p| p.desconto).collect()
    }

    pub fn preco_liquido(&self) -> f64 {
        let mut descontos = self.descontos_do_vendedor.clone();
        descontos.extend(self.descontos_de_promocoes());
        descontos.extend(self.descontos_de_politicas());
        aplicar_descontos(self.preco_de_tabela, &descontos)
    }

    pub fn total(&self) -> f64 {
        self.quantidade * self.preco_liquido()
    }
}

fn aplicar_descontos(valor: f64, descontos: &Vec<f64>) -> f64 {
    descontos.iter().fold(valor, |desc_total, desc| desc_total * (1.0 - desc / 100.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::politica::RegraItemPedido;

    #[test]
    fn aplica_descontos() {
        let resultado = aplicar_descontos(100.0, &vec![10.0, 5.0]);

        assert_eq!(resultado, 85.5);
    }

    #[test]
    fn calcula_o_preco_liquido() {
        let mut item = ItemDePedido {
            preco_de_tabela: 5.0,
            descontos_do_vendedor: vec![1.0, 2.0],
            promocoes: vec![
                RegraItemPedido { desconto: 3.0, ..Default::default() },
                RegraItemPedido { desconto: 4.0, ..Default::default() },
            ],
            politicas: vec![
                RegraItemPedido { desconto: 5.0, ..Default::default() },
                RegraItemPedido { desconto: 6.0, ..Default::default() },
            ],
            ..Default::default()
        };

        assert_eq!(item.preco_liquido(), 4.033905321599999)
    }

    #[test]
    fn calcula_o_total_do_item() {
        let mut item = ItemDePedido {
            quantidade: 2.0,
            preco_de_tabela: 5.0,
            descontos_do_vendedor: vec![1.0, 2.0],
            promocoes: vec![
                RegraItemPedido { desconto: 3.0, ..Default::default() },
                RegraItemPedido { desconto: 4.0, ..Default::default() },
            ],
            politicas: vec![
                RegraItemPedido { desconto: 5.0, ..Default::default() },
                RegraItemPedido { desconto: 6.0, ..Default::default() },
            ],
            ..Default::default()
        };

        assert_eq!(item.total(), 8.067810643199998);
    }

    #[test]
    fn retorna_os_descontos_de_promocoes() {
        let mut item = ItemDePedido {
            promocoes: vec![
                RegraItemPedido { desconto: 5.5, ..Default::default() },
                RegraItemPedido { desconto: 10.0, ..Default::default() },
            ],
            ..Default::default()
        };

        assert_eq!(item.descontos_de_promocoes(), vec![5.5, 10.0]);
    }

    #[test]
    fn retorna_os_descontos_de_politicas() {
        let mut item = ItemDePedido {
            politicas: vec![
                RegraItemPedido { desconto: 5.5, ..Default::default() },
                RegraItemPedido { desconto: 10.0, ..Default::default() },
            ],
            ..Default::default()
        };

        assert_eq!(item.descontos_de_politicas(), vec![5.5, 10.0]);
    }
}

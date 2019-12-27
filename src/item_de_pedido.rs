use crate::politica::RegraItemPedido;
use rust_decimal::Decimal;

#[derive(Clone, Debug, Default)]
pub struct ItemDePedido {
    pub id: u32,
    pub pedido_id: u32,
    pub produto_id: u32,
    pub quantidade: Decimal,
    pub preco_de_tabela: Decimal,
    pub descontos_do_vendedor: Vec<Decimal>,
    pub promocoes: Vec<RegraItemPedido>,
    pub politicas: Vec<RegraItemPedido>,
}

impl ItemDePedido {
    pub fn descontos_de_promocoes(&self) -> Vec<Decimal> {
        self.promocoes.iter().map(|p| p.desconto).collect()
    }

    pub fn descontos_de_politicas(&self) -> Vec<Decimal> {
        self.politicas.iter().map(|p| p.desconto).collect()
    }

    pub fn preco_liquido(&self) -> Decimal {
        let mut descontos = self.descontos_do_vendedor.clone();
        descontos.extend(self.descontos_de_promocoes());
        descontos.extend(self.descontos_de_politicas());
        aplicar_descontos(self.preco_de_tabela, &descontos)
    }

    pub fn total(&self) -> Decimal {
        self.quantidade * self.preco_liquido()
    }
}

fn aplicar_descontos(valor: Decimal, descontos: &Vec<Decimal>) -> Decimal {
    let um = Decimal::new(1, 0);
    let cem = Decimal::new(100, 0);
    descontos.iter().fold(valor, |desc_total, desc| desc_total * (um - desc / &cem))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::politica::RegraItemPedido;

    #[test]
    fn aplica_descontos() {
        let resultado = aplicar_descontos(
            Decimal::new(100, 0),
            &vec![Decimal::new(10, 0), Decimal::new(5, 0)]
        );

        assert_eq!(resultado, Decimal::new(855, 1));
    }

    #[test]
    fn calcula_o_preco_liquido() {
        let item = ItemDePedido {
            preco_de_tabela: Decimal::new(5, 0),
            descontos_do_vendedor: vec![Decimal::new(1, 0), Decimal::new(2, 0)],
            promocoes: vec![
                RegraItemPedido { desconto: Decimal::new(3, 0), ..Default::default() },
                RegraItemPedido { desconto: Decimal::new(4, 0), ..Default::default() },
            ],
            politicas: vec![
                RegraItemPedido { desconto: Decimal::new(5, 0), ..Default::default() },
                RegraItemPedido { desconto: Decimal::new(6, 0), ..Default::default() },
            ],
            ..Default::default()
        };

        assert_eq!(item.preco_liquido(), Decimal::new(40339053216, 10));
    }

    #[test]
    fn calcula_o_total_do_item() {
        let item = ItemDePedido {
            quantidade: Decimal::new(2, 0),
            preco_de_tabela: Decimal::new(5, 0),
            descontos_do_vendedor: vec![Decimal::new(1, 0), Decimal::new(2, 0)],
            promocoes: vec![
                RegraItemPedido { desconto: Decimal::new(3, 0), ..Default::default() },
                RegraItemPedido { desconto: Decimal::new(4, 0), ..Default::default() },
            ],
            politicas: vec![
                RegraItemPedido { desconto: Decimal::new(5, 0), ..Default::default() },
                RegraItemPedido { desconto: Decimal::new(6, 0), ..Default::default() },
            ],
            ..Default::default()
        };

        assert_eq!(item.total(), Decimal::new(80678106432, 10));
    }

    #[test]
    fn retorna_os_descontos_de_promocoes() {
        let item = ItemDePedido {
            promocoes: vec![
                RegraItemPedido { desconto: Decimal::new(55, 1), ..Default::default() },
                RegraItemPedido { desconto: Decimal::new(10, 0), ..Default::default() },
            ],
            ..Default::default()
        };

        assert_eq!(item.descontos_de_promocoes(), vec![Decimal::new(55, 1), Decimal::new(10, 0)]);
    }

    #[test]
    fn retorna_os_descontos_de_politicas() {
        let item = ItemDePedido {
            politicas: vec![
                RegraItemPedido { desconto: Decimal::new(55, 1), ..Default::default() },
                RegraItemPedido { desconto: Decimal::new(10, 0), ..Default::default() },
            ],
            ..Default::default()
        };

        assert_eq!(item.descontos_de_politicas(), vec![Decimal::new(55, 1), Decimal::new(10, 0)]);
    }
}

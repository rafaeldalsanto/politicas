use crate::politica::RegraItemPedido;
use rust_decimal::Decimal;
use rust_decimal_macros::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
    let um = dec!(1);
    let cem = dec!(100);
    descontos.iter().fold(valor, |desc_total, desc| desc_total * (um - desc / &cem))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::politica::RegraItemPedido;

    #[test]
    fn aplica_descontos() {
        let resultado = aplicar_descontos(
            dec!(100),
            &vec![dec!(10), dec!(5)]
        );

        assert_eq!(resultado, dec!(85.5));
    }

    #[test]
    fn calcula_o_preco_liquido() {
        let item = ItemDePedido {
            preco_de_tabela: dec!(5),
            descontos_do_vendedor: vec![dec!(1), dec!(2)],
            promocoes: vec![
                RegraItemPedido { desconto: dec!(3), ..Default::default() },
                RegraItemPedido { desconto: dec!(4), ..Default::default() },
            ],
            politicas: vec![
                RegraItemPedido { desconto: dec!(5), ..Default::default() },
                RegraItemPedido { desconto: dec!(6), ..Default::default() },
            ],
            ..Default::default()
        };

        assert_eq!(item.preco_liquido(), dec!(4.0339053216));
    }

    #[test]
    fn calcula_o_total_do_item() {
        let item = ItemDePedido {
            quantidade: dec!(2),
            preco_de_tabela: dec!(5),
            descontos_do_vendedor: vec![dec!(1), dec!(2)],
            promocoes: vec![
                RegraItemPedido { desconto: dec!(3), ..Default::default() },
                RegraItemPedido { desconto: dec!(4), ..Default::default() },
            ],
            politicas: vec![
                RegraItemPedido { desconto: dec!(5), ..Default::default() },
                RegraItemPedido { desconto: dec!(6), ..Default::default() },
            ],
            ..Default::default()
        };

        assert_eq!(item.total(), dec!(8.0678106432));
    }

    #[test]
    fn retorna_os_descontos_de_promocoes() {
        let item = ItemDePedido {
            promocoes: vec![
                RegraItemPedido { desconto: dec!(5.5), ..Default::default() },
                RegraItemPedido { desconto: dec!(10), ..Default::default() },
            ],
            ..Default::default()
        };

        assert_eq!(item.descontos_de_promocoes(), vec![dec!(5.5), dec!(10)]);
    }

    #[test]
    fn retorna_os_descontos_de_politicas() {
        let item = ItemDePedido {
            politicas: vec![
                RegraItemPedido { desconto: dec!(5.5), ..Default::default() },
                RegraItemPedido { desconto: dec!(10), ..Default::default() },
            ],
            ..Default::default()
        };

        assert_eq!(item.descontos_de_politicas(), vec![dec!(5.5), dec!(10)]);
    }
}

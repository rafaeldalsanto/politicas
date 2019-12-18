use crate::politica::RegraItemPedido;

#[derive(Clone)]
pub struct ItemDePedido {
    pub produto_id: u32,
    pub quantidade: f64,
    pub preco_tabela: f64,
    pub descontos_do_vendedor: Vec<f64>,
    pub politicas: Vec<RegraItemPedido>,
}

impl ItemDePedido {
    pub fn new(produto_id: u32, quantidade: f64, preco_tabela: f64, descontos_do_vendedor: Vec<f64>) -> ItemDePedido {
        ItemDePedido {
            produto_id,
            quantidade,
            preco_tabela,
            descontos_do_vendedor,
            politicas: Vec::new(),
        }
    }

    pub fn preco_liquido(&self) -> f64 {
        let mut descontos = self.descontos_do_vendedor.clone();
        descontos.extend(self.descontos_de_politicas());
        aplicar_descontos(self.preco_tabela, &descontos)
    }

    pub fn descontos_de_politicas(&self) -> Vec<f64> {
        self.politicas.iter().map(|p| p.desconto).collect()
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

    #[test]
    fn aplica_descontos() {
        let resultado = aplicar_descontos(100.0, &vec![10.0, 5.0]);
        assert_eq!(resultado, 85.5);
    }

    #[test]
    fn calcula_o_preco_liquido() {
        let mut item = ItemDePedido::new(1, 2.0, 5.0, vec![10.0]);
        item.politicas.push(RegraItemPedido {regra_id: 10, item_id: 100, desconto: 10.0});
        item.politicas.push(RegraItemPedido {regra_id: 11, item_id: 100, desconto: 5.0});
        assert_eq!(item.preco_liquido(), 3.8474999999999997)
    }

    #[test]
    fn calcula_o_total_do_item() {
        let item = ItemDePedido::new(1, 2.0, 5.0, vec![10.0]);
        assert_eq!(item.total(), 9.0);
    }

    #[test]
    fn retorna_os_descontos_de_politicas() {
        let mut item = ItemDePedido::new(1, 2.0, 5.0, vec![]);
        item.politicas.push(RegraItemPedido {regra_id: 10, item_id: 100, desconto: 5.5});
        item.politicas.push(RegraItemPedido {regra_id: 11, item_id: 100, desconto: 10.0});
        assert_eq!(item.descontos_de_politicas(), vec![5.5, 10.0]);
    }
}

use crate::politica::Politica;
use crate::pedido::Pedido;
//use std::collections::HashMap;

pub fn processar_politicas(politicas: &Vec<Politica>, pedido: &Pedido) -> Pedido {
    let novo_pedido = pedido.clone();

    if politicas.is_empty() {
        return novo_pedido;
    }

    novo_pedido
}

//fn remover_descontos_de_politicas_existentes(pedido: &Pedido) -> (Pedido, HashMap<(u32, u32), f64>) {
//    let mut descontos_antigos: HashMap<(u32, u32), f64> = HashMap::new();
//
//    for mut item in pedido.itens {
//        for regra_item_pedido in item.politicas {
//            descontos_antigos.insert((regra_item_pedido.regra_id, regra_item_pedido.item_id), regra_item_pedido.desconto);
//        }
//        item.politicas = Vec::new()
//    }
//    (pedido, descontos_antigos)
//}

#[cfg(test)]
mod test {
    use super::*;
    use crate::item_de_pedido::ItemDePedido;
    use rust_decimal_macros::*;

    #[test]
    fn retorna_o_pedido_original_se_nao_existem_politicas() {
        let pedido_original = Pedido {
            itens: vec![
                ItemDePedido {
                    id: 1,
                    produto_id: 2,
                    quantidade: dec!(3),
                    preco_de_tabela: dec!(4),
                    ..Default::default()
                }
            ]
        };

        let novo_pedido = processar_politicas(&Vec::new(), &pedido_original);

        assert_eq!(pedido_original, novo_pedido);
    }
}
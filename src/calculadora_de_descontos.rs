use crate::politica::{Politica};
use crate::pedido::Pedido;
//use std::collections::HashMap;
//use rust_decimal::Decimal;
use crate::item_de_pedido::ItemDePedido;

pub fn processar_politicas(politicas: &Vec<Politica>, pedido: &Pedido) -> Pedido {
    let novo_pedido = clonar_pedido_sem_politicas(pedido);

    if politicas.is_empty() {
        return novo_pedido;
    }

    novo_pedido
}

fn clonar_pedido_sem_politicas(pedido: &Pedido) -> Pedido {
    Pedido {
        itens: pedido.itens.iter().map(remove_politicas).collect(),
        ..pedido.clone()
    }
}

fn remove_politicas(item: &ItemDePedido) -> ItemDePedido {
    ItemDePedido {
        politicas: vec![],
        ..item.clone()
    }
}

//fn remover_descontos_de_politicas_existentes(pedido: &mut Pedido) -> HashMap<(u32, u32), Decimal> {
//    let mut descontos_antigos: HashMap<(u32, u32), Decimal> = HashMap::new();
//
//    for mut item in pedido.itens {
//        for regra_item_pedido in item.politicas {
//            descontos_antigos.insert((regra_item_pedido.regra_id, regra_item_pedido.item_id), regra_item_pedido.desconto);
//        }
//        item.politicas = Vec::new()
//    }
//    descontos_antigos
//}

#[cfg(test)]
mod test {
    use super::*;
    use crate::item_de_pedido::ItemDePedido;
    use rust_decimal_macros::*;
    use crate::politica::RegraItemPedido;

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

    #[test]
    fn remove_as_politicas_aplicadas_ao_pedido_antes_de_aplicar_as_novas() {
        let pedido_original = Pedido {
            itens: vec![
                ItemDePedido {
                    id: 1,
                    produto_id: 2,
                    quantidade: dec!(1),
                    preco_de_tabela: dec!(100),
                    politicas: vec![
                        RegraItemPedido {
                            regra_id: 3,
                            item_id: 1,
                            desconto: dec!(10),
                        }
                    ],
                    ..Default::default()
                }
            ]
        };

        let novo_pedido = processar_politicas(&Vec::new(), &pedido_original);

        assert_eq!(dec!(100), novo_pedido.total());
    }
}
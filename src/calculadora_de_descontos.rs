use crate::politica::{Politica, RegraItemPedido};
use crate::pedido::Pedido;
use std::collections::HashMap;
use rust_decimal::Decimal;
use crate::item_de_pedido::ItemDePedido;

pub fn processar_politicas(politicas: &Vec<Politica>, pedido: &Pedido) -> Pedido {
    let descontos_antigos = obter_descontos_de_politicas(pedido);
    let mut novo_pedido = clonar_pedido_sem_politicas(pedido);

    if politicas.is_empty() {
        return novo_pedido;
    }

    let mut novos_descontos: HashMap<u32, Vec<RegraItemPedido>> = HashMap::new();
    for politica in politicas.iter() {
        for (indice, item) in novo_pedido.itens.iter().enumerate() {
            let mut descontos: Vec<RegraItemPedido> = vec![];
            for regra in politica.regras.iter() {
                if regra.filtro.avaliar(indice, &novo_pedido) {

                    descontos.push(RegraItemPedido {
                        regra_id: regra.id,
                        item_id: item.id,
                        desconto: get_or_default(&descontos_antigos, &(regra.id, item.id), regra.desconto_sugerido)
                    });
                }
            }
            novos_descontos.insert(item.id, descontos);
        }

        for item in novo_pedido.itens.iter_mut() {
            item.politicas.extend(novos_descontos.get(&item.id).unwrap())
        }
    }

    novo_pedido
}

fn get_or_default(hm: &HashMap<(u32, u32), Decimal>, key: &(u32, u32), default: Decimal) -> Decimal {
    match hm.get(key) {
        Some(v) => v.clone(),
        None => default.clone()
    }
}

fn clonar_pedido_sem_politicas(pedido: &Pedido) -> Pedido {
    Pedido {
        itens: pedido.itens.iter().map(clonar_item_sem_politicas).collect(),
        ..pedido.clone()
    }
}

fn clonar_item_sem_politicas(item: &ItemDePedido) -> ItemDePedido {
    ItemDePedido {
        politicas: vec![],
        ..item.clone()
    }
}

fn obter_descontos_de_politicas(pedido: &Pedido) -> HashMap<(u32, u32), Decimal> {
    let mut descontos_antigos: HashMap<(u32, u32), Decimal> = HashMap::new();

    for item in pedido.itens.iter() {
        for regra_item_pedido in item.politicas.iter() {
            descontos_antigos.insert((regra_item_pedido.regra_id, regra_item_pedido.item_id), regra_item_pedido.desconto);
        }
    }
    descontos_antigos
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::item_de_pedido::ItemDePedido;
    use rust_decimal_macros::*;
    use crate::politica::{RegraItemPedido, Regra};
    use crate::filtros::valor_do_pedido::ValorDoPedido;
    use crate::filtros::intervalo::Intervalo;

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

    #[test]
    fn aplica_desconto_de_uma_regra() {
        let intervalo = Intervalo { minimo: Some(dec!(0)), maximo: Some(dec!(1000)) };
        let filtro = ValorDoPedido { intervalo };
        let politica = Politica {
            regras: vec![Regra {
                filtro: Box::new(filtro),
                desconto_maximo: dec!(10),
                desconto_sugerido: dec!(10),
                ..Default::default()
            }],
            ..Default::default()
        };

        let pedido_original = Pedido {
            itens: vec![
                ItemDePedido {
                    id: 1,
                    quantidade: dec!(1),
                    preco_de_tabela: dec!(100),
                    ..Default::default()
                }
            ]
        };

        let pedido_com_descontos = processar_politicas(&vec![politica], &pedido_original);

        assert_eq!(pedido_com_descontos.total(), dec!(90));
    }

    #[test]
    fn mantem_o_desconto_dado_pelo_vendedor_se_o_item_ganhar_a_mesma_regra() {
        let intervalo = Intervalo { minimo: Some(dec!(0)), maximo: Some(dec!(1000)) };
        let filtro = ValorDoPedido { intervalo };
        let politica = Politica {
            regras: vec![Regra {
                id: 2,
                filtro: Box::new(filtro),
                desconto_maximo: dec!(10),
                desconto_sugerido: dec!(10),
                ..Default::default()
            }],
            ..Default::default()
        };

        let pedido_original = Pedido {
            itens: vec![
                ItemDePedido {
                    id: 1,
                    quantidade: dec!(1),
                    preco_de_tabela: dec!(100),
                    politicas: vec![RegraItemPedido {
                        regra_id: politica.regras[0].id,
                        item_id: 1,
                        desconto: dec!(5)
                    }],
                    ..Default::default()
                }
            ]
        };

        let pedido_com_descontos = processar_politicas(&vec![politica], &pedido_original);

        assert_eq!(pedido_com_descontos.total(), dec!(95));
    }
}
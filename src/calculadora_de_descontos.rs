use crate::politica::Politica;
use crate::pedido::Pedido;
use std::collections::HashMap;

pub fn processar_politicas(politicas: Vec<Politica>, pedido: Pedido) -> Pedido {
    if politicas.is_empty() {
        return pedido
    }

    pedido
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
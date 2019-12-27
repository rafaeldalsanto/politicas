use rust_decimal::Decimal;

#[derive(Copy, Clone, Debug, Default)]
pub struct Intervalo {
    pub minimo: Option<Decimal>,
    pub maximo: Option<Decimal>,
}

impl Intervalo {
    pub fn contem(&self, valor: Decimal) -> bool {
        let minimo_ok = match self.minimo {
            None => true,
            Some(min) => valor >= min
        };
        let maximo_ok = match self.maximo {
            None => true,
            Some(max) => valor <= max
        };
        minimo_ok && maximo_ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contem_quando_o_valor_esta_entre_o_minimo_e_o_maximo() {
        let intervalo = Intervalo { minimo: Some(Decimal::new(1, 0)), maximo: Some(Decimal::new(10, 0)) };
        assert!(intervalo.contem(Decimal::new(5, 0)));
    }

    #[test]
    fn contem_quando_o_valor_eh_menor_que_o_maximo() {
        let intervalo = Intervalo { minimo: None, maximo: Some(Decimal::new(10, 0)) };
        assert!(intervalo.contem(Decimal::new(5, 0)));
    }

    #[test]
    fn contem_quando_o_valor_eh_maior_que_o_minimo() {
        let intervalo = Intervalo { minimo: Some(Decimal::new(1, 0)), maximo: None };
        assert!(intervalo.contem(Decimal::new(5, 0)));
    }

    #[test]
    fn nao_contem_quando_o_valor_eh_maior_que_o_maximo() {
        let intervalo = Intervalo { minimo: None, maximo: Some(Decimal::new(5, 0)) };
        assert!(!intervalo.contem(Decimal::new(10, 0)));
    }

    #[test]
    fn nao_contem_quando_o_valor_eh_menor_que_o_minimo() {
        let intervalo = Intervalo { minimo: Some(Decimal::new(15, 0)), maximo: None };
        assert!(!intervalo.contem(Decimal::new(10, 0)));
    }
}

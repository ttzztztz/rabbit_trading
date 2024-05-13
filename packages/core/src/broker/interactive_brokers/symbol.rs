use std::collections::HashMap;

use super::config::IBConfig;
use crate::model::trading::symbol::Symbol;

pub struct IBSymbolHelper {
    // Load into memory
    symbol_to_conid: HashMap<String, i64>,
    conid_to_symbol: HashMap<i64, String>,
}

impl IBSymbolHelper {
    pub fn new(config: IBConfig) -> Self {
        let symbol_to_conid = config.symbol_to_conid.clone();
        let conid_to_symbol = config
            .symbol_to_conid
            .clone()
            .into_iter()
            .map(|(symbol, conid)| (conid, symbol))
            .collect();

        IBSymbolHelper {
            symbol_to_conid,
            conid_to_symbol,
        }
    }

    pub fn get_conid(&self, symbol: &Symbol) -> Option<i64> {
        let symbol_string = symbol.to_string();
        self.symbol_to_conid
            .get(&symbol_string)
            .map(|val| val.clone())
    }

    pub fn get_symbol(&self, conid: i64) -> Option<Symbol> {
        let symbol_string_option = self.conid_to_symbol.get(&conid).map(|val| val.clone());
        symbol_string_option.map(|_| todo!("parse symbol string to symbol")) // TODO
    }
}

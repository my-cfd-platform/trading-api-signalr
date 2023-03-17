use my_nosql_contracts::TradingInstrumentGroupNoSqlEntity;

use crate::InstrumentGroupSignalRModel;

impl From<TradingInstrumentGroupNoSqlEntity> for InstrumentGroupSignalRModel {
    fn from(src: TradingInstrumentGroupNoSqlEntity) -> Self {
        Self {
            id: src.id,
            name: src.name,
            weight: src.weight,
        }
    }
}

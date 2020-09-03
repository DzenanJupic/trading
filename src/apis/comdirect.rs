use trading_utils::{BrokerInterface, Order, Position, BrokerCapability};

struct Comdirect {

}

impl BrokerInterface for Comdirect {
    const NAME: &'static str = "comdirect";

    fn capabilities(&self) -> &[BrokerCapability] {
        use trading_utils::BrokerCapability::*;

        &[
            // todo
        ]
    }

    fn login(&mut self) {
        unimplemented!()
    }

    fn logout(&mut self) {
        unimplemented!()
    }

    fn all_deposits(&self) {
        unimplemented!()
    }

    fn deposit_balance(&self) {
        unimplemented!()
    }

    fn deposit_transactions(&self) {
        unimplemented!()
    }

    fn all_orders(&self) {
        unimplemented!()
    }

    fn get_order(&self) {
        unimplemented!()
    }

    fn change_order(&self) {
        unimplemented!()
    }

    fn delete_order(&self) {
        unimplemented!()
    }

    fn all_positions(&self) {
        unimplemented!()
    }

    fn get_positions(&self) {
        unimplemented!()
    }

    fn buy(&self, order: Order) {
        unimplemented!()
    }

    fn sell(&self, position: Position) {
        unimplemented!()
    }
}
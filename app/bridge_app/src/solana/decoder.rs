use anchor_lang::AnchorDeserialize;
use anchor_lang::Discriminator;

use crate::types::OrderFormatter;

pub enum ProgramEvent {
    OrderCancelled(bridge::OrderCancelled),
    OrderCompleted(bridge::OrderCompleted),
    OrderCreated(bridge::OrderCreated),
}

pub fn decode(data: &[u8]) -> Option<OrderFormatter> {
    if data.len() < 8 {
        return None;
    }
    let disc = &data[..8];
    let payload = &data[8..];
    if disc == bridge::OrderCreated::DISCRIMINATOR {
        let decoded = bridge::OrderCreated::try_from_slice(payload).unwrap();
        print!(
            "Decoded msg Order Created {:?},{:?},{:?},{:?},{:?},{:?},{:?}",
            decoded.amount0,
            decoded.amount1,
            decoded.receiver,
            decoded.sender,
            decoded.timecreation,
            decoded.token0,
            decoded.token1
        );
        let order = OrderFormatter::new(
            decoded.timecreation,
            0,
            decoded.token1,
            decoded.token0.to_string(),
            decoded.amount0,
            decoded.amount1,
            decoded.receiver,
            decoded.sender.to_string(),
        );
        return Some(order);
    }
    if disc == bridge::OrderCompleted::DISCRIMINATOR {
        let decoded = bridge::OrderCompleted::try_from_slice(payload).unwrap();
        print!(
            "Decoded msg Order Completed {:?},{:?},{:?},{:?},{:?},{:?},{:?}",
            decoded.amount0,
            decoded.amount1,
            decoded.receiver,
            decoded.sender,
            decoded.timeexecuted,
            decoded.token0,
            decoded.token1,
        );
        let order = OrderFormatter::new(
            decoded.timestarted,
            decoded.timeexecuted,
            decoded.token0,
            decoded.token1.to_string(),
            decoded.amount0,
            decoded.amount1,
            decoded.sender,
            decoded.receiver.to_string(),
        );
        return Some(order);
    }
    if disc == bridge::OrderCancelled::DISCRIMINATOR {
        let decoded = bridge::OrderCancelled::try_from_slice(payload).unwrap();
        print!(
            "Decoded msg Order Canceled{:?},{:?},{:?},{:?},{:?},{:?}",
            decoded.amount0,
            decoded.maker,
            decoded.order_id,
            decoded.time_cancelled,
            decoded.token0,
            decoded.token1
        );
        let order = OrderFormatter::new(
            decoded.timestarted,
            decoded.time_cancelled,
            decoded.token1,
            decoded.token0.to_string(),
            decoded.amount0,
            0,
            String::new(),
            decoded.maker.to_string(),
        );
        return Some(order);
    }
    return None;
}

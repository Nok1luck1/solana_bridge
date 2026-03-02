use anchor_lang::idl;
use anchor_lang::AccountDeserialize;
use anchor_lang::AnchorDeserialize;
use anchor_lang::Discriminator;
use anchor_lang::InstructionData;
use bridge::OrderExecution;

use crate::eth::Bridge::OrderCreated;

pub fn decode(data: &[u8]) {
    if data.len() < 8 {
        return;
    }
    let disc = &data[..8];
    let payload = &data[8..];
    if disc == bridge::OrderCreated::DISCRIMINATOR {
        if let Ok(decoded) = bridge::OrderCreated::try_from_slice(payload) {
            print!(
                "decoded msg Order Created {:?},{:?},{:?},{:?},{:?},{:?},{:?}",
                decoded.amount0,
                decoded.amount1,
                decoded.receiver,
                decoded.sender,
                decoded.timecreation,
                decoded.token0,
                decoded.token1
            );
        }
        return;
    }
    if disc == bridge::OrderCompleted::DISCRIMINATOR {
        if let Ok(decoded) = bridge::OrderCompleted::try_from_slice(payload) {
            print!(
                "decoded msg Order completed {:?},{:?},{:?},{:?},{:?},{:?},{:?}",
                decoded.amount0,
                decoded.amount1,
                decoded.receiver,
                decoded.sender,
                decoded.timeexecuted,
                decoded.token0,
                decoded.token1,
            );
        }
        return;
    }
    if disc == bridge::OrderCancelled::DISCRIMINATOR {
        if let Ok(decoded) = bridge::OrderCancelled::try_from_slice(payload) {
            print!(
                "decoded msg Order Canceled{:?},{:?},{:?},{:?},{:?},{:?}",
                decoded.amount0,
                decoded.maker,
                decoded.order_id,
                decoded.time_cancelled,
                decoded.token0,
                decoded.token1
            );
        }
        return;
    }
}

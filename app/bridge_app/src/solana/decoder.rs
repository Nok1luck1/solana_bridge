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
    let payload = &data[..8];
    if disc == bridge::OrderCreated::DISCRIMINATOR {
        if let Ok(decoded) = bridge::OrderCreated::try_from_slice(payload) {
            print!(
                "decoded msg {:?},{:?},{:?},{:?},{:?},{:?},{:?}",
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
    if disc == bridge::OrderExecution::DISCRIMINATOR {
        if let Ok(decoded) = bridge::OrderExecution::try_from_slice(payload) {
            print!(
                "decoded msg {:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",
                decoded.bump,
                decoded.id,
                decoded.maker,
                decoded.receiver,
                decoded.timeend,
                decoded.token0,
                decoded.token0amount,
                decoded.token1,
                decoded.token1amount
            );
        }
        return;
    }
    if disc == bridge::OrderCreated::DISCRIMINATOR {
        if let Ok(decoded) = bridge::OrderCreated::try_from_slice(payload) {
            print!(
                "decoded msg {:?},{:?},{:?},{:?},{:?}",
                decoded.timecreation,
                decoded.token0,
                decoded.token1,
                decoded.amount0,
                decoded.amount1
            );
        }
        return;
    }
}

use crate::mock::*;
use hex_literal::hex;
use parity_scale_codec::Encode;
use sp_consensus_aura::inherents::InherentType;
use sp_consensus_aura::AURA_ENGINE_ID;
use sp_runtime::generic::DigestItem;
use sp_runtime::traits::BlakeTwo256;
#[test]
fn test_author_id_insertion() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| match relay_block_num {
            1 => {
                let slot: InherentType = 13u64.into();
                sproof.author_id =
                    HeaderAs::NonEncoded(sp_runtime::generic::Header::<u32, BlakeTwo256> {
                        parent_hash: Default::default(),
                        number: Default::default(),
                        state_root: Default::default(),
                        extrinsics_root: Default::default(),
                        digest: sp_runtime::generic::Digest {
                            logs: vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())],
                        },
                    })
            }
            _ => unreachable!(),
        })
        .add(1, || {
            assert_eq!(AuthorRewarding::latest_author(), Some(13u64));
        });
}

#[test]
fn test_author_id_insertion_real_data() {
    BlockTests::new()
        .with_relay_sproof_builder(|_, relay_block_num, sproof| {
            // Statemint data:
            // Block: 3,511,063
            // Slot: 140,006,956
            let statemint_data = hex!(
                "5d1b54ce2845dedd7f43805849747c44388b7b7cc84dc5083815cc2b58b513145e4cd6000a98bf2792
                 1e16366f5a2a388595f87744608684f43ff613026241634390d0c28a9dee52544070b989c71634db54
                 222b86391a75fa37d12544e7022bcd3cd42a080661757261202c56580800000000056175726101018f
                 b36de33276e8d54f77ea0a006ed7ab97b8d0aad00869f7ce6a5709eb1fc3256428b8b2428a2a3ec4fa
                 1c1058ab0e33c5a6b2b5789ab7b3e0accaeccafb4506"
            );

            match relay_block_num {
                1 => sproof.author_id = HeaderAs::AlreadyEncoded(statemint_data.to_vec()),
                _ => unreachable!(),
            }
        })
        .add(1, || {
            assert_eq!(
                AuthorRewarding::latest_author(),
                // Our mock author fetcher will just note the slot
                Some(140006956u64)
            );
        });
}

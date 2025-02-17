//! The Solana host and client SDK.
//!
//! This is the base library for all off-chain programs that interact with
//! Solana or otherwise operate on Solana data structures. On-chain programs
//! instead use the [`solana-program`] crate, the modules of which are
//! re-exported by this crate, like the relationship between the Rust
//! `core` and `std` crates. As much of the functionality of this crate is
//! provided by `solana-program`, see that crate's documentation for an
//! overview.
//!
//! [`solana-program`]: https://docs.rs/solana-program
//!
//! Many of the modules in this crate are primarily of use to the Solana runtime
//! itself. Additional crates provide capabilities built on `solana-sdk`, and
//! many programs will need to link to those crates as well, particularly for
//! clients communicating with Solana nodes over RPC.
//!
//! Such crates include:
//!
//! - [`solana-client`] - For interacting with a Solana node via the [JSON-RPC API][json].
//! - [`solana-cli-config`] - Loading and saving the Solana CLI configuration file.
//! - [`solana-clap-utils`] - Routines for setting up the CLI using [`clap`], as
//!   used by the Solana CLI. Includes functions for loading all types of
//!   signers supported by the CLI.
//!
//! [`solana-client`]: https://docs.rs/solana-client
//! [`solana-cli-config`]: https://docs.rs/solana-cli-config
//! [`solana-clap-utils`]: https://docs.rs/solana-clap-utils
//! [json]: https://solana.com/docs/rpc
//! [`clap`]: https://docs.rs/clap

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

// Allows macro expansion of `use ::solana_sdk::*` to work within this crate
extern crate self as solana_sdk;

#[cfg(feature = "full")]
pub use solana_commitment_config as commitment_config;
#[cfg(not(target_os = "solana"))]
pub use solana_program::program_stubs;
// These solana_program imports could be *-imported, but that causes a bunch of
// confusing duplication in the docs due to a rustdoc bug. #26211
#[allow(deprecated)]
pub use solana_program::sdk_ids;
#[cfg(target_arch = "wasm32")]
pub use solana_program::wasm_bindgen;
pub use solana_program::{
    account_info, address_lookup_table, big_mod_exp, blake3, bpf_loader, bpf_loader_deprecated,
    bpf_loader_upgradeable, clock, config, custom_heap_default, custom_panic_default,
    debug_account_data, declare_deprecated_sysvar_id, declare_sysvar_id, ed25519_program,
    epoch_rewards, epoch_schedule, fee_calculator, impl_sysvar_get, incinerator, instruction,
    keccak, lamports, loader_instruction, loader_upgradeable_instruction, loader_v4,
    loader_v4_instruction, message, msg, native_token, nonce, program, program_error,
    program_option, program_pack, rent, secp256k1_program, serialize_utils, slot_hashes,
    slot_history, stable_layout, stake, stake_history, syscalls, system_instruction,
    system_program, sysvar, unchecked_div_by_const, vote,
};
#[cfg(feature = "borsh")]
pub use solana_program::{borsh, borsh0_10, borsh1};
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `solana-signer` crate instead")]
pub use solana_signer::signers;
pub mod entrypoint;
pub mod entrypoint_deprecated;
pub mod example_mocks;
pub mod feature;
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `solana-hard-forks` crate instead")]
pub use solana_hard_forks as hard_forks;
pub mod hash;
pub mod log;
pub mod native_loader;
pub mod net;
pub mod precompiles;
pub mod program_utils;
pub mod pubkey;
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `solana_rent_collector` crate instead")]
pub use solana_rent_collector as rent_collector;
#[deprecated(since = "2.2.0", note = "Use `solana-reward-info` crate instead")]
pub mod reward_info {
    pub use solana_reward_info::RewardInfo;
}
#[deprecated(since = "2.2.0", note = "Use `solana-reward-info` crate instead")]
pub mod reward_type {
    pub use solana_reward_info::RewardType;
}
pub mod rpc_port;
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `solana-shred-version` crate instead")]
pub use solana_shred_version as shred_version;
pub mod signature;
pub mod signer;
pub mod transaction;
pub mod transport;
pub mod wasm;

#[deprecated(since = "2.1.0", note = "Use `solana-account` crate instead")]
pub use solana_account as account;
#[deprecated(
    since = "2.1.0",
    note = "Use `solana_account::state_traits` crate instead"
)]
pub use solana_account::state_traits as account_utils;
#[deprecated(since = "2.1.0", note = "Use `solana-bn254` crate instead")]
pub use solana_bn254 as alt_bn128;
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `solana-client-traits` crate instead")]
pub use solana_client_traits as client;
#[deprecated(
    since = "2.2.0",
    note = "Use `solana-compute-budget-interface` crate instead"
)]
#[cfg(feature = "full")]
pub use solana_compute_budget_interface as compute_budget;
#[deprecated(since = "2.1.0", note = "Use `solana-decode-error` crate instead")]
pub use solana_decode_error as decode_error;
#[deprecated(since = "2.1.0", note = "Use `solana-derivation-path` crate instead")]
pub use solana_derivation_path as derivation_path;
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `solana-ed25519-program` crate instead")]
pub use solana_ed25519_program as ed25519_instruction;
#[deprecated(since = "2.2.0", note = "Use `solana-epoch-info` crate instead")]
pub use solana_epoch_info as epoch_info;
#[deprecated(
    since = "2.2.0",
    note = "Use `solana-epoch-rewards-hasher` crate instead"
)]
pub use solana_epoch_rewards_hasher as epoch_rewards_hasher;
#[deprecated(since = "2.1.0", note = "Use `solana-feature-set` crate instead")]
pub use solana_feature_set as feature_set;
#[deprecated(since = "2.2.0", note = "Use `solana-fee-structure` crate instead")]
pub use solana_fee_structure as fee;
#[deprecated(since = "2.1.0", note = "Use `solana-inflation` crate instead")]
pub use solana_inflation as inflation;
#[deprecated(
    since = "2.2.0",
    note = "Use `solana_message::inner_instruction` instead"
)]
pub use solana_message::inner_instruction;
#[deprecated(since = "2.2.0", note = "Use `solana-nonce-account` crate instead")]
pub use solana_nonce_account as nonce_account;
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `solana-offchain-message` crate instead")]
pub use solana_offchain_message as offchain_message;
#[deprecated(since = "2.1.0", note = "Use `solana-packet` crate instead")]
pub use solana_packet as packet;
#[deprecated(since = "2.2.0", note = "Use `solana-poh-config` crate instead")]
pub use solana_poh_config as poh_config;
#[deprecated(since = "2.1.0", note = "Use `solana-program-memory` crate instead")]
pub use solana_program_memory as program_memory;
#[deprecated(since = "2.1.0", note = "Use `solana_pubkey::pubkey` instead")]
/// Convenience macro to define a static public key.
///
/// Input: a single literal base58 string representation of a Pubkey
///
/// # Example
///
/// ```
/// use std::str::FromStr;
/// use solana_program::{pubkey, pubkey::Pubkey};
///
/// static ID: Pubkey = pubkey!("My11111111111111111111111111111111111111111");
///
/// let my_id = Pubkey::from_str("My11111111111111111111111111111111111111111").unwrap();
/// assert_eq!(ID, my_id);
/// ```
pub use solana_pubkey::pubkey;
#[cfg(feature = "full")]
#[deprecated(since = "2.2.0", note = "Use `solana-quic-definitions` crate instead")]
pub use solana_quic_definitions as quic;
#[deprecated(since = "2.2.0", note = "Use `solana-rent-debits` crate instead")]
pub use solana_rent_debits as rent_debits;
#[cfg(feature = "full")]
#[deprecated(
    since = "2.2.0",
    note = "Use `solana-reserved-account-keys` crate instead"
)]
pub use solana_reserved_account_keys as reserved_account_keys;
#[deprecated(since = "2.1.0", note = "Use `solana-sanitize` crate instead")]
pub use solana_sanitize as sanitize;
/// Same as `declare_id` except report that this id has been deprecated.
pub use solana_sdk_macro::declare_deprecated_id;
/// Convenience macro to declare a static public key and functions to interact with it.
///
/// Input: a single literal base58 string representation of a program's id
///
/// # Example
///
/// ```
/// # // wrapper is used so that the macro invocation occurs in the item position
/// # // rather than in the statement position which isn't allowed.
/// use std::str::FromStr;
/// use solana_sdk::{declare_id, pubkey::Pubkey};
///
/// # mod item_wrapper {
/// #   use solana_sdk::declare_id;
/// declare_id!("My11111111111111111111111111111111111111111");
/// # }
/// # use item_wrapper::id;
///
/// let my_id = Pubkey::from_str("My11111111111111111111111111111111111111111").unwrap();
/// assert_eq!(id(), my_id);
/// ```
pub use solana_sdk_macro::declare_id;
/// Convenience macro to define multiple static public keys.
pub use solana_sdk_macro::pubkeys;
#[deprecated(since = "2.2.0", note = "Use `solana-secp256k1-program` crate instead")]
#[cfg(feature = "full")]
pub use solana_secp256k1_program as secp256k1_instruction;
#[deprecated(since = "2.1.0", note = "Use `solana-secp256k1-recover` crate instead")]
pub use solana_secp256k1_recover as secp256k1_recover;
#[deprecated(since = "2.2.0", note = "Use `solana-serde` crate instead")]
pub use solana_serde as deserialize_utils;
#[deprecated(since = "2.1.0", note = "Use `solana-serde-varint` crate instead")]
pub use solana_serde_varint as serde_varint;
#[deprecated(since = "2.1.0", note = "Use `solana-short-vec` crate instead")]
pub use solana_short_vec as short_vec;

#[deprecated(since = "2.2.0", note = "Use `solana-time-utils` crate instead")]
pub use solana_time_utils as timing;
#[cfg(feature = "full")]
#[deprecated(
    since = "2.2.0",
    note = "Use `solana_transaction::simple_vote_transaction_checker` instead"
)]
pub use solana_transaction::simple_vote_transaction_checker;
#[deprecated(
    since = "2.2.0",
    note = "Use `solana-transaction-context` crate instead"
)]
pub use solana_transaction_context as transaction_context;
#[deprecated(since = "2.2.0", note = "Use `solana-validator-exit` crate instead")]
pub use solana_validator_exit as exit;

/// Convenience macro for `AddAssign` with saturating arithmetic.
/// Replace by `std::num::Saturating` once stable
#[macro_export]
macro_rules! saturating_add_assign {
    ($i:expr, $v:expr) => {{
        $i = $i.saturating_add($v)
    }};
}

pub extern crate bs58;

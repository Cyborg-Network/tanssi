use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::ConstU32, sp_runtime::RuntimeDebug, BoundedVec};
use scale_info::TypeInfo;
use sp_core::hash::H256;

pub type TaskId = u64;

#[derive(PartialEq, Eq, Clone, Decode, Encode, TypeInfo, Debug, MaxEncodedLen)]
pub enum TaskStatusType {
	Assigned,
	PendingValidation,
	Completed,
	Expired,
}

#[derive(PartialEq, Eq, Clone, Decode, Encode, TypeInfo, Debug, MaxEncodedLen)]
pub enum TaskType {
	Docker,
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct TaskInfo<AccountId, BlockNumber> {
	pub task_owner: AccountId,
	pub create_block: BlockNumber,
	pub metadata: BoundedVec<u8, ConstU32<500>>,
	pub zk_files_cid: BoundedVec<u8, ConstU32<500>>,
	pub time_elapsed: Option<BlockNumber>,
	pub average_cpu_percentage_use: Option<u8>,
	pub task_type: TaskType,
	pub result: Option<BoundedVec<u8, ConstU32<500>>>,
	pub compute_hours_deposit: Option<u32>,
	pub consume_compute_hours: Option<u32>,
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct VerificationHashes<AccountId> {
	pub account: AccountId,
	pub completed_hash: Option<H256>,
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct Verifications<AccountId> {
	pub executor: VerificationHashes<AccountId>,
	pub verifier: Option<VerificationHashes<AccountId>>,
	pub resolver: Option<VerificationHashes<AccountId>>,
}

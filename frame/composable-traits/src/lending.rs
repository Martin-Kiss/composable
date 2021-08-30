use codec::Codec;
use frame_support::{pallet_prelude::*, sp_runtime::{FixedI128, Permill, Perquintill}, sp_std::{fmt::Debug, vec::Vec}};



use crate::vault::Deposit;

#[derive(Clone, Encode, Decode, Default, Debug)]
pub struct AccountConfig<AccountId, AssetId>
where
	AccountId: core::cmp::Ord,
{
	pub deposit: AssetId,
	pub collateral: AssetId,
	/// can pause borrow & deposits of assets
	pub pause_guardian: AccountId,
	pub reserve_factor: Permill,
	pub collateral_factor: Permill,
	//pub liquidation_fee: Permill,
}

pub trait Composable {
	type Error;
	type Balance;
	type BlockNumber;
	type AccountId: core::cmp::Ord;
}


// ASK: not clear how Vault will prevent withdrawing collateral?
/// Basic lending with no its own wrapper (liquidity) token.
///  User will deposit borrow and collateral assets via `Vault`.
/// `Liquidation` is other trait.
/// Based on Blacksmith (Warp v2) IBSLendingPair.sol and Parallel Finance.
pub trait Lending: Composable {
	type AssetId;
	type VaultId: Clone + Codec + Debug + PartialEq;
	/// (deposit AssetId, collateral AssetId, VaultId) <-> PairId
	type PairId: Self::AccountId;

	/// creates market for new pair in specified vault
	fn create(
		vault: Self::VaultId,
		fee_withdrawal: Self::AccountId, // The account to withdraw fees to
		deposit: Deposit<Self::Balance, Self::BlockNumber>,
		config: AccountConfig<Self::AccountId, Self::AssetId>,
	) -> Result<Self::PairId, Self::Error>;

	fn get_pair_in_vault(vault: Self::VaultId) -> Result<Vec<Self::PairId>, Self::Error>;

	fn get_pairs_all() -> Result<Vec<Self::PairId>, Self::Error>;

	fn borrow(
		pair: Self::PairId,
		debt_owner: &Self::AccountId,
		amount_to_borrow: Self::Balance,
	) -> Result<(), Self::Error>;

	/// `from` repays some of `beneficiary` debts.
	///
	/// - `pair`        : the pair to be repaid.
	/// - `repay_amount`: the amount to be repaid.
	fn repay_borrow(
		pair: Self::PairId,
		from: &Self::AccountId,
		beneficiary: &Self::AccountId,
		repay_amount: Self::Balance,
	) -> Result<(), Self::Error>;

	/// part or whole of deposited assets and interest into account
	fn redeem(
		pair: Self::PairId,
		to: &Self::AccountId,
		redeem_amount: Self::Balance,
	) -> Result<(), Self::Error>;

	fn calculate_liquidation_fee(amount: Self::Balance) -> Self::Balance;

	fn total_borrows(pair: Self::PairId) -> Result<Self::Balance, Self::Error>;

	fn accrue_interest(pair: Self::PairId) -> Result<(), Self::Error>;

	fn borrow_balance_current(
		pair: Self::PairId,
		account: &Self::AccountId,
	) -> Result<Self::Balance, Self::Error>;

	fn withdraw_fees(to_withdraw: Self::Balance) -> Result<(), Self::Error>;

	fn collateral_of_account(
		pair: Self::PairId,
		account: &Self::AccountId,
	) -> Result<Self::Balance, Self::Error>;

	/// Borrower shouldn't borrow more than his total collateral value
	fn collateral_required(
		pair: Self::PairId,
		borrow_amount: Self::Balance,
	) -> Result<Self::Balance, Self::Error>;

	fn get_borrow_limit(
		pair: Self::PairId,
		account: Self::AccountId,
	) -> Result<Self::Balance, Self::Error>;
}

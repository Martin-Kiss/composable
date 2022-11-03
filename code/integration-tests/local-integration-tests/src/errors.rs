use crate::{
	helpers::*,
	kusama_test_net::{KusamaRelay, This, SIBLING_PARA_ID, THIS_PARA_ID},
	prelude::*,
};

use common::Balance;
use composable_traits::currency::{
	AssetDataMutate, AssetExistentialDepositInspect, AssetRatioInspect,
};
use orml_traits::MultiCurrency;
use xcm::VersionedMultiAsset;

/// under ED, but above Weight
pub fn under_existential_deposit<
	AssetsRegistry: AssetRatioInspect<AssetId = CurrencyId>
		+ AssetExistentialDepositInspect<AssetId = CurrencyId, Balance = Balance>,
>(
	asset_id: LocalAssetId,
	_instruction_count: usize,
) -> Balance {
	let ed = multi_existential_deposits::<AssetsRegistry>(&asset_id);
	assert_gt!(ed, Balance::one());
	ed - Balance::one()
}

#[test]
fn transfer_native_from_relay_enough_for_fee_but_not_enough_for_ed_ends_up_in_treasury() {
	simtest();
	let receiver = charlie();
	let (picasso_treasury, under_ed) = This::execute_with(|| {
		use this_runtime::*;
		let under_ed = under_existential_deposit::<AssetsRegistry>(LocalAssetId::KSM, 3);
		assert_eq!(Tokens::free_balance(CurrencyId::KSM, &AccountId::from(receiver)), 0,);
		(Tokens::free_balance(CurrencyId::KSM, &this_runtime::TreasuryAccount::get()), under_ed)
	});

	KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		let _ = <Balances as frame_support::traits::fungible::Balanced<AccountId>>::deposit(
			&AccountId::from(alice()),
			under_ed * 10000,
		)
		.unwrap();
		assert_ok!(XcmPallet::reserve_transfer_assets(
			Origin::signed(alice().into()),
			Box::new(Parachain(THIS_PARA_ID).into().into()),
			Box::new(Junction::AccountId32 { id: receiver, network: NetworkId::Any }.into().into()),
			Box::new((Here, under_ed).into()),
			0
		));
	});

	This::execute_with(|| {
		use this_runtime::*;
		assert_eq!(
			Tokens::free_balance(CurrencyId::KSM, &AccountId::from(receiver)),
			0,
			"assets did not get to recipient as it is not enough to pay ED"
		);
		assert_eq!(
			Tokens::free_balance(CurrencyId::KSM, &TreasuryAccount::get()),
			under_ed - picasso_treasury
		);
	});
}

#[test]
fn transfer_relay_native_to_non_existing_chain_by_local_id() {
	simtest();
	let transfer_amount = 3 * RELAY_NATIVE::ONE;
	let limit = 4_600_000_000;

	mint_relay_native_on_parachain(transfer_amount * 2, &AccountId::from(alice()), THIS_PARA_ID);

	This::execute_with(|| {
		use this_runtime::*;
		let before = Assets::free_balance(CurrencyId::KSM, &alice().into());
		let transferred = XTokens::transfer(
			Origin::signed(alice().into()),
			CurrencyId::KSM,
			transfer_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(100500),
						Junction::AccountId32 { id: bob(), network: NetworkId::Any },
					),
				)
				.into(),
			),
			limit,
		);

		assert_ok!(transferred);
	});
}

#[test]
fn transfer_non_existing_asset_by_local_id() {
	simtest();
	let transfer_amount = 3 * RELAY_NATIVE::ONE;
	let limit = 4_600_000_000;

	mint_relay_native_on_parachain(transfer_amount * 2, &AccountId::from(alice()), THIS_PARA_ID);

	This::execute_with(|| {
		use this_runtime::*;
		let before = Assets::free_balance(CurrencyId::KSM, &alice().into());
		let transferred = XTokens::transfer(
			Origin::signed(alice().into()),
			CurrencyId(100500),
			transfer_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(SIBLING_PARA_ID),
						Junction::AccountId32 { id: bob(), network: NetworkId::Any },
					),
				)
				.into(),
			),
			limit,
		);

		assert_ok!(transferred);
	});
}

#[test]
fn transfer_existing_asset_but_with_relevant_outgoing_fee_by_local_id() {
	simtest();
	let transfer_amount = 3 * RELAY_NATIVE::ONE;
	let limit = 4_600_000_000;

	mint_relay_native_on_parachain(transfer_amount * 2, &AccountId::from(alice()), THIS_PARA_ID);

	This::execute_with(|| {
		use this_runtime::*;

		AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId(100500),
			XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(THIS_PARA_ID), GeneralIndex(100500)),
			)),
			Some(Rational64::one()),
			None,
		)
		.unwrap();
		<CurrencyFactory as AssetDataMutate>::update_existential_deposit(
			CurrencyId(100500),
			Some(42),
		);
		assert_ok!(Tokens::deposit(CurrencyId(100500), &alice().into(), 2 * transfer_amount));

		AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId(123_666),
			XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(THIS_PARA_ID), GeneralIndex(123_666)),
			)),
			Some(Rational64::one()),
			None,
		)
		.unwrap();
		<CurrencyFactory as AssetDataMutate>::update_existential_deposit(
			CurrencyId(123_666),
			Some(42),
		);
		assert_ok!(Tokens::deposit(CurrencyId(123_666), &alice().into(), 2 * transfer_amount));

		let before = Assets::free_balance(CurrencyId::KSM, &alice().into());
		let transferred = XTokens::transfer_multiasset_with_fee(
			Origin::signed(alice().into()),
			Box::new(VersionedMultiAsset::V1(MultiAsset {
				fun: Fungible(transfer_amount),
				id: AssetId::Concrete(MultiLocation::new(
					1,
					X2(Parachain(THIS_PARA_ID), GeneralIndex(100500)),
				)),
			})),
			Box::new(VersionedMultiAsset::V1(MultiAsset {
				fun: Fungible(transfer_amount),
				id: AssetId::Concrete(MultiLocation::new(
					1,
					X2(Parachain(SIBLING_PARA_ID), GeneralIndex(123_666)),
				)),
			})),
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(SIBLING_PARA_ID),
						Junction::AccountId32 { id: bob(), network: NetworkId::Any },
					),
				)
				.into(),
			),
			limit,
		);
		// let transferred = XTokens::transfer_multiasset_with_fee(
		// 	Origin::signed(alice().into()),
		// 	vec![(CurrencyId(100500),transfer_amount),(CurrencyId(123_666),transfer_amount)],
		// 	0,
		// 	Box::new(
		// 		MultiLocation::new(
		// 			1,
		// 			X2(
		// 				Parachain(SIBLING_PARA_ID),
		// 				Junction::AccountId32 { id: bob(), network: NetworkId::Any },
		// 			),
		// 		)
		// 		.into(),
		// 	),
		// 	limit,
		// );

		assert_ok!(transferred);
	});
}

#[test]
fn transfer_ssexisting_asset_but_with_relevant_outgoing_fee_by_local_12id() {
	simtest();
	let transfer_amount = 3 * RELAY_NATIVE::ONE;
	let limit = 4_600_000_000;

	mint_relay_native_on_parachain(transfer_amount * 2, &AccountId::from(alice()), THIS_PARA_ID);

	This::execute_with(|| {
		use this_runtime::*;

		AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId(100500),
			XcmAssetLocation(MultiLocation::new(
				1,
				X2(Parachain(THIS_PARA_ID), GeneralIndex(100500)),
			)),
			Some(Rational64::one()),
			None,
		)
		.unwrap();
		<CurrencyFactory as AssetDataMutate>::update_existential_deposit(
			CurrencyId(100500),
			Some(42),
		);

		assert_ok!(Tokens::deposit(CurrencyId(100500), &alice().into(), 2 * transfer_amount));

		let before = Assets::free_balance(CurrencyId::KSM, &alice().into());
		let transferred = XTokens::transfer(
			Origin::signed(alice().into()),
			CurrencyId(100500),
			transfer_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(SIBLING_PARA_ID),
						Junction::AccountId32 { id: bob(), network: NetworkId::Any },
					),
				)
				.into(),
			),
			limit,
		);

		assert_ok!(transferred);
	});
}

#[test]
fn cannot_send_message_to_self() {
	simtest();
	let transfer_amount = 3 * RELAY_NATIVE::ONE;
	let limit = 4_600_000_000;

	mint_relay_native_on_parachain(transfer_amount * 2, &AccountId::from(alice()), THIS_PARA_ID);

	This::execute_with(|| {
		use this_runtime::*;

		AssetsRegistry::update_asset(
			RawOrigin::Root.into(),
			CurrencyId(100500),
			XcmAssetLocation(MultiLocation::new(
				0,
				X2(Parachain(THIS_PARA_ID), GeneralIndex(100500)),
			)),
			Some(Rational64::one()),
			None,
		)
		.unwrap();
		<CurrencyFactory as AssetDataMutate>::update_existential_deposit(
			CurrencyId(100500),
			Some(42),
		);

		assert_ok!(Tokens::deposit(CurrencyId(100500), &alice().into(), 2 * transfer_amount));

		let before = Assets::free_balance(CurrencyId::KSM, &alice().into());
		let transferred = XTokens::transfer(
			Origin::signed(alice().into()),
			CurrencyId(100500),
			transfer_amount,
			Box::new(
				MultiLocation::new(
					1,
					X2(
						Parachain(SIBLING_PARA_ID),
						Junction::AccountId32 { id: bob(), network: NetworkId::Any },
					),
				)
				.into(),
			),
			limit,
		);

		assert_ok!(transferred);
	});
}

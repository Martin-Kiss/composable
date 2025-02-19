#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod abstraction;
mod asset;
mod bridge;
#[cfg(feature = "cosmwasm")]
pub mod cosmwasm;
mod instruction;
mod network;
mod packet;
mod program;
mod protocol;

pub use crate::{
	asset::*, bridge::*, instruction::*, network::*, packet::*, program::*, protocol::*,
};
use alloc::{collections::VecDeque, vec::Vec};
use core::marker::PhantomData;

/// Strongly typed network builder originating on `CurrentNetwork` network.
#[derive(Clone)]
pub struct ProgramBuilder<CurrentNetwork, Account, Assets> {
	pub tag: Vec<u8>,
	pub instructions: VecDeque<Instruction<NetworkId, Vec<u8>, Account, Assets>>,
	pub _marker: PhantomData<CurrentNetwork>,
}

impl<CurrentNetwork, Account, Assets> ProgramBuilder<CurrentNetwork, Account, Assets>
where
	CurrentNetwork: Network,
	CurrentNetwork::EncodedCall: Into<Vec<u8>>,
{
	pub fn new(tag: impl Into<Vec<u8>>) -> Self {
		ProgramBuilder { tag: tag.into(), instructions: VecDeque::new(), _marker: PhantomData }
	}

	pub fn transfer(
		mut self,
		to: impl Into<Destination<Account>>,
		assets: impl Into<Assets>,
	) -> Self {
		self.instructions
			.push_back(Instruction::Transfer { to: to.into(), assets: assets.into() });
		self
	}

	pub fn spawn<SpawningNetwork, E, FinalNetwork, F>(
		self,
		tag: impl Into<Vec<u8>>,
		salt: impl Into<Vec<u8>>,
		bridge_security: impl Into<BridgeSecurity>,
		assets: impl Into<Assets>,
		f: F,
	) -> Result<ProgramBuilder<FinalNetwork, Account, Assets>, E>
	where
		F: FnOnce(
			ProgramBuilder<SpawningNetwork, Account, Assets>,
		) -> Result<ProgramBuilder<FinalNetwork, Account, Assets>, E>,
		SpawningNetwork: Network,
		SpawningNetwork::EncodedCall: Into<Vec<u8>>,
		FinalNetwork: Network,
		FinalNetwork::EncodedCall: Into<Vec<u8>>,
	{
		// We need to recreate the builder to mutate the phantom marker.
		let mut builder =
			ProgramBuilder { tag: self.tag, instructions: self.instructions, _marker: PhantomData };
		builder.instructions.push_back(Instruction::Spawn {
			bridge_security: bridge_security.into(),
			salt: salt.into(),
			assets: assets.into(),
			network: SpawningNetwork::ID,
			program: f(ProgramBuilder::<SpawningNetwork, Account, Assets>::new(tag.into()))?
				.build(),
		});
		Ok(builder)
	}

	pub fn call_raw(mut self, encoded: CurrentNetwork::EncodedCall) -> Self {
		self.instructions
			.push_back(Instruction::Call { bindings: Vec::new(), encoded: encoded.into() });
		self
	}

	pub fn call<T>(self, protocol: T) -> Result<Self, T::Error>
	where
		T: Protocol<CurrentNetwork>,
	{
		protocol.serialize().map(|encoded_call| self.call_raw(encoded_call))
	}

	pub fn build(self) -> Program<VecDeque<Instruction<NetworkId, Vec<u8>, Account, Assets>>> {
		Program { tag: self.tag, instructions: self.instructions }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::vec;

	struct DummyProtocol1;
	#[derive(Debug)]
	struct DummyProtocol1Error;
	impl Protocol<Picasso> for DummyProtocol1 {
		type Error = DummyProtocol1Error;
		fn serialize(&self) -> Result<<Picasso as Network>::EncodedCall, Self::Error> {
			Ok(vec![0xCA, 0xFE, 0xBE, 0xEF])
		}
	}
	impl Protocol<Ethereum> for DummyProtocol1 {
		type Error = DummyProtocol1Error;
		fn serialize(&self) -> Result<<Ethereum as Network>::EncodedCall, Self::Error> {
			Ok(vec![0xC0, 0xDE, 0xC0, 0xDE])
		}
	}

	struct DummyProtocol2;
	#[derive(Debug)]
	struct DummyProtocol2Error;
	impl Protocol<Picasso> for DummyProtocol2 {
		type Error = DummyProtocol2Error;
		fn serialize(&self) -> Result<<Picasso as Network>::EncodedCall, Self::Error> {
			Ok(vec![0xCA, 0xFE, 0xBA, 0xBE])
		}
	}
	impl Protocol<Ethereum> for DummyProtocol2 {
		type Error = DummyProtocol2Error;
		fn serialize(&self) -> Result<<Ethereum as Network>::EncodedCall, Self::Error> {
			Ok(vec![0xDE, 0xAD, 0xC0, 0xDE])
		}
	}

	#[derive(Debug)]
	enum ProgramBuildError {
		DummyProtocol1(DummyProtocol1Error),
		DummyProtocol2(DummyProtocol2Error),
	}
	impl From<DummyProtocol1Error> for ProgramBuildError {
		fn from(x: DummyProtocol1Error) -> Self {
			ProgramBuildError::DummyProtocol1(x)
		}
	}
	impl From<DummyProtocol2Error> for ProgramBuildError {
		fn from(x: DummyProtocol2Error) -> Self {
			ProgramBuildError::DummyProtocol2(x)
		}
	}

	#[test]
	fn can_build() {
		let program = || -> Result<_, ProgramBuildError> {
			Ok(ProgramBuilder::<Picasso, (), Funds>::new("Main program".as_bytes().to_vec())
				.call(DummyProtocol1)?
				.spawn::<Ethereum, ProgramBuildError, _, _>(
					Vec::default(),
					Vec::default(),
					BridgeSecurity::Deterministic,
					Funds::empty(),
					|child| {
						Ok(child
							.call(DummyProtocol2)?
							.call(DummyProtocol1)?
							.transfer(Destination::Relayer, Funds::from([(PICA::ID, u128::MAX)])))
					},
				)?
				.build())
		}()
		.expect("valid program");

		assert_eq!(
			program,
			Program {
				tag: "Main program".as_bytes().to_vec(),
				instructions: VecDeque::from([
					// Protocol 1 on picasso
					Instruction::Call { bindings: vec![], encoded: vec![202, 254, 190, 239] },
					// Move to ethereum
					Instruction::Spawn {
						network: Ethereum::ID,
						bridge_security: BridgeSecurity::Deterministic,
						salt: Vec::new(),
						assets: Funds::empty(),
						program: Program {
							tag: Default::default(),
							instructions: VecDeque::from([
								// Protocol 2 on eth
								Instruction::Call {
									bindings: vec![],
									encoded: vec![222, 173, 192, 222]
								},
								// Protocol 1 on eth, different encoding than on previous network
								Instruction::Call {
									bindings: vec![],
									encoded: vec![192, 222, 192, 222]
								},
								Instruction::Transfer {
									to: Destination::Relayer,
									assets: Funds::from(vec![(PICA::ID, u128::MAX)])
								}
							])
						}
					}
				])
			},
		);
	}
}

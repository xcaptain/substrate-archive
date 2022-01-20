// Copyright 2017-2021 Parity Technologies (UK) Ltd.
// This file is part of substrate-archive.

// substrate-archive is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// substrate-archive is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with substrate-archive.  If not, see <http://www.gnu.org/licenses/>.

//! This module implements traits that are not used in the Archive Node
//!
//! Most will return None, Err, or panic in worst-case scenario
//! They should never be called under normal circumstances

use std::marker::PhantomData;

use sc_client_api::backend::{AuxStore, BlockImportOperation, NewBlockState, TransactionForSB};
use sp_blockchain::{well_known_cache_keys::Id, Error as BlockchainError};
use sp_core::offchain::OffchainStorage;
use sp_runtime::{generic::BlockId, traits::Block as BlockT, Justification, Justifications, StateVersion};
use sp_state_machine::{ChildStorageCollection, IndexOperation, StorageCollection};
use sp_storage::Storage;

use crate::{database::ReadOnlyDb, read_only_backend::ReadOnlyBackend, util::columns};

type ChainResult<T> = Result<T, BlockchainError>;

//// Dummy Block Import Operation struct
pub struct RealBlockImportOperation<D> {
	pub _marker: PhantomData<D>,
}

impl<Block: BlockT, D: ReadOnlyDb> BlockImportOperation<Block> for RealBlockImportOperation<D> {
	type State = super::state_backend::TrieState<Block, D>;

	fn state(&self) -> ChainResult<Option<&Self::State>> {
		log::warn!("Block import operations not supported.");
		Ok(None)
	}

	fn set_block_data(
		&mut self,
		_header: Block::Header,
		_body: Option<Vec<Block::Extrinsic>>,
		_indexed_body: Option<Vec<Vec<u8>>>,
		_justification: Option<Justifications>,
		_state: NewBlockState,
	) -> ChainResult<()> {
		log::warn!("Block state may not be set with a Read Only Backend");
		Ok(())
	}

	fn update_cache(&mut self, _cache: std::collections::HashMap<Id, Vec<u8>>) {
		// TODO: maybe we should have a cache??
		log::warn!("No cache on a read only backend");
	}

	fn update_db_storage(&mut self, _update: TransactionForSB<Self::State, Block>) -> ChainResult<()> {
		log::warn!("Cannot modify storage of a read only backend. Storage not updated");
		Ok(())
	}

	fn set_genesis_state(
		&mut self,
		_storage: Storage,
		_commit: bool,
		_state_version: StateVersion,
	) -> sp_blockchain::Result<Block::Hash> {
		log::warn!("Cannot set state of a read only backend. Genesis not set");
		Ok(Default::default())
	}

	fn reset_storage(&mut self, _reset: Storage, _state_version: StateVersion) -> ChainResult<Block::Hash> {
		log::warn!("Cannot modify storage of a read only backend. Storage not reset.");
		Ok(Default::default())
	}

	fn update_storage(&mut self, _update: StorageCollection, _child_update: ChildStorageCollection) -> ChainResult<()> {
		log::warn!("Cannot modify storage of a read only backend. Storage not updated.");
		Ok(())
	}

	fn insert_aux<I>(&mut self, _ops: I) -> ChainResult<()>
	where
		I: IntoIterator<Item = (Vec<u8>, Option<Vec<u8>>)>,
	{
		log::warn!("Cannot modify storage of a read only backend. Aux not inserted.");
		Ok(())
	}

	fn mark_finalized(&mut self, _id: BlockId<Block>, _justification: Option<Justification>) -> ChainResult<()> {
		log::warn!("Cannot modify storage of a read only backend. finalized not marked.");
		Ok(())
	}

	fn mark_head(&mut self, _id: BlockId<Block>) -> ChainResult<()> {
		log::warn!("Cannot modify storage of a read only backend. Head not marked.");
		Ok(())
	}

	fn update_transaction_index(&mut self, _: Vec<IndexOperation>) -> sp_blockchain::Result<()> {
		log::warn!("Tried updating transaction index; Block import operations not supported on Read Only backend");
		Ok(())
	}
}

#[derive(Debug, Clone)]
pub struct OffchainStorageBackend;

impl OffchainStorage for OffchainStorageBackend {
	fn set(&mut self, _prefix: &[u8], _key: &[u8], _value: &[u8]) {
		log::warn!("Cannot modify storage of a read only backend. Offchain Storage not set.");
	}

	fn remove(&mut self, _prefix: &[u8], _key: &[u8]) {
		log::warn!("Cannot modify storage of a read only backend. Offchain Storage not set.");
	}

	fn get(&self, _prefix: &[u8], _key: &[u8]) -> Option<Vec<u8>> {
		log::warn!("Offchain Storage operations not supported");
		None
	}

	fn compare_and_set(&mut self, _prefix: &[u8], _key: &[u8], _old_value: Option<&[u8]>, _new_value: &[u8]) -> bool {
		log::warn!("Cannot modify storage of a read only backend. Offchain storage not set");
		false
	}
}

impl<Block: BlockT, DB: ReadOnlyDb> AuxStore for ReadOnlyBackend<Block, DB> {
	fn insert_aux<
		'a,
		'b: 'a,
		'c: 'a,
		I: IntoIterator<Item = &'a (&'c [u8], &'c [u8])>,
		D: IntoIterator<Item = &'a &'b [u8]>,
	>(
		&self,
		_insert: I,
		_delete: D,
	) -> ChainResult<()> {
		log::warn!("Insert operations not supported for Read Only Backend");
		Ok(())
	}

	fn get_aux(&self, key: &[u8]) -> ChainResult<Option<Vec<u8>>> {
		Ok(self.db.get(columns::AUX, key))
	}
}

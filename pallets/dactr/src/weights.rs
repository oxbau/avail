// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for da_control
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-01, STEPS: `50`, REPEAT: 25, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-5-24`, CPU: `Intel(R) Xeon(R) Platinum 8175M CPU @ 2.50GHz`
//! EXECUTION: Some(Native), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/data-avail
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=25
// --log=warn
// --execution=native
// --template=./.maintain/frame-weight-template.hbs
// --header=./HEADER-APACHE2
// --pallet=da-control
// --extrinsic=*
// --output=./pallets/dactr/src/weights.rs
// -linfo

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for da_control.
pub trait WeightInfo {
	fn create_application_key() -> Weight;
	fn submit_block_length_proposal() -> Weight;
	fn submit_data(i: u32, ) -> Weight;
	fn data_root(i: u32, ) -> Weight;
	fn data_root_batch(i: u32, ) -> Weight;
	fn commitment_builder_32(i: u32, ) -> Weight;
	fn commitment_builder_64(i: u32, ) -> Weight;
	fn commitment_builder_128(i: u32, ) -> Weight;
	fn commitment_builder_256(i: u32, ) -> Weight;
}

/// Weights for da_control using the Data Avaiability node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: DataAvailability AppKeys (r:1 w:1)
	// Storage: DataAvailability NextAppId (r:1 w:1)
	fn create_application_key() -> Weight {
		// Minimum execution time: 26_706 nanoseconds.
		Weight::from_parts(27_653_000, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: System DynamicBlockLength (r:1 w:1)
	fn submit_block_length_proposal() -> Weight {
		// Minimum execution time: 22_841 nanoseconds.
		Weight::from_parts(23_486_000, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// The range of component `i` is `[1, 524288]`.
	fn submit_data(i: u32, ) -> Weight {
		// Minimum execution time: 17_107 nanoseconds.
		Weight::from_parts(9_816_023, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(287, 0).saturating_mul(i as u64))
	}
	/// The range of component `i` is `[0, 524288]`.
	fn data_root(i: u32, ) -> Weight {
		// Minimum execution time: 1_457 nanoseconds.
		Weight::from_parts(1_487_000, 0)
			// Standard Error: 21
			.saturating_add(Weight::from_parts(5_251, 0).saturating_mul(i as u64))
	}
	/// The range of component `i` is `[0, 2097152]`.
	fn data_root_batch(i: u32, ) -> Weight {
		// Minimum execution time: 778 nanoseconds.
		Weight::from_parts(825_000, 0)
			// Standard Error: 5
			.saturating_add(Weight::from_parts(4_931, 0).saturating_mul(i as u64))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_32(i: u32, ) -> Weight {
		// Minimum execution time: 1_674_133 nanoseconds.
		Weight::from_parts(1_782_369_000, 0)
			// Standard Error: 33_902_431
			.saturating_add(Weight::from_parts(214_276_156, 0).saturating_mul(i as u64))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_64(i: u32, ) -> Weight {
		// Minimum execution time: 1_669_019 nanoseconds.
		Weight::from_parts(1_742_205_000, 0)
			// Standard Error: 46_184_424
			.saturating_add(Weight::from_parts(2_679_720_641, 0).saturating_mul(i as u64))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_128(i: u32, ) -> Weight {
		// Minimum execution time: 1_843_601 nanoseconds.
		Weight::from_parts(1_942_890_000, 0)
			// Standard Error: 43_871_963
			.saturating_add(Weight::from_parts(4_222_032_369, 0).saturating_mul(i as u64))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_256(i: u32, ) -> Weight {
		// Minimum execution time: 1_864_952 nanoseconds.
		Weight::from_parts(459_958_961_086, 0)
			// Standard Error: 102_554_927
			.saturating_add(Weight::from_parts(5_707_004_205, 0).saturating_mul(i as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: DataAvailability AppKeys (r:1 w:1)
	// Storage: DataAvailability NextAppId (r:1 w:1)
	fn create_application_key() -> Weight {
		// Minimum execution time: 26_706 nanoseconds.
		Weight::from_parts(27_653_000, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: System DynamicBlockLength (r:1 w:1)
	fn submit_block_length_proposal() -> Weight {
		// Minimum execution time: 22_841 nanoseconds.
		Weight::from_parts(23_486_000, 0)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// The range of component `i` is `[1, 524288]`.
	fn submit_data(i: u32, ) -> Weight {
		// Minimum execution time: 17_107 nanoseconds.
		Weight::from_parts(9_816_023, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(287, 0).saturating_mul(i as u64))
	}
	/// The range of component `i` is `[0, 524288]`.
	fn data_root(i: u32, ) -> Weight {
		// Minimum execution time: 1_457 nanoseconds.
		Weight::from_parts(1_487_000, 0)
			// Standard Error: 21
			.saturating_add(Weight::from_parts(5_251, 0).saturating_mul(i as u64))
	}
	/// The range of component `i` is `[0, 2097152]`.
	fn data_root_batch(i: u32, ) -> Weight {
		// Minimum execution time: 778 nanoseconds.
		Weight::from_parts(825_000, 0)
			// Standard Error: 5
			.saturating_add(Weight::from_parts(4_931, 0).saturating_mul(i as u64))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_32(i: u32, ) -> Weight {
		// Minimum execution time: 1_674_133 nanoseconds.
		Weight::from_parts(1_782_369_000, 0)
			// Standard Error: 33_902_431
			.saturating_add(Weight::from_parts(214_276_156, 0).saturating_mul(i as u64))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_64(i: u32, ) -> Weight {
		// Minimum execution time: 1_669_019 nanoseconds.
		Weight::from_parts(1_742_205_000, 0)
			// Standard Error: 46_184_424
			.saturating_add(Weight::from_parts(2_679_720_641, 0).saturating_mul(i as u64))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_128(i: u32, ) -> Weight {
		// Minimum execution time: 1_843_601 nanoseconds.
		Weight::from_parts(1_942_890_000, 0)
			// Standard Error: 43_871_963
			.saturating_add(Weight::from_parts(4_222_032_369, 0).saturating_mul(i as u64))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_256(i: u32, ) -> Weight {
		// Minimum execution time: 1_864_952 nanoseconds.
		Weight::from_parts(459_958_961_086, 0)
			// Standard Error: 102_554_927
			.saturating_add(Weight::from_parts(5_707_004_205, 0).saturating_mul(i as u64))
	}
}

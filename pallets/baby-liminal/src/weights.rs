//! Autogenerated weights for pallet_baby_liminal
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-12, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("chainspec.json"), DB CACHE: 1024

// Executed Command:
// target/release/aleph-node
// benchmark
// pallet
// --chain=chainspec.json
// --pallet=pallet_baby_liminal
// --extrinsic=*
// --steps=20
// --repeat=10
// --template=.maintain/pallet-weight-template.hbs
// --execution=wasm
// --wasm-execution=compiled
// --output=pallets/baby-liminal/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_baby_liminal.
pub trait WeightInfo {
    fn store_key(key_length: u32) -> Weight;
    fn overwrite_equal_key(key_length: u32) -> Weight;
    fn overwrite_key(key_length: u32) -> Weight;
    fn delete_key(key_length: u32) -> Weight;
}

impl<I: BenchmarkInfo> WeightInfo for I {
    fn store_key(key_length: u32) -> Weight {
        <I as BenchmarkInfo>::store_key(key_length)
    }

    fn overwrite_key(key_length: u32) -> Weight {
        <I as BenchmarkInfo>::overwrite_key(key_length)
    }

    fn overwrite_equal_key(key_length: u32) -> Weight {
        <I as BenchmarkInfo>::overwrite_equal_key(key_length)
    }

    fn delete_key(key_length: u32) -> Weight {
        <I as BenchmarkInfo>::delete_key(key_length)
    }
}

/// Benchmark results for pallet_baby_liminal.
trait BenchmarkInfo {
    fn store_key(l: u32, ) -> Weight;
    fn overwrite_equal_key(l: u32, ) -> Weight;
    fn overwrite_key(l: u32, ) -> Weight;
    fn delete_key(l: u32, ) -> Weight;
}

/// Weights for pallet_baby_liminal using the Substrate node and recommended hardware.
pub struct AlephWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> BenchmarkInfo for AlephWeight<T> {
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyOwners (r:0 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:0 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn store_key(l: u32, ) -> Weight {
		// Minimum execution time: 33_270 nanoseconds.
		Weight::from_parts(36_539_015_u64, 0)
			// Standard Error: 56
			.saturating_add(Weight::from_parts(669_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeyOwners (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn overwrite_equal_key(l: u32, ) -> Weight {
		// Minimum execution time: 39_501 nanoseconds.
		Weight::from_parts(44_208_753_u64, 0)
			// Standard Error: 279
			.saturating_add(Weight::from_parts(1_189_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeyOwners (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 9999]`.
	fn overwrite_key(l: u32, ) -> Weight {
		// Minimum execution time: 37_732 nanoseconds.
		Weight::from_parts(43_852_538_u64, 0)
			// Standard Error: 61
			.saturating_add(Weight::from_parts(775_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeyOwners (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeys (r:0 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn delete_key(_l: u32, ) -> Weight {
		// Minimum execution time: 30_814 nanoseconds.
		Weight::from_parts(34_440_507_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl BenchmarkInfo for () {
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyOwners (r:0 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:0 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn store_key(l: u32, ) -> Weight {
		// Minimum execution time: 33_270 nanoseconds.
		Weight::from_parts(36_539_015_u64, 0)
			// Standard Error: 56
			.saturating_add(Weight::from_parts(669_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeyOwners (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn overwrite_equal_key(l: u32, ) -> Weight {
		// Minimum execution time: 39_501 nanoseconds.
		Weight::from_parts(44_208_753_u64, 0)
			// Standard Error: 279
			.saturating_add(Weight::from_parts(1_189_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeyOwners (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 9999]`.
	fn overwrite_key(l: u32, ) -> Weight {
		// Minimum execution time: 37_732 nanoseconds.
		Weight::from_parts(43_852_538_u64, 0)
			// Standard Error: 61
			.saturating_add(Weight::from_parts(775_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeyOwners (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeys (r:0 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn delete_key(_l: u32, ) -> Weight {
		// Minimum execution time: 30_814 nanoseconds.
		Weight::from_parts(34_440_507_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}

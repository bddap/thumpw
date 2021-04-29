#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

const CHUNK_STORAGE_WIEGHT: u64 = 1_000_000;
const EMPTY_CHUNK: [[[u16; 16]; 16]; 16] = [[[0u16; 16]; 16]; 16];

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn chunk)]
    // Learn more about declaring storage items:
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
    pub type Chunks<T> = StorageMap<
        _,
        Blake2_128Concat,
        [i32; 3],
        (
            <T as frame_system::Config>::AccountId,
            [[[u16; 16]; 16]; 16],
        ),
    >;

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// That chunk has not yet been claimed.
        ChunkDoesNotExist,
        /// That chunk does not belong to you.
        NotYours,
        /// You can't claim that chunk because it is already owned by someone.
        AlreadyOwned,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a block to the world. Will fail if you do not own the chunk.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn write_block(
            origin: OriginFor<T>,
            location: [i32; 3],
            block: u16,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let [x, y, z] = location;
            let location_of_chunk = [x / 16, y / 16, z / 16];

            let (owner, mut chunk) =
                <Chunks<T>>::get(location_of_chunk).ok_or(Error::<T>::ChunkDoesNotExist)?;
            ensure!(owner == who, Error::<T>::NotYours);

            let local = crate::world_to_chunk(location);
            chunk[local[0]][local[1]][local[2]] = block;
            <Chunks<T>>::insert(location_of_chunk, (owner, chunk));

            Ok(().into())
        }

        /// Grab some space.
        #[pallet::weight(
            10_000 + T::DbWeight::get().reads_writes(1, 1) + crate::CHUNK_STORAGE_WIEGHT
        )]
        pub fn claim_chunk(origin: OriginFor<T>, location: [i32; 3]) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let [x, y, z] = location;
            let location_of_chunk = [x / 16, y / 16, z / 16];
            ensure!(
                !<Chunks<T>>::contains_key(location_of_chunk),
                Error::<T>::AlreadyOwned
            );

            <Chunks<T>>::insert(location_of_chunk, (who, &crate::EMPTY_CHUNK));

            Ok(().into())
        }

        /// Transfer ownership to a friend.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn give_chunk(
            origin: OriginFor<T>,
            location: [i32; 3],
            recipient: <T as frame_system::Config>::AccountId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let [x, y, z] = location;
            let location_of_chunk = [x / 16, y / 16, z / 16];
            let (owner, chunk) =
                <Chunks<T>>::get(location_of_chunk).ok_or(Error::<T>::ChunkDoesNotExist)?;
            ensure!(owner == who, Error::<T>::NotYours);

            <Chunks<T>>::insert(location_of_chunk, (recipient, chunk));

            Ok(().into())
        }
    }
}

fn world_to_chunk(location: [i32; 3]) -> [usize; 3] {
    let in_chunk = |i: i32| ((i % 16 + 16) % 16) as usize;
    let [x, y, z] = location;
    let ret = [in_chunk(x), in_chunk(y), in_chunk(z)];
    ret
}

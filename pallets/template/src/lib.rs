#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec; // Step 3.1 will include this in `Cargo.toml`

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    // Pallets use events to inform users when important changes are made.
    // Event documentation should end with an array that provides descriptive names for parameters.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a record has been stored.[who,coordinates]
        ProductStored(T::AccountId, Vec<u8>),
       /// Event emitted when coordinates are verified. [onwer]
	   ProductSold(Vec<u8>),
    }
    
    #[pallet::error]
    pub enum Error<T> {
            /// Entered product already exists .
            ProductAlreadyExists,
            /// Entered product doesn't exists.
            NoSuchProduct,
        }
    
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
    
	#[pallet::storage] 
	pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (Vec<u8>,Vec<u8>,Vec<u8>,T::AccountId), ValueQuery>; 

    
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    
    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub(super) fn sell_product(
			origin: OriginFor<T>,
			productid: Vec<u8>,
			productname: Vec<u8>,
			price: Vec<u8>,
			seller: Vec<u8>,
		) -> DispatchResultWithPostInfo {

			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let sender = ensure_signed(origin)?;
		
			// Verify that the specified coordinates doesn't exist in block.         
			ensure!(!Proofs::<T>::contains_key(&productid), Error::<T>::ProductAlreadyExists);

			// Store the proof with the coordinates, (owner and sender).
			Proofs::<T>::insert(&productid, (&productname,&price,&seller,&sender));

			// Emit an event that the land record is stored.
			Self::deposit_event(Event::ProductStored(sender,productid));

			Ok(().into())
		}
	
		#[pallet::weight(10_000)]
		fn buy_product(
			origin: OriginFor<T>,
			productid: Vec<u8>,
			_productname: Vec<u8>
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			ensure_signed(origin)?;

			// Verify that the land record exist in block .
			ensure!(Proofs::<T>::contains_key(&productid), Error::<T>::NoSuchProduct);
			Proofs::<T>::remove(&productid);

			// Emit an event that the land record is verified.
			Self::deposit_event(Event::ProductSold(productid));

			Ok(().into())
		}
		

	}
}


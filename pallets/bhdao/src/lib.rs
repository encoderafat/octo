#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

//#[cfg(feature = "runtime-benchmarks")]
//mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::Currency,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::{
		TypeInfo,
	};
	use sp_runtime::ArithmeticError;
	use sp_std::vec::Vec;


	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

	//type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::type_value]
    pub fn DefaultQualificationVotingWindow<T: Config>() -> u32
    {
        14400u32
    }

	#[pallet::type_value]
    pub fn DefaultVerificationVotingWindow<T: Config>() -> u32
    {
        14400u32
    }

	#[pallet::type_value]
    pub fn DefaultQualificationQuorum<T: Config>() -> u32
    {
        0u32
    }

	#[pallet::type_value]
    pub fn DefaultVerificationQuorum<T: Config>() -> u32
    {
        0u32
    }

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Document<T:Config> {
		pub creator: T::AccountId,
		pub title: Vec<u8>,
		pub description: Vec<u8>,
		pub format: Vec<u8>,
		pub hash: Vec<u8>,
		pub status: DocumentStatus,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Vote<T:Config> {
		pub document_id: u64,
		pub yes_votes: u64,
		pub no_votes: u64,
		pub start: T::BlockNumber,
		pub end: T::BlockNumber,
		pub status: VoteStatus,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Roles {
		QualifierRole = 1,
		CollectorRole = 2,
		ContributorRole = 3,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum VoteStatus {
		InProgress,
		Passed,
		Failed,
		Expired,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum DocumentStatus {
		Submitted,
		UnderReview,
		SuccessfulReview,
		VoteInProgress,
		Verified,
		Rejected,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum VoteType {
		Qualification,
		Verification,
		Proposal,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config +pallet_nft::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_key)]
	pub(super) type Key<T:Config> = StorageValue<_, T::AccountId,OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_total_items)]
	pub(super) type TotalItems<T> = StorageValue<_, u64,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_total_transactions)]
	pub(super) type TotalTransactions<T> = StorageValue<_, u64,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn contributors_uid_count)]
	pub(super) type ContributorsCount<T> = StorageValue<_, u32,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn collectors_uid_count)]
	pub(super) type CollectorsCount<T> = StorageValue<_, u32,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn qualifiers_uid_count)]
	pub(super) type QualifiersCount<T> = StorageValue<_, u32,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_qualification_vote_count)]
	pub(super) type QualificationVotesCount<T> = StorageValue<_, u64,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_verification_vote_count)]
	pub(super) type VerificationVotesCount<T> = StorageValue<_, u64,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_qualification_voting_window)]
	pub(super) type QualificationVotingWindow<T> = StorageValue<_, u32,ValueQuery,DefaultQualificationVotingWindow<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_verification_voting_window)]
	pub(super) type VerificationVotingWindow<T> = StorageValue<_, u32,ValueQuery,DefaultVerificationVotingWindow<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_qualification_quorum)]
	pub(super) type QualificationQuorum<T> = StorageValue<_, u32,ValueQuery,DefaultQualificationQuorum<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_verification_quorum)]
	pub(super) type VerificationQuorum<T> = StorageValue<_, u32,ValueQuery,DefaultVerificationQuorum<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_transactions_per_address)]
	pub(super) type TransactionsPerAddress<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		u64,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_document)]
	pub(super) type Documents<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64,
		Document<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_qualification_vote)]
	pub(super) type QualificationVotes<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64,
		Vote<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_verification_vote)]
	pub(super) type VerificationVotes<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64,
		Vote<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_member_vote)]
	pub(super) type MemberVote<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::AccountId,VoteType,u64),
		bool,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_all_qualifiers)]
	pub(super) type Qualifiers<T:Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_all_collectors)]
	pub(super) type Collectors<T:Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_all_contributors)]
	pub(super) type Contributors<T:Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		QualifierAdded(T::AccountId,u32),
		CollectorAdded(T::AccountId,u32),
		ContributorAdded(T::AccountId,u32),
		DocumentCreated(T::AccountId,u64),
		DocumentStatusUpdated(u64,u8),
		QualificationVotingWindowChanged(u32),
		QualificationVotingStarted(u64),
		VerificationVotingWindowChanged(u32),
		VerificationVotingStarted(u64),
		QualificationVotingEnded(u64),
		VerificationVotingEnded(u64),
		QualificationQuorumChanged(u32),
		VerificationQuorumChanged(u32),
		VoteCast(u8,u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		QualifierAlreadyExists,
		CollectorAlreadyExists,
		ContributorAlreadyExists,
		NotAQualifier,
		NotACollector,
		NotAContributor,
		NotAuthorized,
		DocumentNotFound,
		IncorrectDocumentStatus,
		DocumentTitleNotProvided,
		DocumentDescriptionNotProvided,
		DocumentFormatNotProvided,
		DocumentIPFSHashNotProvided,
		VerificationVoteAlreadyCreated,
		VotingWindowNotValid,
		DocumentNotReviewed,
		VoteNotFound,
		VoteNotInProgress,
		VoteStillInProgress,
		DocumentNotUnderReview,
		MemberAlreadyVoted,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn init_collections(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin.clone())?;
			let max_qualifiers: u32 = 200;
			let max_collectors : u32 = 100;
			let max_contributors : u32 = 1000;

			// create qualifiers collection
			pallet_nft::Pallet::<T>::create_collection(origin.clone(),Roles::QualifierRole as u32,max_qualifiers,b"Qualifiers".to_vec()).ok();

			//create collectors collection
			pallet_nft::Pallet::<T>::create_collection(origin.clone(),Roles::CollectorRole as u32,max_collectors,b"Collectors".to_vec()).ok();

			//create contributors collection
			pallet_nft::Pallet::<T>::create_collection(origin.clone(),Roles::ContributorRole as u32,max_contributors,b"Contributors".to_vec()).ok();

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_qualifier(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;
			let mut qualifiers = Qualifiers::<T>::get();

			let uid = Self::qualifiers_uid_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			match qualifiers.binary_search(&who) {
				Ok(_) => Err(Error::<T>::QualifierAlreadyExists.into()),
				Err(index) => {
					qualifiers.insert(index, who.clone());
					Qualifiers::<T>::put(qualifiers);
					QualifiersCount::<T>::put(uid.clone());
					//mint NFT
					pallet_nft::Pallet::<T>::mint(origin.clone(),Roles::QualifierRole as u32,who.clone()).ok();
					Self::deposit_event(Event::QualifierAdded(who,uid));
					Ok(())
				}
			}

		}

		#[pallet::weight(10_000 )]
		pub fn add_collector(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;
			let mut collectors = Collectors::<T>::get();

			let uid = Self::collectors_uid_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			match collectors.binary_search(&who) {
				Ok(_) => Err(Error::<T>::CollectorAlreadyExists.into()),
				Err(index) => {
					collectors.insert(index, who.clone());
					Collectors::<T>::put(collectors);
					CollectorsCount::<T>::put(uid.clone());
					//mint NFT
					pallet_nft::Pallet::<T>::mint(origin.clone(),Roles::CollectorRole as u32,who.clone()).ok();
					Self::deposit_event(Event::CollectorAdded(who,uid));
					Ok(())
				}
			}

		}

		#[pallet::weight(10_000)]
		pub fn add_contributor(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;
			let mut contributors = Contributors::<T>::get();

			let uid = Self::contributors_uid_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			match contributors.binary_search(&who) {
				Ok(_) => Err(Error::<T>::ContributorAlreadyExists.into()),
				Err(index) => {
					contributors.insert(index, who.clone());
					Contributors::<T>::put(contributors);
					ContributorsCount::<T>::put(uid.clone());
					//mint NFT
					pallet_nft::Pallet::<T>::mint(origin.clone(),Roles::ContributorRole as u32,who.clone()).ok();
					Self::deposit_event(Event::ContributorAdded(who,uid));
					Ok(())
				}
			}

		}


		#[pallet::weight(10_000)]
		pub fn create_document(origin: OriginFor<T>, title: Vec<u8>, description: Vec<u8>,
		format: Vec<u8>, hash: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::ensure_contributor(who.clone()),Error::<T>::NotAContributor);
			ensure!(!title.is_empty(),Error::<T>::DocumentTitleNotProvided);
			ensure!(!description.is_empty(),Error::<T>::DocumentDescriptionNotProvided);
			ensure!(!format.is_empty(),Error::<T>::DocumentFormatNotProvided);
			ensure!(!hash.is_empty(),Error::<T>::DocumentIPFSHashNotProvided);

			let uid = Self::get_total_items().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let document = Document::<T> {
				creator: who.clone(),
				title: title.clone(),
				description: description.clone(),
				format: format.clone(),
				hash: hash.clone(),
				status: DocumentStatus::Submitted,
			};

			Documents::<T>::insert(uid.clone(),document);
			TotalItems::<T>::put(&uid);

			Self::deposit_event(Event::DocumentCreated(who,uid));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn create_qualification_voting(origin: OriginFor<T>, document_id: u64) -> DispatchResult{

			let who = ensure_signed(origin)?;
			ensure!(Self::ensure_qualifier(who.clone()),Error::<T>::NotAQualifier);

			let mut document = Self::get_document(document_id.clone()).ok_or(Error::<T>::DocumentNotFound)?;

			ensure!(document.status == DocumentStatus::Submitted, Error::<T>::VerificationVoteAlreadyCreated);

			let uid = Self::get_qualification_vote_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let now = <frame_system::Pallet<T>>::block_number();

			let end = now + QualificationVotingWindow::<T>::get().into();

			let vote = Vote::<T> {
				document_id: document_id,
				yes_votes: 0,
				no_votes: 0,
				start: now,
				end: end,
				status: VoteStatus::InProgress,
			};

			QualificationVotes::<T>::insert(uid.clone(),&vote);
			QualificationVotesCount::<T>::put(uid.clone());
			Self::deposit_event(Event::QualificationVotingStarted(uid));

			document.status = DocumentStatus::UnderReview;
			Documents::<T>::insert(document_id.clone(),document);
			Self::deposit_event(Event::DocumentStatusUpdated(document_id,1));
			
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn create_verification_voting(origin: OriginFor<T>, document_id: u64) -> DispatchResult{

			let who = ensure_signed(origin)?;
			ensure!(Self::ensure_qualifier(who.clone()) || Self::ensure_contributor(who.clone()),Error::<T>::NotAuthorized);

			let mut document = Self::get_document(document_id.clone()).ok_or(Error::<T>::DocumentNotFound)?;

			ensure!(document.status == DocumentStatus::SuccessfulReview, Error::<T>::DocumentNotReviewed);

			let uid = Self::get_verification_vote_count().checked_add(1).ok_or(ArithmeticError::Overflow)?;

			let now = <frame_system::Pallet<T>>::block_number();

			let end = now + VerificationVotingWindow::<T>::get().into();

			let vote = Vote::<T> {
				document_id: document_id,
				yes_votes: 0,
				no_votes: 0,
				start: now,
				end: end,
				status: VoteStatus::InProgress,
			};

			VerificationVotes::<T>::insert(uid.clone(),&vote);
			VerificationVotesCount::<T>::put(uid.clone());
			Self::deposit_event(Event::VerificationVotingStarted(uid));

			document.status = DocumentStatus::VoteInProgress;
			Documents::<T>::insert(document_id.clone(),document);
			Self::deposit_event(Event::DocumentStatusUpdated(document_id,3));
			
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn cast_qualification_vote(origin: OriginFor<T>, voting_id: u64, vote_cast: bool) -> DispatchResult  {
			let who = ensure_signed(origin)?;
			ensure!(Self::ensure_qualifier(who.clone()),Error::<T>::NotAQualifier);
			let vote_type = VoteType::Qualification;
			ensure!(!MemberVote::<T>::contains_key((who.clone(),vote_type.clone(),voting_id.clone())),Error::<T>::MemberAlreadyVoted);

			let mut vote = Self::get_qualification_vote(voting_id.clone()).ok_or(Error::<T>::VoteNotFound)?;
			ensure!(vote.status == VoteStatus::InProgress, Error::<T>::VoteNotInProgress);
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now > vote.start && now < vote.end, Error::<T>::VotingWindowNotValid);

			if vote_cast {
				vote.yes_votes = vote.yes_votes + 1;
			} else {
				vote.no_votes = vote.no_votes + 1;
			}


			QualificationVotes::<T>::insert(voting_id.clone(),&vote);
			MemberVote::<T>::insert((who.clone(),vote_type.clone(),voting_id.clone()),vote_cast);
			Self::deposit_event(Event::VoteCast(0,voting_id));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn cast_verification_vote(origin: OriginFor<T>, voting_id: u64, vote_cast: bool) -> DispatchResult  {
			let who = ensure_signed(origin)?;
			ensure!(Self::ensure_contributor(who.clone()),Error::<T>::NotAContributor);
			let vote_type = VoteType::Verification;
			ensure!(!MemberVote::<T>::contains_key((who.clone(),vote_type.clone(),voting_id.clone())),Error::<T>::MemberAlreadyVoted);

			let mut vote = Self::get_verification_vote(voting_id.clone()).ok_or(Error::<T>::VoteNotFound)?;
			ensure!(vote.status == VoteStatus::InProgress, Error::<T>::VoteNotInProgress);
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now > vote.start && now < vote.end, Error::<T>::VotingWindowNotValid);

			if vote_cast {
				vote.yes_votes = vote.yes_votes + 1;
			} else {
				vote.no_votes = vote.no_votes + 1;
			}

			VerificationVotes::<T>::insert(voting_id.clone(),&vote);
			MemberVote::<T>::insert((who.clone(),vote_type.clone(),voting_id.clone()),vote_cast);
			Self::deposit_event(Event::VoteCast(1,voting_id));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn finalize_qualification_voting(origin: OriginFor<T>, voting_id: u64) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::ensure_qualifier(who.clone()),Error::<T>::NotAQualifier);

			let mut vote = Self::get_qualification_vote(voting_id.clone()).ok_or(Error::<T>::VoteNotFound)?;
			ensure!(vote.status == VoteStatus::InProgress, Error::<T>::VoteNotInProgress);
			let mut document = Self::get_document(vote.document_id.clone()).ok_or(Error::<T>::DocumentNotFound)?;
			ensure!(document.status == DocumentStatus::UnderReview, Error::<T>::DocumentNotUnderReview);
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now > vote.end,Error::<T>::VoteStillInProgress);

			let quorum = QualificationQuorum::<T>::get().into();
			let total_votes = vote.yes_votes + vote.no_votes;

			if total_votes < quorum {
				vote.status = VoteStatus::Failed;
				document.status = DocumentStatus::Rejected;
				QualificationVotes::<T>::insert(voting_id.clone(),&vote);
				Documents::<T>::insert(vote.document_id.clone(),document);
				Self::deposit_event(Event::DocumentStatusUpdated(vote.document_id,5));
				Self::deposit_event(Event::QualificationVotingEnded(voting_id));

				return Ok(());
			}

			match vote.yes_votes > vote.no_votes {
				true => {
					vote.status = VoteStatus::Passed;
					document.status = DocumentStatus::SuccessfulReview;
					QualificationVotes::<T>::insert(voting_id.clone(),&vote);
					Documents::<T>::insert(vote.document_id.clone(),document);
					Self::deposit_event(Event::DocumentStatusUpdated(vote.document_id,2));
					Self::deposit_event(Event::QualificationVotingEnded(voting_id));
				},
				false => {
					vote.status = VoteStatus::Failed;
					document.status = DocumentStatus::Rejected;
					QualificationVotes::<T>::insert(voting_id.clone(),&vote);
					Documents::<T>::insert(vote.document_id.clone(),document);
					Self::deposit_event(Event::DocumentStatusUpdated(vote.document_id,5));
					Self::deposit_event(Event::QualificationVotingEnded(voting_id));
				},
			}

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn finalize_verification_voting(origin: OriginFor<T>, voting_id: u64) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::ensure_qualifier(who.clone()) || Self::ensure_contributor(who.clone()),Error::<T>::NotAuthorized);

			let mut vote = Self::get_verification_vote(voting_id.clone()).ok_or(Error::<T>::VoteNotFound)?;
			ensure!(vote.status == VoteStatus::InProgress, Error::<T>::VoteNotInProgress);
			let mut document = Self::get_document(vote.document_id.clone()).ok_or(Error::<T>::DocumentNotFound)?;
			ensure!(document.status == DocumentStatus::VoteInProgress, Error::<T>::IncorrectDocumentStatus);
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now > vote.end,Error::<T>::VoteStillInProgress);

			let quorum = VerificationQuorum::<T>::get().into();
			let total_votes = vote.yes_votes + vote.no_votes;

			if total_votes < quorum {
				vote.status = VoteStatus::Failed;
				document.status = DocumentStatus::Rejected;
				VerificationVotes::<T>::insert(voting_id.clone(),&vote);
				Documents::<T>::insert(vote.document_id.clone(),document);
				Self::deposit_event(Event::DocumentStatusUpdated(vote.document_id,5));
				Self::deposit_event(Event::VerificationVotingEnded(voting_id));

				return Ok(());
			}

			match vote.yes_votes > vote.no_votes {
				true => {
					vote.status = VoteStatus::Passed;
					document.status = DocumentStatus::Verified;
					VerificationVotes::<T>::insert(voting_id.clone(),&vote);
					Documents::<T>::insert(vote.document_id.clone(),document);
					Self::deposit_event(Event::DocumentStatusUpdated(vote.document_id,4));
					Self::deposit_event(Event::VerificationVotingEnded(voting_id));
				},
				false => {
					vote.status = VoteStatus::Failed;
					document.status = DocumentStatus::Rejected;
					VerificationVotes::<T>::insert(voting_id.clone(),&vote);
					Documents::<T>::insert(vote.document_id.clone(),document);
					Self::deposit_event(Event::DocumentStatusUpdated(vote.document_id,5));
					Self::deposit_event(Event::VerificationVotingEnded(voting_id));
				},
			}

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn set_qualification_voting_window(origin: OriginFor<T>, window: u32) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(window > 0, Error::<T>::VotingWindowNotValid);

			QualificationVotingWindow::<T>::put(window.clone());

			Self::deposit_event(Event::QualificationVotingWindowChanged(window));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn set_verification_voting_window(origin: OriginFor<T>, window: u32) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(window > 0, Error::<T>::VotingWindowNotValid);

			VerificationVotingWindow::<T>::put(window.clone());

			Self::deposit_event(Event::VerificationVotingWindowChanged(window));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn set_qualification_quorum(origin: OriginFor<T>, quorum: u32) -> DispatchResult {
			ensure_root(origin)?;

			QualificationQuorum::<T>::put(quorum.clone());

			Self::deposit_event(Event::QualificationQuorumChanged(quorum));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn set_verification_quorum(origin: OriginFor<T>, quorum: u32) -> DispatchResult {
			ensure_root(origin)?;

			VerificationQuorum::<T>::put(quorum.clone());

			Self::deposit_event(Event::VerificationQuorumChanged(quorum));

			Ok(())
		}

	}

	// Helpful functions
	impl<T: Config> Pallet<T> {
		pub fn ensure_contributor(who: T::AccountId) -> bool {

			//let val = pallet_nft::Pallet::<T>::get_token((who.clone(),Roles::QualifierRole as u32));
			let contributors = Contributors::<T>::get();

			let check: bool;

			match contributors.binary_search(&who) {
				Ok(_) => check = true,
				Err(_index) => check = false,
			}
			
			check
		}

		pub fn ensure_collector(who: T::AccountId) -> bool {
			let collectors = Collectors::<T>::get();

			let check: bool;

			match collectors.binary_search(&who) {
				Ok(_) => check = true,
				Err(_index) => check = false,
			}
			
			check
		}

		pub fn ensure_qualifier(who: T::AccountId) -> bool {
			let qualifiers = Qualifiers::<T>::get();

			let check: bool;

			match qualifiers.binary_search(&who) {
				Ok(_) => check = true,
				Err(_index) => check = false,
			}
			
			check
		}

		pub fn update_document_status(document_uid: u64, status: u8) -> DispatchResult {
			let mut document = Self::get_document(document_uid).ok_or(Error::<T>::DocumentNotFound)?;

			match status {
				0 => {
					document.status = DocumentStatus::Submitted;
				},
				1 => {
					document.status = DocumentStatus::UnderReview;
				},
				2 => {
					document.status = DocumentStatus::SuccessfulReview;
				},
				3 => {
					document.status = DocumentStatus::VoteInProgress;
				},
				4 => {
					document.status = DocumentStatus::Verified;
				},
				5 => {
					document.status = DocumentStatus::Rejected;
				},
				_ => ()
			}

			Documents::<T>::insert(&document_uid, &document);
			Self::deposit_event(Event::DocumentStatusUpdated(document_uid,status));

			Ok(())
		}
		
	}
}

use sp_runtime::{offchain:http,
    transaction_validity::{
    InvalidTransaction, TransactionLongevity, TransactionValidity, ValidTransaction,
}};
use support::{debug, dispatch};
use system::offchain;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"abcd");

pub mod crypto {
    pub use super::KEY_TYPE;
    use sp_runtime::app_crypto::{app_crypto, sr25519};
    app_crypto!(sr25519, KEY_TYPE);
}

pub trait Trait: timestamp::Trait + system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Call: From<Call<Self>>;

    type SubmitSignedTransaction: offchain::SubmitSignedTransaction<Self, <Self as Trait>::Call>;
    type SubmitUnsignedTransaction: offchain::SubmitUnsignedTransaction<Self, <Self as Trait>::Call>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn offchain_worker(block: T::BlockNumber) {
            debug::info!("Hello World.");
        }
    }

    pub fn onchain_callback(origin, _block: T::BlockNumber, input: Vec<u8>) -> dispatch::Result {
        let who = ensure_signed(origin)?;
        debug::info!("{:?}", core::str::from_utf8(&input).unwrap());
        Ok(())
    }

    fn offchain_worker(block: T::BlockNumber) {
        // let call = Call::onchain_callback(block, b"Hello world!".to_vec());
        // T::SubmitSignedTransaction::submit_signed(call);
        // T::SubmitUnsignedTransaction::submit_unsigned(call);

        match Self::fetch_data() {
            Ok(res) => debug::info!("Result: {}", core::str::from_utf8(&res).unwrap()),
            Err(e) => debug::error!("Error fetch_data: {}", e);
        };
    }
}

#[allow(deprecated)]
impl<T: Trait> support::unsigned::ValidateUnsigned for Module<T> {
    type Call = Call<T>;

    fn validate_unsigned(call: &Self::Call) -> TransactionValidity {
        match call {
            Call::onchain_callback(block, input) => Ok(ValidTransaction {
                priority: 0,
                requires: vec![],
                provides: vec![(block, input).encode()],
                longevity: TransactionLongevity::max_value(),
                propagate: true,
            }),
            _ => InvalidTransaction::Call.into(),
        }
    }
}

impl<T:Trait> Module<T> {
    fn fetch_data() -> Result<Vec<u8>, &'static str> {
        let pending = http::Request::get("https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD")
        .send()
        .map_err(|_| "Error in sending http GET request")?;

        let response = pending.wait().map_err(|_| "Error in waiting http response back")?;

        if response.code != 200 {
            debug::warn!("Unexpected status code: {}", response.code);
            return Err("Non-200 status code returned from http request");
        }

        Ok(response.body().collect::<Vec<u8>>())
    }
}
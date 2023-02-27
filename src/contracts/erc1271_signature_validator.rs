pub use erc1271_signature_validator::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod erc1271_signature_validator {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ERC1271SignatureValidator was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidSignature\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"magicValue\",\"type\":\"bytes4\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ERC1271SIGNATUREVALIDATOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct ERC1271SignatureValidator<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ERC1271SignatureValidator<M> {
        fn clone(&self) -> Self {
            ERC1271SignatureValidator(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ERC1271SignatureValidator<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ERC1271SignatureValidator<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ERC1271SignatureValidator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ERC1271SignatureValidator<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                ERC1271SIGNATUREVALIDATOR_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `isValidSignature` (0x20c13b0b) function"]
        pub fn is_valid_signature(
            &self,
            data: ethers::core::types::Bytes,
            signature: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([32, 193, 59, 11], (data, signature))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ERC1271SignatureValidator<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `isValidSignature` function with signature `isValidSignature(bytes,bytes)` and selector `[32, 193, 59, 11]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isValidSignature", abi = "isValidSignature(bytes,bytes)")]
    pub struct IsValidSignatureCall {
        pub data: ethers::core::types::Bytes,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all return fields from the `isValidSignature` function with signature `isValidSignature(bytes,bytes)` and selector `[32, 193, 59, 11]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsValidSignatureReturn {
        pub magic_value: [u8; 4],
    }
}

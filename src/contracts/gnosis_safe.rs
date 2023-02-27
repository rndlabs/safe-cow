pub use gnosis_safe::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod gnosis_safe {
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
    #[doc = "GnosisSafe was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AddedOwner\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"approvedHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ApproveHash\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"handler\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ChangedFallbackHandler\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"guard\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ChangedGuard\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"threshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ChangedThreshold\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"module\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DisabledModule\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"module\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EnabledModule\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"payment\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExecutionFailure\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"module\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ExecutionFromModuleFailure\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"module\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ExecutionFromModuleSuccess\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"payment\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExecutionSuccess\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RemovedOwner\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SafeReceived\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"initiator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address[]\",\"name\":\"owners\",\"type\":\"address[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"threshold\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"initializer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"fallbackHandler\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SafeSetup\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"msgHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"SignMsg\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"VERSION\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_threshold\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addOwnerWithThreshold\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"hashToApprove\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveHash\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"approvedHashes\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_threshold\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeThreshold\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signatures\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"requiredSignatures\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkNSignatures\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signatures\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkSignatures\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"prevModule\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"module\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disableModule\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"domainSeparator\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"module\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enableModule\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"enum Enum.Operation\",\"name\":\"operation\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"safeTxGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"baseGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasPrice\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"gasToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refundReceiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"encodeTransactionData\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"enum Enum.Operation\",\"name\":\"operation\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"safeTxGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"baseGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasPrice\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"gasToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"refundReceiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signatures\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"execTransaction\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"enum Enum.Operation\",\"name\":\"operation\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"execTransactionFromModule\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"enum Enum.Operation\",\"name\":\"operation\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"execTransactionFromModuleReturnData\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"start\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"pageSize\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getModulesPaginated\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"array\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"next\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOwners\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"offset\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"length\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getStorageAt\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"enum Enum.Operation\",\"name\":\"operation\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"safeTxGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"baseGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasPrice\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"gasToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"refundReceiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTransactionHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"module\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isModuleEnabled\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOwner\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"prevOwner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_threshold\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"enum Enum.Operation\",\"name\":\"operation\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"requiredTxGas\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"handler\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFallbackHandler\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"guard\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setGuard\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_owners\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_threshold\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"fallbackHandler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"paymentToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"payment\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"paymentReceiver\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setup\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"signedMessages\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"targetContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"calldataPayload\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"simulateAndRevert\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"prevOwner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"oldOwner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapOwner\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static GNOSISSAFE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct GnosisSafe<M>(ethers::contract::Contract<M>);
    impl<M> Clone for GnosisSafe<M> {
        fn clone(&self) -> Self {
            GnosisSafe(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for GnosisSafe<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for GnosisSafe<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(GnosisSafe))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> GnosisSafe<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), GNOSISSAFE_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `VERSION` (0xffa1ad74) function"]
        pub fn version(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addOwnerWithThreshold` (0x0d582f13) function"]
        pub fn add_owner_with_threshold(
            &self,
            owner: ethers::core::types::Address,
            threshold: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 88, 47, 19], (owner, threshold))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approveHash` (0xd4d9bdcd) function"]
        pub fn approve_hash(
            &self,
            hash_to_approve: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 217, 189, 205], hash_to_approve)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approvedHashes` (0x7d832974) function"]
        pub fn approved_hashes(
            &self,
            p0: ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([125, 131, 41, 116], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeThreshold` (0x694e80c3) function"]
        pub fn change_threshold(
            &self,
            threshold: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 78, 128, 195], threshold)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkNSignatures` (0x12fb68e0) function"]
        pub fn check_n_signatures(
            &self,
            data_hash: [u8; 32],
            data: ethers::core::types::Bytes,
            signatures: ethers::core::types::Bytes,
            required_signatures: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [18, 251, 104, 224],
                    (data_hash, data, signatures, required_signatures),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkSignatures` (0x934f3a11) function"]
        pub fn check_signatures(
            &self,
            data_hash: [u8; 32],
            data: ethers::core::types::Bytes,
            signatures: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 79, 58, 17], (data_hash, data, signatures))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `disableModule` (0xe009cfde) function"]
        pub fn disable_module(
            &self,
            prev_module: ethers::core::types::Address,
            module: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 9, 207, 222], (prev_module, module))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `domainSeparator` (0xf698da25) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([246, 152, 218, 37], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enableModule` (0x610b5925) function"]
        pub fn enable_module(
            &self,
            module: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 11, 89, 37], module)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `encodeTransactionData` (0xe86637db) function"]
        pub fn encode_transaction_data(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ethers::core::types::U256,
            base_gas: ethers::core::types::U256,
            gas_price: ethers::core::types::U256,
            gas_token: ethers::core::types::Address,
            refund_receiver: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash(
                    [232, 102, 55, 219],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        nonce,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `execTransaction` (0x6a761202) function"]
        pub fn exec_transaction(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ethers::core::types::U256,
            base_gas: ethers::core::types::U256,
            gas_price: ethers::core::types::U256,
            gas_token: ethers::core::types::Address,
            refund_receiver: ethers::core::types::Address,
            signatures: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [106, 118, 18, 2],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        signatures,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `execTransactionFromModule` (0x468721a7) function"]
        pub fn exec_transaction_from_module(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
            operation: u8,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([70, 135, 33, 167], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `execTransactionFromModuleReturnData` (0x5229073f) function"]
        pub fn exec_transaction_from_module_return_data(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
            operation: u8,
        ) -> ethers::contract::builders::ContractCall<M, (bool, ethers::core::types::Bytes)>
        {
            self.0
                .method_hash([82, 41, 7, 63], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getChainId` (0x3408e470) function"]
        pub fn get_chain_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getModulesPaginated` (0xcc2f8452) function"]
        pub fn get_modules_paginated(
            &self,
            start: ethers::core::types::Address,
            page_size: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<ethers::core::types::Address>,
                ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([204, 47, 132, 82], (start, page_size))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOwners` (0xa0e67e2b) function"]
        pub fn get_owners(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([160, 230, 126, 43], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getStorageAt` (0x5624b25b) function"]
        pub fn get_storage_at(
            &self,
            offset: ethers::core::types::U256,
            length: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([86, 36, 178, 91], (offset, length))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getThreshold` (0xe75235b8) function"]
        pub fn get_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([231, 82, 53, 184], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTransactionHash` (0xd8d11f78) function"]
        pub fn get_transaction_hash(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
            operation: u8,
            safe_tx_gas: ethers::core::types::U256,
            base_gas: ethers::core::types::U256,
            gas_price: ethers::core::types::U256,
            gas_token: ethers::core::types::Address,
            refund_receiver: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [216, 209, 31, 120],
                    (
                        to,
                        value,
                        data,
                        operation,
                        safe_tx_gas,
                        base_gas,
                        gas_price,
                        gas_token,
                        refund_receiver,
                        nonce,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isModuleEnabled` (0x2d9ad53d) function"]
        pub fn is_module_enabled(
            &self,
            module: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([45, 154, 213, 61], module)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOwner` (0x2f54bf6e) function"]
        pub fn is_owner(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 84, 191, 110], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonce` (0xaffed0e0) function"]
        pub fn nonce(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeOwner` (0xf8dc5dd9) function"]
        pub fn remove_owner(
            &self,
            prev_owner: ethers::core::types::Address,
            owner: ethers::core::types::Address,
            threshold: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 220, 93, 217], (prev_owner, owner, threshold))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `requiredTxGas` (0xc4ca3a9c) function"]
        pub fn required_tx_gas(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
            operation: u8,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([196, 202, 58, 156], (to, value, data, operation))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFallbackHandler` (0xf08a0323) function"]
        pub fn set_fallback_handler(
            &self,
            handler: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 138, 3, 35], handler)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGuard` (0xe19a9dd9) function"]
        pub fn set_guard(
            &self,
            guard: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 154, 157, 217], guard)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setup` (0xb63e800d) function"]
        pub fn setup(
            &self,
            owners: ::std::vec::Vec<ethers::core::types::Address>,
            threshold: ethers::core::types::U256,
            to: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
            fallback_handler: ethers::core::types::Address,
            payment_token: ethers::core::types::Address,
            payment: ethers::core::types::U256,
            payment_receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [182, 62, 128, 13],
                    (
                        owners,
                        threshold,
                        to,
                        data,
                        fallback_handler,
                        payment_token,
                        payment,
                        payment_receiver,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `signedMessages` (0x5ae6bd37) function"]
        pub fn signed_messages(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([90, 230, 189, 55], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `simulateAndRevert` (0xb4faba09) function"]
        pub fn simulate_and_revert(
            &self,
            target_contract: ethers::core::types::Address,
            calldata_payload: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 250, 186, 9], (target_contract, calldata_payload))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapOwner` (0xe318b52b) function"]
        pub fn swap_owner(
            &self,
            prev_owner: ethers::core::types::Address,
            old_owner: ethers::core::types::Address,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 24, 181, 43], (prev_owner, old_owner, new_owner))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AddedOwner` event"]
        pub fn added_owner_filter(&self) -> ethers::contract::builders::Event<M, AddedOwnerFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ApproveHash` event"]
        pub fn approve_hash_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ApproveHashFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ChangedFallbackHandler` event"]
        pub fn changed_fallback_handler_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ChangedFallbackHandlerFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ChangedGuard` event"]
        pub fn changed_guard_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ChangedGuardFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ChangedThreshold` event"]
        pub fn changed_threshold_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ChangedThresholdFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DisabledModule` event"]
        pub fn disabled_module_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DisabledModuleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EnabledModule` event"]
        pub fn enabled_module_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EnabledModuleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExecutionFailure` event"]
        pub fn execution_failure_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecutionFailureFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExecutionFromModuleFailure` event"]
        pub fn execution_from_module_failure_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecutionFromModuleFailureFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExecutionFromModuleSuccess` event"]
        pub fn execution_from_module_success_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecutionFromModuleSuccessFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExecutionSuccess` event"]
        pub fn execution_success_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecutionSuccessFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RemovedOwner` event"]
        pub fn removed_owner_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RemovedOwnerFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SafeReceived` event"]
        pub fn safe_received_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SafeReceivedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SafeSetup` event"]
        pub fn safe_setup_filter(&self) -> ethers::contract::builders::Event<M, SafeSetupFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SignMsg` event"]
        pub fn sign_msg_filter(&self) -> ethers::contract::builders::Event<M, SignMsgFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, GnosisSafeEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for GnosisSafe<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "AddedOwner", abi = "AddedOwner(address)")]
    pub struct AddedOwnerFilter {
        pub owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "ApproveHash", abi = "ApproveHash(bytes32,address)")]
    pub struct ApproveHashFilter {
        #[ethevent(indexed)]
        pub approved_hash: [u8; 32],
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ChangedFallbackHandler",
        abi = "ChangedFallbackHandler(address)"
    )]
    pub struct ChangedFallbackHandlerFilter {
        pub handler: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "ChangedGuard", abi = "ChangedGuard(address)")]
    pub struct ChangedGuardFilter {
        pub guard: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "ChangedThreshold", abi = "ChangedThreshold(uint256)")]
    pub struct ChangedThresholdFilter {
        pub threshold: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "DisabledModule", abi = "DisabledModule(address)")]
    pub struct DisabledModuleFilter {
        pub module: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "EnabledModule", abi = "EnabledModule(address)")]
    pub struct EnabledModuleFilter {
        pub module: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "ExecutionFailure", abi = "ExecutionFailure(bytes32,uint256)")]
    pub struct ExecutionFailureFilter {
        pub tx_hash: [u8; 32],
        pub payment: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ExecutionFromModuleFailure",
        abi = "ExecutionFromModuleFailure(address)"
    )]
    pub struct ExecutionFromModuleFailureFilter {
        #[ethevent(indexed)]
        pub module: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ExecutionFromModuleSuccess",
        abi = "ExecutionFromModuleSuccess(address)"
    )]
    pub struct ExecutionFromModuleSuccessFilter {
        #[ethevent(indexed)]
        pub module: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "ExecutionSuccess", abi = "ExecutionSuccess(bytes32,uint256)")]
    pub struct ExecutionSuccessFilter {
        pub tx_hash: [u8; 32],
        pub payment: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "RemovedOwner", abi = "RemovedOwner(address)")]
    pub struct RemovedOwnerFilter {
        pub owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SafeReceived", abi = "SafeReceived(address,uint256)")]
    pub struct SafeReceivedFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "SafeSetup",
        abi = "SafeSetup(address,address[],uint256,address,address)"
    )]
    pub struct SafeSetupFilter {
        #[ethevent(indexed)]
        pub initiator: ethers::core::types::Address,
        pub owners: Vec<ethers::core::types::Address>,
        pub threshold: ethers::core::types::U256,
        pub initializer: ethers::core::types::Address,
        pub fallback_handler: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SignMsg", abi = "SignMsg(bytes32)")]
    pub struct SignMsgFilter {
        #[ethevent(indexed)]
        pub msg_hash: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum GnosisSafeEvents {
        AddedOwnerFilter(AddedOwnerFilter),
        ApproveHashFilter(ApproveHashFilter),
        ChangedFallbackHandlerFilter(ChangedFallbackHandlerFilter),
        ChangedGuardFilter(ChangedGuardFilter),
        ChangedThresholdFilter(ChangedThresholdFilter),
        DisabledModuleFilter(DisabledModuleFilter),
        EnabledModuleFilter(EnabledModuleFilter),
        ExecutionFailureFilter(ExecutionFailureFilter),
        ExecutionFromModuleFailureFilter(ExecutionFromModuleFailureFilter),
        ExecutionFromModuleSuccessFilter(ExecutionFromModuleSuccessFilter),
        ExecutionSuccessFilter(ExecutionSuccessFilter),
        RemovedOwnerFilter(RemovedOwnerFilter),
        SafeReceivedFilter(SafeReceivedFilter),
        SafeSetupFilter(SafeSetupFilter),
        SignMsgFilter(SignMsgFilter),
    }
    impl ethers::contract::EthLogDecode for GnosisSafeEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddedOwnerFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::AddedOwnerFilter(decoded));
            }
            if let Ok(decoded) = ApproveHashFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ApproveHashFilter(decoded));
            }
            if let Ok(decoded) = ChangedFallbackHandlerFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ChangedFallbackHandlerFilter(decoded));
            }
            if let Ok(decoded) = ChangedGuardFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ChangedGuardFilter(decoded));
            }
            if let Ok(decoded) = ChangedThresholdFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ChangedThresholdFilter(decoded));
            }
            if let Ok(decoded) = DisabledModuleFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::DisabledModuleFilter(decoded));
            }
            if let Ok(decoded) = EnabledModuleFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::EnabledModuleFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFailureFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ExecutionFailureFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFromModuleFailureFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ExecutionFromModuleFailureFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFromModuleSuccessFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ExecutionFromModuleSuccessFilter(decoded));
            }
            if let Ok(decoded) = ExecutionSuccessFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::ExecutionSuccessFilter(decoded));
            }
            if let Ok(decoded) = RemovedOwnerFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::RemovedOwnerFilter(decoded));
            }
            if let Ok(decoded) = SafeReceivedFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::SafeReceivedFilter(decoded));
            }
            if let Ok(decoded) = SafeSetupFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::SafeSetupFilter(decoded));
            }
            if let Ok(decoded) = SignMsgFilter::decode_log(log) {
                return Ok(GnosisSafeEvents::SignMsgFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for GnosisSafeEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GnosisSafeEvents::AddedOwnerFilter(element) => element.fmt(f),
                GnosisSafeEvents::ApproveHashFilter(element) => element.fmt(f),
                GnosisSafeEvents::ChangedFallbackHandlerFilter(element) => element.fmt(f),
                GnosisSafeEvents::ChangedGuardFilter(element) => element.fmt(f),
                GnosisSafeEvents::ChangedThresholdFilter(element) => element.fmt(f),
                GnosisSafeEvents::DisabledModuleFilter(element) => element.fmt(f),
                GnosisSafeEvents::EnabledModuleFilter(element) => element.fmt(f),
                GnosisSafeEvents::ExecutionFailureFilter(element) => element.fmt(f),
                GnosisSafeEvents::ExecutionFromModuleFailureFilter(element) => element.fmt(f),
                GnosisSafeEvents::ExecutionFromModuleSuccessFilter(element) => element.fmt(f),
                GnosisSafeEvents::ExecutionSuccessFilter(element) => element.fmt(f),
                GnosisSafeEvents::RemovedOwnerFilter(element) => element.fmt(f),
                GnosisSafeEvents::SafeReceivedFilter(element) => element.fmt(f),
                GnosisSafeEvents::SafeSetupFilter(element) => element.fmt(f),
                GnosisSafeEvents::SignMsgFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `VERSION` function with signature `VERSION()` and selector `[255, 161, 173, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "VERSION", abi = "VERSION()")]
    pub struct VersionCall;
    #[doc = "Container type for all input parameters for the `addOwnerWithThreshold` function with signature `addOwnerWithThreshold(address,uint256)` and selector `[13, 88, 47, 19]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "addOwnerWithThreshold",
        abi = "addOwnerWithThreshold(address,uint256)"
    )]
    pub struct AddOwnerWithThresholdCall {
        pub owner: ethers::core::types::Address,
        pub threshold: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `approveHash` function with signature `approveHash(bytes32)` and selector `[212, 217, 189, 205]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approveHash", abi = "approveHash(bytes32)")]
    pub struct ApproveHashCall {
        pub hash_to_approve: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `approvedHashes` function with signature `approvedHashes(address,bytes32)` and selector `[125, 131, 41, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approvedHashes", abi = "approvedHashes(address,bytes32)")]
    pub struct ApprovedHashesCall(pub ethers::core::types::Address, pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `changeThreshold` function with signature `changeThreshold(uint256)` and selector `[105, 78, 128, 195]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "changeThreshold", abi = "changeThreshold(uint256)")]
    pub struct ChangeThresholdCall {
        pub threshold: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `checkNSignatures` function with signature `checkNSignatures(bytes32,bytes,bytes,uint256)` and selector `[18, 251, 104, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "checkNSignatures",
        abi = "checkNSignatures(bytes32,bytes,bytes,uint256)"
    )]
    pub struct CheckNSignaturesCall {
        pub data_hash: [u8; 32],
        pub data: ethers::core::types::Bytes,
        pub signatures: ethers::core::types::Bytes,
        pub required_signatures: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `checkSignatures` function with signature `checkSignatures(bytes32,bytes,bytes)` and selector `[147, 79, 58, 17]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "checkSignatures", abi = "checkSignatures(bytes32,bytes,bytes)")]
    pub struct CheckSignaturesCall {
        pub data_hash: [u8; 32],
        pub data: ethers::core::types::Bytes,
        pub signatures: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `disableModule` function with signature `disableModule(address,address)` and selector `[224, 9, 207, 222]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "disableModule", abi = "disableModule(address,address)")]
    pub struct DisableModuleCall {
        pub prev_module: ethers::core::types::Address,
        pub module: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `domainSeparator` function with signature `domainSeparator()` and selector `[246, 152, 218, 37]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "domainSeparator", abi = "domainSeparator()")]
    pub struct DomainSeparatorCall;
    #[doc = "Container type for all input parameters for the `enableModule` function with signature `enableModule(address)` and selector `[97, 11, 89, 37]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "enableModule", abi = "enableModule(address)")]
    pub struct EnableModuleCall {
        pub module: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `encodeTransactionData` function with signature `encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `[232, 102, 55, 219]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "encodeTransactionData",
        abi = "encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)"
    )]
    pub struct EncodeTransactionDataCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ethers::core::types::U256,
        pub base_gas: ethers::core::types::U256,
        pub gas_price: ethers::core::types::U256,
        pub gas_token: ethers::core::types::Address,
        pub refund_receiver: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `execTransaction` function with signature `execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)` and selector `[106, 118, 18, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "execTransaction",
        abi = "execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)"
    )]
    pub struct ExecTransactionCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ethers::core::types::U256,
        pub base_gas: ethers::core::types::U256,
        pub gas_price: ethers::core::types::U256,
        pub gas_token: ethers::core::types::Address,
        pub refund_receiver: ethers::core::types::Address,
        pub signatures: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `execTransactionFromModule` function with signature `execTransactionFromModule(address,uint256,bytes,uint8)` and selector `[70, 135, 33, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "execTransactionFromModule",
        abi = "execTransactionFromModule(address,uint256,bytes,uint8)"
    )]
    pub struct ExecTransactionFromModuleCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
        pub operation: u8,
    }
    #[doc = "Container type for all input parameters for the `execTransactionFromModuleReturnData` function with signature `execTransactionFromModuleReturnData(address,uint256,bytes,uint8)` and selector `[82, 41, 7, 63]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "execTransactionFromModuleReturnData",
        abi = "execTransactionFromModuleReturnData(address,uint256,bytes,uint8)"
    )]
    pub struct ExecTransactionFromModuleReturnDataCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
        pub operation: u8,
    }
    #[doc = "Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    #[doc = "Container type for all input parameters for the `getModulesPaginated` function with signature `getModulesPaginated(address,uint256)` and selector `[204, 47, 132, 82]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getModulesPaginated",
        abi = "getModulesPaginated(address,uint256)"
    )]
    pub struct GetModulesPaginatedCall {
        pub start: ethers::core::types::Address,
        pub page_size: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getOwners` function with signature `getOwners()` and selector `[160, 230, 126, 43]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getOwners", abi = "getOwners()")]
    pub struct GetOwnersCall;
    #[doc = "Container type for all input parameters for the `getStorageAt` function with signature `getStorageAt(uint256,uint256)` and selector `[86, 36, 178, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getStorageAt", abi = "getStorageAt(uint256,uint256)")]
    pub struct GetStorageAtCall {
        pub offset: ethers::core::types::U256,
        pub length: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getThreshold` function with signature `getThreshold()` and selector `[231, 82, 53, 184]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getThreshold", abi = "getThreshold()")]
    pub struct GetThresholdCall;
    #[doc = "Container type for all input parameters for the `getTransactionHash` function with signature `getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `[216, 209, 31, 120]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getTransactionHash",
        abi = "getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)"
    )]
    pub struct GetTransactionHashCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
        pub operation: u8,
        pub safe_tx_gas: ethers::core::types::U256,
        pub base_gas: ethers::core::types::U256,
        pub gas_price: ethers::core::types::U256,
        pub gas_token: ethers::core::types::Address,
        pub refund_receiver: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isModuleEnabled` function with signature `isModuleEnabled(address)` and selector `[45, 154, 213, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isModuleEnabled", abi = "isModuleEnabled(address)")]
    pub struct IsModuleEnabledCall {
        pub module: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isOwner` function with signature `isOwner(address)` and selector `[47, 84, 191, 110]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isOwner", abi = "isOwner(address)")]
    pub struct IsOwnerCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `nonce` function with signature `nonce()` and selector `[175, 254, 208, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "nonce", abi = "nonce()")]
    pub struct NonceCall;
    #[doc = "Container type for all input parameters for the `removeOwner` function with signature `removeOwner(address,address,uint256)` and selector `[248, 220, 93, 217]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "removeOwner", abi = "removeOwner(address,address,uint256)")]
    pub struct RemoveOwnerCall {
        pub prev_owner: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
        pub threshold: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `requiredTxGas` function with signature `requiredTxGas(address,uint256,bytes,uint8)` and selector `[196, 202, 58, 156]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "requiredTxGas",
        abi = "requiredTxGas(address,uint256,bytes,uint8)"
    )]
    pub struct RequiredTxGasCall {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
        pub operation: u8,
    }
    #[doc = "Container type for all input parameters for the `setFallbackHandler` function with signature `setFallbackHandler(address)` and selector `[240, 138, 3, 35]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFallbackHandler", abi = "setFallbackHandler(address)")]
    pub struct SetFallbackHandlerCall {
        pub handler: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setGuard` function with signature `setGuard(address)` and selector `[225, 154, 157, 217]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setGuard", abi = "setGuard(address)")]
    pub struct SetGuardCall {
        pub guard: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setup` function with signature `setup(address[],uint256,address,bytes,address,address,uint256,address)` and selector `[182, 62, 128, 13]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setup",
        abi = "setup(address[],uint256,address,bytes,address,address,uint256,address)"
    )]
    pub struct SetupCall {
        pub owners: ::std::vec::Vec<ethers::core::types::Address>,
        pub threshold: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
        pub fallback_handler: ethers::core::types::Address,
        pub payment_token: ethers::core::types::Address,
        pub payment: ethers::core::types::U256,
        pub payment_receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `signedMessages` function with signature `signedMessages(bytes32)` and selector `[90, 230, 189, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "signedMessages", abi = "signedMessages(bytes32)")]
    pub struct SignedMessagesCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `simulateAndRevert` function with signature `simulateAndRevert(address,bytes)` and selector `[180, 250, 186, 9]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "simulateAndRevert", abi = "simulateAndRevert(address,bytes)")]
    pub struct SimulateAndRevertCall {
        pub target_contract: ethers::core::types::Address,
        pub calldata_payload: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `swapOwner` function with signature `swapOwner(address,address,address)` and selector `[227, 24, 181, 43]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "swapOwner", abi = "swapOwner(address,address,address)")]
    pub struct SwapOwnerCall {
        pub prev_owner: ethers::core::types::Address,
        pub old_owner: ethers::core::types::Address,
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum GnosisSafeCalls {
        Version(VersionCall),
        AddOwnerWithThreshold(AddOwnerWithThresholdCall),
        ApproveHash(ApproveHashCall),
        ApprovedHashes(ApprovedHashesCall),
        ChangeThreshold(ChangeThresholdCall),
        CheckNSignatures(CheckNSignaturesCall),
        CheckSignatures(CheckSignaturesCall),
        DisableModule(DisableModuleCall),
        DomainSeparator(DomainSeparatorCall),
        EnableModule(EnableModuleCall),
        EncodeTransactionData(EncodeTransactionDataCall),
        ExecTransaction(ExecTransactionCall),
        ExecTransactionFromModule(ExecTransactionFromModuleCall),
        ExecTransactionFromModuleReturnData(ExecTransactionFromModuleReturnDataCall),
        GetChainId(GetChainIdCall),
        GetModulesPaginated(GetModulesPaginatedCall),
        GetOwners(GetOwnersCall),
        GetStorageAt(GetStorageAtCall),
        GetThreshold(GetThresholdCall),
        GetTransactionHash(GetTransactionHashCall),
        IsModuleEnabled(IsModuleEnabledCall),
        IsOwner(IsOwnerCall),
        Nonce(NonceCall),
        RemoveOwner(RemoveOwnerCall),
        RequiredTxGas(RequiredTxGasCall),
        SetFallbackHandler(SetFallbackHandlerCall),
        SetGuard(SetGuardCall),
        Setup(SetupCall),
        SignedMessages(SignedMessagesCall),
        SimulateAndRevert(SimulateAndRevertCall),
        SwapOwner(SwapOwnerCall),
    }
    impl ethers::core::abi::AbiDecode for GnosisSafeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::Version(decoded));
            }
            if let Ok(decoded) =
                <AddOwnerWithThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::AddOwnerWithThreshold(decoded));
            }
            if let Ok(decoded) =
                <ApproveHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::ApproveHash(decoded));
            }
            if let Ok(decoded) =
                <ApprovedHashesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::ApprovedHashes(decoded));
            }
            if let Ok(decoded) =
                <ChangeThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::ChangeThreshold(decoded));
            }
            if let Ok(decoded) =
                <CheckNSignaturesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::CheckNSignatures(decoded));
            }
            if let Ok(decoded) =
                <CheckSignaturesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::CheckSignatures(decoded));
            }
            if let Ok(decoded) =
                <DisableModuleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::DisableModule(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <EnableModuleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::EnableModule(decoded));
            }
            if let Ok(decoded) =
                <EncodeTransactionDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::EncodeTransactionData(decoded));
            }
            if let Ok(decoded) =
                <ExecTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::ExecTransaction(decoded));
            }
            if let Ok(decoded) =
                <ExecTransactionFromModuleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GnosisSafeCalls::ExecTransactionFromModule(decoded));
            }
            if let Ok(decoded) =
                <ExecTransactionFromModuleReturnDataCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GnosisSafeCalls::ExecTransactionFromModuleReturnData(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <GetModulesPaginatedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::GetModulesPaginated(decoded));
            }
            if let Ok(decoded) =
                <GetOwnersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::GetOwners(decoded));
            }
            if let Ok(decoded) =
                <GetStorageAtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::GetStorageAt(decoded));
            }
            if let Ok(decoded) =
                <GetThresholdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::GetThreshold(decoded));
            }
            if let Ok(decoded) =
                <GetTransactionHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::GetTransactionHash(decoded));
            }
            if let Ok(decoded) =
                <IsModuleEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::IsModuleEnabled(decoded));
            }
            if let Ok(decoded) =
                <IsOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::IsOwner(decoded));
            }
            if let Ok(decoded) = <NonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::Nonce(decoded));
            }
            if let Ok(decoded) =
                <RemoveOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::RemoveOwner(decoded));
            }
            if let Ok(decoded) =
                <RequiredTxGasCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::RequiredTxGas(decoded));
            }
            if let Ok(decoded) =
                <SetFallbackHandlerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::SetFallbackHandler(decoded));
            }
            if let Ok(decoded) =
                <SetGuardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::SetGuard(decoded));
            }
            if let Ok(decoded) = <SetupCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::Setup(decoded));
            }
            if let Ok(decoded) =
                <SignedMessagesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::SignedMessages(decoded));
            }
            if let Ok(decoded) =
                <SimulateAndRevertCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::SimulateAndRevert(decoded));
            }
            if let Ok(decoded) =
                <SwapOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GnosisSafeCalls::SwapOwner(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for GnosisSafeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                GnosisSafeCalls::Version(element) => element.encode(),
                GnosisSafeCalls::AddOwnerWithThreshold(element) => element.encode(),
                GnosisSafeCalls::ApproveHash(element) => element.encode(),
                GnosisSafeCalls::ApprovedHashes(element) => element.encode(),
                GnosisSafeCalls::ChangeThreshold(element) => element.encode(),
                GnosisSafeCalls::CheckNSignatures(element) => element.encode(),
                GnosisSafeCalls::CheckSignatures(element) => element.encode(),
                GnosisSafeCalls::DisableModule(element) => element.encode(),
                GnosisSafeCalls::DomainSeparator(element) => element.encode(),
                GnosisSafeCalls::EnableModule(element) => element.encode(),
                GnosisSafeCalls::EncodeTransactionData(element) => element.encode(),
                GnosisSafeCalls::ExecTransaction(element) => element.encode(),
                GnosisSafeCalls::ExecTransactionFromModule(element) => element.encode(),
                GnosisSafeCalls::ExecTransactionFromModuleReturnData(element) => element.encode(),
                GnosisSafeCalls::GetChainId(element) => element.encode(),
                GnosisSafeCalls::GetModulesPaginated(element) => element.encode(),
                GnosisSafeCalls::GetOwners(element) => element.encode(),
                GnosisSafeCalls::GetStorageAt(element) => element.encode(),
                GnosisSafeCalls::GetThreshold(element) => element.encode(),
                GnosisSafeCalls::GetTransactionHash(element) => element.encode(),
                GnosisSafeCalls::IsModuleEnabled(element) => element.encode(),
                GnosisSafeCalls::IsOwner(element) => element.encode(),
                GnosisSafeCalls::Nonce(element) => element.encode(),
                GnosisSafeCalls::RemoveOwner(element) => element.encode(),
                GnosisSafeCalls::RequiredTxGas(element) => element.encode(),
                GnosisSafeCalls::SetFallbackHandler(element) => element.encode(),
                GnosisSafeCalls::SetGuard(element) => element.encode(),
                GnosisSafeCalls::Setup(element) => element.encode(),
                GnosisSafeCalls::SignedMessages(element) => element.encode(),
                GnosisSafeCalls::SimulateAndRevert(element) => element.encode(),
                GnosisSafeCalls::SwapOwner(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for GnosisSafeCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GnosisSafeCalls::Version(element) => element.fmt(f),
                GnosisSafeCalls::AddOwnerWithThreshold(element) => element.fmt(f),
                GnosisSafeCalls::ApproveHash(element) => element.fmt(f),
                GnosisSafeCalls::ApprovedHashes(element) => element.fmt(f),
                GnosisSafeCalls::ChangeThreshold(element) => element.fmt(f),
                GnosisSafeCalls::CheckNSignatures(element) => element.fmt(f),
                GnosisSafeCalls::CheckSignatures(element) => element.fmt(f),
                GnosisSafeCalls::DisableModule(element) => element.fmt(f),
                GnosisSafeCalls::DomainSeparator(element) => element.fmt(f),
                GnosisSafeCalls::EnableModule(element) => element.fmt(f),
                GnosisSafeCalls::EncodeTransactionData(element) => element.fmt(f),
                GnosisSafeCalls::ExecTransaction(element) => element.fmt(f),
                GnosisSafeCalls::ExecTransactionFromModule(element) => element.fmt(f),
                GnosisSafeCalls::ExecTransactionFromModuleReturnData(element) => element.fmt(f),
                GnosisSafeCalls::GetChainId(element) => element.fmt(f),
                GnosisSafeCalls::GetModulesPaginated(element) => element.fmt(f),
                GnosisSafeCalls::GetOwners(element) => element.fmt(f),
                GnosisSafeCalls::GetStorageAt(element) => element.fmt(f),
                GnosisSafeCalls::GetThreshold(element) => element.fmt(f),
                GnosisSafeCalls::GetTransactionHash(element) => element.fmt(f),
                GnosisSafeCalls::IsModuleEnabled(element) => element.fmt(f),
                GnosisSafeCalls::IsOwner(element) => element.fmt(f),
                GnosisSafeCalls::Nonce(element) => element.fmt(f),
                GnosisSafeCalls::RemoveOwner(element) => element.fmt(f),
                GnosisSafeCalls::RequiredTxGas(element) => element.fmt(f),
                GnosisSafeCalls::SetFallbackHandler(element) => element.fmt(f),
                GnosisSafeCalls::SetGuard(element) => element.fmt(f),
                GnosisSafeCalls::Setup(element) => element.fmt(f),
                GnosisSafeCalls::SignedMessages(element) => element.fmt(f),
                GnosisSafeCalls::SimulateAndRevert(element) => element.fmt(f),
                GnosisSafeCalls::SwapOwner(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<VersionCall> for GnosisSafeCalls {
        fn from(var: VersionCall) -> Self {
            GnosisSafeCalls::Version(var)
        }
    }
    impl ::std::convert::From<AddOwnerWithThresholdCall> for GnosisSafeCalls {
        fn from(var: AddOwnerWithThresholdCall) -> Self {
            GnosisSafeCalls::AddOwnerWithThreshold(var)
        }
    }
    impl ::std::convert::From<ApproveHashCall> for GnosisSafeCalls {
        fn from(var: ApproveHashCall) -> Self {
            GnosisSafeCalls::ApproveHash(var)
        }
    }
    impl ::std::convert::From<ApprovedHashesCall> for GnosisSafeCalls {
        fn from(var: ApprovedHashesCall) -> Self {
            GnosisSafeCalls::ApprovedHashes(var)
        }
    }
    impl ::std::convert::From<ChangeThresholdCall> for GnosisSafeCalls {
        fn from(var: ChangeThresholdCall) -> Self {
            GnosisSafeCalls::ChangeThreshold(var)
        }
    }
    impl ::std::convert::From<CheckNSignaturesCall> for GnosisSafeCalls {
        fn from(var: CheckNSignaturesCall) -> Self {
            GnosisSafeCalls::CheckNSignatures(var)
        }
    }
    impl ::std::convert::From<CheckSignaturesCall> for GnosisSafeCalls {
        fn from(var: CheckSignaturesCall) -> Self {
            GnosisSafeCalls::CheckSignatures(var)
        }
    }
    impl ::std::convert::From<DisableModuleCall> for GnosisSafeCalls {
        fn from(var: DisableModuleCall) -> Self {
            GnosisSafeCalls::DisableModule(var)
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for GnosisSafeCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            GnosisSafeCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<EnableModuleCall> for GnosisSafeCalls {
        fn from(var: EnableModuleCall) -> Self {
            GnosisSafeCalls::EnableModule(var)
        }
    }
    impl ::std::convert::From<EncodeTransactionDataCall> for GnosisSafeCalls {
        fn from(var: EncodeTransactionDataCall) -> Self {
            GnosisSafeCalls::EncodeTransactionData(var)
        }
    }
    impl ::std::convert::From<ExecTransactionCall> for GnosisSafeCalls {
        fn from(var: ExecTransactionCall) -> Self {
            GnosisSafeCalls::ExecTransaction(var)
        }
    }
    impl ::std::convert::From<ExecTransactionFromModuleCall> for GnosisSafeCalls {
        fn from(var: ExecTransactionFromModuleCall) -> Self {
            GnosisSafeCalls::ExecTransactionFromModule(var)
        }
    }
    impl ::std::convert::From<ExecTransactionFromModuleReturnDataCall> for GnosisSafeCalls {
        fn from(var: ExecTransactionFromModuleReturnDataCall) -> Self {
            GnosisSafeCalls::ExecTransactionFromModuleReturnData(var)
        }
    }
    impl ::std::convert::From<GetChainIdCall> for GnosisSafeCalls {
        fn from(var: GetChainIdCall) -> Self {
            GnosisSafeCalls::GetChainId(var)
        }
    }
    impl ::std::convert::From<GetModulesPaginatedCall> for GnosisSafeCalls {
        fn from(var: GetModulesPaginatedCall) -> Self {
            GnosisSafeCalls::GetModulesPaginated(var)
        }
    }
    impl ::std::convert::From<GetOwnersCall> for GnosisSafeCalls {
        fn from(var: GetOwnersCall) -> Self {
            GnosisSafeCalls::GetOwners(var)
        }
    }
    impl ::std::convert::From<GetStorageAtCall> for GnosisSafeCalls {
        fn from(var: GetStorageAtCall) -> Self {
            GnosisSafeCalls::GetStorageAt(var)
        }
    }
    impl ::std::convert::From<GetThresholdCall> for GnosisSafeCalls {
        fn from(var: GetThresholdCall) -> Self {
            GnosisSafeCalls::GetThreshold(var)
        }
    }
    impl ::std::convert::From<GetTransactionHashCall> for GnosisSafeCalls {
        fn from(var: GetTransactionHashCall) -> Self {
            GnosisSafeCalls::GetTransactionHash(var)
        }
    }
    impl ::std::convert::From<IsModuleEnabledCall> for GnosisSafeCalls {
        fn from(var: IsModuleEnabledCall) -> Self {
            GnosisSafeCalls::IsModuleEnabled(var)
        }
    }
    impl ::std::convert::From<IsOwnerCall> for GnosisSafeCalls {
        fn from(var: IsOwnerCall) -> Self {
            GnosisSafeCalls::IsOwner(var)
        }
    }
    impl ::std::convert::From<NonceCall> for GnosisSafeCalls {
        fn from(var: NonceCall) -> Self {
            GnosisSafeCalls::Nonce(var)
        }
    }
    impl ::std::convert::From<RemoveOwnerCall> for GnosisSafeCalls {
        fn from(var: RemoveOwnerCall) -> Self {
            GnosisSafeCalls::RemoveOwner(var)
        }
    }
    impl ::std::convert::From<RequiredTxGasCall> for GnosisSafeCalls {
        fn from(var: RequiredTxGasCall) -> Self {
            GnosisSafeCalls::RequiredTxGas(var)
        }
    }
    impl ::std::convert::From<SetFallbackHandlerCall> for GnosisSafeCalls {
        fn from(var: SetFallbackHandlerCall) -> Self {
            GnosisSafeCalls::SetFallbackHandler(var)
        }
    }
    impl ::std::convert::From<SetGuardCall> for GnosisSafeCalls {
        fn from(var: SetGuardCall) -> Self {
            GnosisSafeCalls::SetGuard(var)
        }
    }
    impl ::std::convert::From<SetupCall> for GnosisSafeCalls {
        fn from(var: SetupCall) -> Self {
            GnosisSafeCalls::Setup(var)
        }
    }
    impl ::std::convert::From<SignedMessagesCall> for GnosisSafeCalls {
        fn from(var: SignedMessagesCall) -> Self {
            GnosisSafeCalls::SignedMessages(var)
        }
    }
    impl ::std::convert::From<SimulateAndRevertCall> for GnosisSafeCalls {
        fn from(var: SimulateAndRevertCall) -> Self {
            GnosisSafeCalls::SimulateAndRevert(var)
        }
    }
    impl ::std::convert::From<SwapOwnerCall> for GnosisSafeCalls {
        fn from(var: SwapOwnerCall) -> Self {
            GnosisSafeCalls::SwapOwner(var)
        }
    }
    #[doc = "Container type for all return fields from the `VERSION` function with signature `VERSION()` and selector `[255, 161, 173, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VersionReturn(pub String);
    #[doc = "Container type for all return fields from the `approvedHashes` function with signature `approvedHashes(address,bytes32)` and selector `[125, 131, 41, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ApprovedHashesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `domainSeparator` function with signature `domainSeparator()` and selector `[246, 152, 218, 37]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `encodeTransactionData` function with signature `encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `[232, 102, 55, 219]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct EncodeTransactionDataReturn(pub ethers::core::types::Bytes);
    #[doc = "Container type for all return fields from the `execTransaction` function with signature `execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)` and selector `[106, 118, 18, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExecTransactionReturn {
        pub success: bool,
    }
    #[doc = "Container type for all return fields from the `execTransactionFromModule` function with signature `execTransactionFromModule(address,uint256,bytes,uint8)` and selector `[70, 135, 33, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExecTransactionFromModuleReturn {
        pub success: bool,
    }
    #[doc = "Container type for all return fields from the `execTransactionFromModuleReturnData` function with signature `execTransactionFromModuleReturnData(address,uint256,bytes,uint8)` and selector `[82, 41, 7, 63]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExecTransactionFromModuleReturnDataReturn {
        pub success: bool,
        pub return_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetChainIdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getModulesPaginated` function with signature `getModulesPaginated(address,uint256)` and selector `[204, 47, 132, 82]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetModulesPaginatedReturn {
        pub array: ::std::vec::Vec<ethers::core::types::Address>,
        pub next: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `getOwners` function with signature `getOwners()` and selector `[160, 230, 126, 43]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetOwnersReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `getStorageAt` function with signature `getStorageAt(uint256,uint256)` and selector `[86, 36, 178, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetStorageAtReturn(pub ethers::core::types::Bytes);
    #[doc = "Container type for all return fields from the `getThreshold` function with signature `getThreshold()` and selector `[231, 82, 53, 184]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetThresholdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getTransactionHash` function with signature `getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `[216, 209, 31, 120]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTransactionHashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `isModuleEnabled` function with signature `isModuleEnabled(address)` and selector `[45, 154, 213, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsModuleEnabledReturn(pub bool);
    #[doc = "Container type for all return fields from the `isOwner` function with signature `isOwner(address)` and selector `[47, 84, 191, 110]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsOwnerReturn(pub bool);
    #[doc = "Container type for all return fields from the `nonce` function with signature `nonce()` and selector `[175, 254, 208, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NonceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `requiredTxGas` function with signature `requiredTxGas(address,uint256,bytes,uint8)` and selector `[196, 202, 58, 156]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RequiredTxGasReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `signedMessages` function with signature `signedMessages(bytes32)` and selector `[90, 230, 189, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SignedMessagesReturn(pub ethers::core::types::U256);
}

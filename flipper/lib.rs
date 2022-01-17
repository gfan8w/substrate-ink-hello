#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;

/***
1）检查版本
$ rustup show
Default host: x86_64-apple-darwin
rustup home:  /Users/mmac/.rustup

installed toolchains
--------------------

stable-x86_64-apple-darwin (default)
nightly-2021-01-13-x86_64-apple-darwin
nightly-2021-09-08-x86_64-apple-darwin
nightly-x86_64-apple-darwin                   < ---- 这个是 nightly-2021-01-16

installed targets for active toolchain
--------------------------------------

wasm32-unknown-unknown
x86_64-apple-darwin

active toolchain
----------------

stable-x86_64-apple-darwin (default)
rustc 1.58.0 (02072b482 2022-01-11)

2）安装 cargo-contract 工具
cargo install cargo-contract --vers ^0.16 --force --locked


3）编译：
cargo +nightly contract build --release
或 cargo +nightly-2021-01-16 contract build --release
测试：cargo +nightly test

4）运行contract-node
clone 下载 substrate-contract-node，cargo build -- --dev --tmp

5）访问
https://paritytech.github.io/canvas-ui
上传 flipper.contract
实例化，
执行、或查询



 */


/// 这里演示ink合约如何调用 Substrate runtime的方法
///  `RandomnessCollectiveFlip::random_seed`. See the
/// file `runtime/chain-extension-example.rs` for that implementation.
///
/// Here we define the operations to interact with the Substrate runtime.
#[ink::chain_extension]
pub trait FetchRandom {
    type ErrorCode = RandomReadErr;

    /// Note: this gives the operation a corresponding `func_id` (1101 in this case),
    /// and the chain-side chain extension will get the `func_id` to do further operations.
    /// 1101是在runtime中定义的方法
    #[ink(extension = 1101, returns_result = false)]
    fn fetch_random() -> [u8; 32];
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RandomReadErr {
    FailGetRandomSource,
}

impl ink_env::chain_extension::FromStatusCode for RandomReadErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetRandomSource),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;
    // type RentFraction = <ink_env::DefaultEnvironment as Environment>::RentFraction;

    type ChainExtension = FetchRandom;
}


///使用自定义的Evniroment，主要就是使用了额外的ChainExtension
#[ink::contract(env = crate::CustomEnvironment)]
mod flipper {

    use super::RandomReadErr; //引入父级的error

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Flipper {
        /// Stores a single `bool` value on the storage.
        value: bool,
        ///保存随机值
        rand: [u8; 32],
    }

    ///随机值被更新的Event
    #[ink(event)]
    pub struct RandomUpdated {
        #[ink(topic)]
        new: [u8; 32],
    }


    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        /// give an u8 length 32 to rand
        #[ink(constructor)]
        pub fn new(init_value: bool, rand_value: [u8; 32]) -> Self {
            Self { value: init_value, rand:rand_value  }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(), Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_flip(&self) -> bool {
            self.value
        }

        /// 从substrate的运行时那里获取到rand的随机值
        #[ink(message)]
        pub fn update_rand(&mut self) -> Result<(), RandomReadErr> {
            // Get the on-chain random seed
            let new_random = self.env().extension().fetch_random()?;
            self.rand = new_random;
            // Emit the `RandomUpdated` event when the random seed
            // is successfully fetched.
            self.env().emit_event(RandomUpdated { new: new_random });
            Ok(())
        }

        /// Simply returns the current value.
        #[ink(message)]
        pub fn get_rand(&self) -> [u8; 32] {
            self.rand
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false,[0u8;32]);
            assert_eq!(flipper.get_flip(), false);
            flipper.flip();
            assert_eq!(flipper.get_flip(), true);
        }
    }
}

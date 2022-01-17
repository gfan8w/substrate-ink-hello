#![cfg_attr(not(feature = "std"), no_std)]

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




#[ink::contract]
mod flipper {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Flipper {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
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
        pub fn get(&self) -> bool {
            self.value
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
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }
    }
}

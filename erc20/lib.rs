#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

/// 一个简单的 ERC20 ink 合约
/// 合约操作地址： https://paritytech.github.io/canvas-ui/#/
#[ink::contract]
mod erc20 {

    use ink_storage::{
        collections::HashMap,
        lazy::Lazy,
    };


    /// erc20的储存
    #[ink(storage)]
    pub struct Erc20 {
        /// 货币总量
        total_supply: Lazy<Balance>,
        /// 每个账户余额
        balances: HashMap<AccountId, Balance>,
        /// 授权，可转账的数量
        allowances: HashMap<(AccountId, AccountId), Balance>,
    }

    /// 转账的信息，包含一个来源账号，一个接收账号，和 转账额
    #[ink(event)]
    pub struct Transfer {
        /// 来源账户，发起账号
        #[ink(topic)]
        from: Option<AccountId>,
        /// 接收账号
        to: Option<AccountId>,
        /// 转账额
        value: Balance,
    }

    /// 批准信息，包含发起账号，接收账号 和额度
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// 没有足够的余额
        InsufficientBalance,
        /// 没有足够的授权
        InsufficientApproval,
    }

    // 包装一下Error
    pub type Result<T> = core::result::Result<T,Error>;


    impl Erc20 {
        /// Constructor,构造，传入总量
        #[ink(constructor)]
        pub fn new(supply: Balance) -> Self {
            let caller =Self::env().caller();
            let mut balances = HashMap::new();
            balances.insert(caller,supply);

            Self::env().emit_event(Transfer{
                from:None,
                to: Some(caller),
                value: supply,
            });

            Self {
                total_supply: Lazy::new(supply),
                balances,
                allowances: HashMap::new(),
            }
        }

        /// 获取总的发行量
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            ink_env::debug_println!("total_supply: {}",*self.total_supply);
            *self.total_supply
        }

        /// 获取账号对应的余额
        #[ink(message)]
        pub fn banlance_of(&self, who: AccountId) -> Balance {
            self.balances.get(&who).copied().unwrap_or(0)
        }

        /// 获取可转额度
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance{
            self.allowances.get(&(owner,spender)).copied().unwrap_or(0)
        }

        /// 转账
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller(); // 等价于 Self::env().caller();
            self.inner_transfer(from,to,value)?;

            Ok(())
        }

        /// 审批账号的可转额度
        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner,to),value);
            self.env().emit_event(Approval{
                owner,
                spender:to,
                value
            });
            Ok(())
        }

        /// 从from 账号转账给to
        #[ink(message)]
        pub fn transfer_from(&mut self,
                             from: AccountId,
                             to: AccountId,
                             value: Balance) -> Result<()> {
            let caller = self.env().caller(); // 等价于 Self::env().caller();
            let allownance = self.allowance(from,caller);
            if allownance < value {
                return Err(Error::InsufficientApproval);
            }
            self.inner_transfer(from,to,value)?;

            self.allowances.insert((from,caller), allownance-value);

            Ok(())

        }

        /// 内部转账函数逻辑
        pub fn inner_transfer(&mut self, from: AccountId,
                              to: AccountId,
                              value: Balance
        ) -> Result<()> {
            let from_balance =self.banlance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, from_balance-value);

            ink_env::debug_println!("{:?} lost amount: {}",from,value);

            let to_balance = self.banlance_of(to);
            self.balances.insert(to,to_balance+value);

            ink_env::debug_println!("{:?} get amount: {}",to,value);

            self.env().emit_event(Transfer{
                from: Some(from),
                to: Some(to),
                value
            });

            Ok(())


        }


    }



}

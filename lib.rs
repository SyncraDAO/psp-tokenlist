#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(
    PSP22,
    PSP22Capped,
    PSP22Metadata,
    PSP22Mintable,    
    Ownable,
    AccessControl
)]
#[openbrush::contract]
pub mod psp22_standard {
    use ink::{
        codegen::{EmitEvent, Env},
        prelude::string::String,
        reflect::ContractEventBase
    };
    use inkwhale_project::impls::admin::*;
    use openbrush::{
        contracts::ownable::*,
        contracts::{
            ownable,
            psp22::extensions::{burnable::*, capped::*, metadata::*, mintable::*},
        },
        modifiers,
        traits::{DefaultEnv, Storage},
    };

    use inkwhale_project::impls::{upgradeable::*};
    
    // MINTER RoleType = 4254773782
    pub const MINTER: RoleType = ink::selector_id!("MINTER");
    
    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Psp22Nft {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        cap: capped::Data,
        #[storage_field]
        access: access_control::Data
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }
    
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    pub type Event = <Psp22Nft as ContractEventBase>::Type;

    impl UpgradeableTrait for Psp22Nft {}
    impl AdminTrait for Psp22Nft {}

    #[overrider(psp22::Internal)]
    fn _emit_transfer_event(
        &self,
        from: Option<AccountId>,
        to: Option<AccountId>,
        amount: Balance,
    ) {
        Psp22Nft::emit_event(
            self.env(),
            Event::Transfer(Transfer {
                from,
                to,
                value: amount,
            }),
        );
    }

    #[overrider(psp22::Internal)]
    fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
        Psp22Nft::emit_event(
            self.env(),
            Event::Approval(Approval {
                owner,
                spender,
                value: amount,
            }),
        );
    }

    #[overrider(psp22::Internal)]
    fn _before_token_transfer(
        &mut self,
        from: Option<&AccountId>,
        _to: Option<&AccountId>,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        if from.is_none() && self._is_cap_exceeded(amount) {
            return Err(PSP22Error::Custom(String::from("Cap exceeded")));
        }
        Ok(())
    }

    impl PSP22Burnable for Psp22Nft {
        #[ink(message)]
        fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            let caller = Self::env().caller();
            if account == caller {
                psp22::Internal::_burn_from(self, account, amount)
            } else {
                Err(PSP22Error::Custom(String::from("Your are not owner")))
            }
        }
    }

    #[default_impl(PSP22Mintable)]
    #[modifiers(only_role(MINTER))]
    fn mint() {}

    impl Psp22Nft {
        #[ink(constructor)]
        pub fn new(cap: Balance, name: String, symbol: String, decimal: u8) -> Self {
            let mut instance = Self::default();
            
            let caller = <Self as DefaultEnv>::env().caller();
            ownable::Internal::_init_with_owner(&mut instance, caller);
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
            
            assert!(instance._init_cap(cap).is_ok());
            instance.metadata.name.set(&Some(name));
            instance.metadata.symbol.set(&Some(symbol));
            instance.metadata.decimals.set(&decimal);            
            instance
        }

        pub fn emit_event<EE: EmitEvent<Self>>(emitter: EE, event: Event) {
            emitter.emit_event(event);
        }
    }
}
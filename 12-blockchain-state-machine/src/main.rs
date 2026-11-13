use crate::support::Dispatch;

mod balances;
mod macros;
mod proof_of_existance;
mod support;
mod system;

mod types {
    use super::*;
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u64;
    pub type Nonce = u64;
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
    pub type Content = String;
}

pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistance(proof_of_existance::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
    proof_of_existance: proof_of_existance::Pallet<Runtime>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existance::Config for Runtime {
    type Content = types::Content;
}

impl Dispatch for Runtime {
    type Caller = <Runtime as crate::system::Config>::AccountId;
    type Call = RuntimeCall;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
        match call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
            RuntimeCall::ProofOfExistance(call) => {
                self.proof_of_existance.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existance: proof_of_existance::Pallet::new(),
        }
    }

    pub fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();
        if self.system.block_number() != block.header.block_number {
            return Err("Block number mismatch");
        }

        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(caller.clone());
            let _ = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrisincs Error at\n\tBlock number: {}\n\tExtrisinc: {}\n\tError: {}",
                    self.system.block_number(),
                    i,
                    e
                )
            });
        }
        Ok(())
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let user_1 = account!("Yuriy");
    let user_2 = account!("Evgeny");
    let user_3 = account!("Azat");
    let user_4 = account!("Ivan");
    let user_5 = account!("Sergey");
    let user_6 = account!("Nikita");
    let user_7 = account!("Daniil");
    let user_8 = account!("Sergey");
    let user_9 = account!("Nikita");
    let user_10 = account!("Daniil");

    runtime.balances.set_balance(&user_1, 100);

    let block_1 = block!(
        1,
        extrinsics!(
            user_1.clone(),
            transfer_call!(user_2.clone(), 30),
            user_1.clone(),
            transfer_call!(user_3.clone(), 20)
        )
    );

    let block_2 = block!(
        2,
        extrinsics!(
            user_2.clone(),
            transfer_call!(user_3.clone(), 10),
            user_3.clone(),
            transfer_call!(user_1.clone(), 20)
        )
    );

    // let block_3 = block!(
    //     3,
    //     extrinsics!(
    //         user_1.clone(),
    //         poe_call!(create, "my_document".to_string()),
    //         user_2.clone(),
    //         poe_call!(revoke, "my_document".to_string())
    //     )
    // );

    runtime.execute_block(block_1).unwrap();
    runtime.execute_block(block_2).unwrap();
    // runtime.execute_block(block_3).unwrap();

    println!("{:?}", runtime);
}

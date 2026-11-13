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

pub struct Transaction {
    sender: types::AccountId,
    receiver: types::AccountId,
    amount: types::Balance,
}

pub struct TransactionService {
    runtime: Runtime,
    users: Vec<types::AccountId>,
    queue: Vec<Transaction>,
}

pub fn init_system() -> TransactionService {
    let mut runtime = Runtime::new();
    let user_1 = account!("Yuriy");
    let user_2 = account!("Evgeny");
    let user_3 = account!("Azat");
    let user_4 = account!("Ivan");
    let user_5 = account!("Sergey");
    let user_6 = account!("Nikita");

    runtime.balances.set_balance(&user_1, 100);
    runtime.balances.set_balance(&user_2, 100);
    runtime.balances.set_balance(&user_3, 100);
    runtime.balances.set_balance(&user_4, 100);
    runtime.balances.set_balance(&user_5, 100);
    runtime.balances.set_balance(&user_6, 100);

    TransactionService {
        runtime,
        users: vec![user_1, user_2, user_3, user_4, user_5, user_6],
        queue: vec![],
    }
}

pub fn listener(service: &mut TransactionService, name_sender: String, name_receiver: String, amount: u128) -> Result<(), String> {
    service.queue.push(Transaction {
        sender: name_sender,
        receiver: name_receiver,
        amount,
    });
    
    if service.queue.len() == 5 {
        let block = block!(
            1,
            extrinsics!(
                service.queue[0].sender.clone(),
                transfer_call!(service.queue[0].receiver.clone(), service.queue[0].amount),
                service.queue[1].sender.clone(),
                transfer_call!(service.queue[1].receiver.clone(), service.queue[1].amount),
                service.queue[2].sender.clone(),
                transfer_call!(service.queue[2].receiver.clone(), service.queue[2].amount),
                service.queue[3].sender.clone(),
                transfer_call!(service.queue[3].receiver.clone(), service.queue[3].amount),
                service.queue[4].sender.clone(),
                transfer_call!(service.queue[4].receiver.clone(), service.queue[4].amount)
            )
        );
        
        service.runtime.execute_block(block).map_err(|e| e.to_string())?;
        service.queue.clear();
    }

    Ok(())
}

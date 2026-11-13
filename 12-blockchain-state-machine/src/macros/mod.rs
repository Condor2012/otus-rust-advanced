/// Macro to create extrinsics more concisely
#[macro_export]
macro_rules! extrinsic {
    ($caller:expr, $call:expr) => {
        crate::types::Extrinsic {
            caller: $caller,
            call: $call,
        }
    };
}

/// Macro to create balance transfer calls
#[macro_export]
macro_rules! transfer_call {
    ($to:expr, $amount:expr) => {
        crate::RuntimeCall::Balances(crate::balances::Call::Transfer {
            to: $to,
            amount: $amount,
        })
    };
}

/// Macro to create proof of existence calls
#[macro_export]
macro_rules! poe_call {
    (create, $claim:expr) => {
        crate::RuntimeCall::ProofOfExistance(crate::proof_of_existance::Call::Create {
            claim: $claim,
        })
    };
    (revoke, $claim:expr) => {
        crate::RuntimeCall::ProofOfExistance(crate::proof_of_existance::Call::Revoke {
            claim: $claim,
        })
    };
}

/// Macro to create a block with extrinsics
#[macro_export]
macro_rules! block {
    ($block_number:expr, $extrinsics:expr) => {
        crate::types::Block {
            header: crate::types::Header {
                block_number: $block_number,
            },
            extrinsics: $extrinsics,
        }
    };
}

/// Macro to create multiple extrinsics easily
#[macro_export]
macro_rules! extrinsics {
    ($($caller:expr, $call:expr),* $(,)?) => {
        vec![
            $(
                crate::extrinsic!($caller, $call),
            )*
        ]
    };
}

/// Macro to initialize string accounts
#[macro_export]
macro_rules! account {
    ($name:expr) => {
        $name.to_string()
    };
}

/// Macro to set balance and return the account
#[macro_export]
macro_rules! setup_account {
    ($runtime:expr, $name:expr, $balance:expr) => {{
        let account = $name.to_string();
        $runtime.balances.set_balance(&account, $balance);
        account
    }};
}

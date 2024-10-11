use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiMessage,
};

pub mod utils;

pub trait EncodedConfirmedTransactionMetaInfo {
    fn get_account_list(&self) -> Option<Vec<String>>;
}

impl EncodedConfirmedTransactionMetaInfo for EncodedConfirmedTransactionWithStatusMeta {
    fn get_account_list(&self) -> Option<Vec<String>> {
        match &self.transaction.transaction {
            EncodedTransaction::Json(ui_transaction) => match &ui_transaction.message {
                UiMessage::Parsed(msg) => Some(
                    msg.account_keys
                        .iter()
                        .map(|x| x.pubkey.clone())
                        .collect::<Vec<String>>(),
                ),
                UiMessage::Raw(_) => {
                    unimplemented!()
                }
            },
            _ => {
                unimplemented!()
            }
        }
    }
}

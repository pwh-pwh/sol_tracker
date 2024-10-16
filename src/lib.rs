use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiMessage,
    UiTransactionTokenBalance,
};

pub mod utils;

pub trait EncodedConfirmedTransactionMetaInfo {
    fn get_account_list(&self) -> Option<Vec<String>>;
    fn get_pre_token_balance_info(&self) -> Option<Vec<UiTransactionTokenBalance>>;

    fn get_post_token_balance_info(&self) -> Option<Vec<UiTransactionTokenBalance>>;

    fn get_account_pre_balance(&self) -> Option<u64>;

    fn get_account_post_balance(&self) -> Option<u64>;
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

    fn get_pre_token_balance_info(&self) -> Option<Vec<UiTransactionTokenBalance>> {
        match &self.transaction.meta {
            None => None,
            Some(meta) => match &meta.pre_token_balances {
                OptionSerializer::Some(tb) => if tb.len() == 0 { None }
                else {
                    Some(tb.clone())
                },
                _ => None,
            },
        }
    }

    fn get_post_token_balance_info(&self) -> Option<Vec<UiTransactionTokenBalance>> {
        match &self.transaction.meta {
            None => None,
            Some(meta) => match &meta.post_token_balances {
                OptionSerializer::Some(tb) => if tb.len() == 0 { None }
                else {
                    Some(tb.clone())
                },
                _ => None,
            },
        }
    }

    fn get_account_pre_balance(&self) -> Option<u64> {
        match &self.transaction.meta {
            None => None,
            Some(meta) => meta.pre_balances.first().copied(),
        }
    }

    fn get_account_post_balance(&self) -> Option<u64> {
        match &self.transaction.meta {
            None => None,
            Some(meta) => meta.post_balances.first().copied(),
        }
    }
}

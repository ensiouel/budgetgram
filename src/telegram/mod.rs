use crate::proto::callback::v1::{CreateCategory, UpdateCategory};
use std::collections::HashMap;
use std::sync::Arc;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::macros::BotCommands;
use teloxide::prelude::{ChatId, Dialogue};
use teloxide::types::MessageId;
use tokio::sync::Mutex;

pub type Dialog = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default, Debug)]
pub enum State {
    #[default]
    None,
    UpdateCategory {
        answer_message_id: MessageId,
        callback: UpdateCategory,
    },
    CreateCategory {
        answer_message_id: MessageId,
        callback: CreateCategory,
    },
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "show settings.")]
    Settings,
}
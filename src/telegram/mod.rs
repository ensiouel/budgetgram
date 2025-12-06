use crate::proto::callback::v1::UpdateCategory;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::macros::BotCommands;
use teloxide::prelude::Dialogue;

pub type Dialog = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    None,
    UpdateCategory(UpdateCategory),
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

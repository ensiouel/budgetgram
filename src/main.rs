use budgetgram::handlers::callback::match_callback_query;
use budgetgram::handlers::settings;
use budgetgram::proto::callback::v1::UpdateCategory;
use budgetgram::telegram::{Command, Dialog, HandlerResult, State};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting budgetgram bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        dptree::entry()
            .enter_dialogue::<Update, InMemStorage<State>, State>()
            .branch(
                Update::filter_message()
                    .branch(
                        dptree::entry().filter_command::<Command>().branch(
                            dptree::case![Command::Settings]
                                .endpoint(settings::command_handlers::show_settings),
                        ),
                    )
                    .branch(
                        dptree::case![State::UpdateCategory(update_category)].endpoint(
                            async |bot: Bot,
                                   _dialog: Dialog,
                                   _message: Message,
                                   _update_category: UpdateCategory|
                                   -> HandlerResult { Ok(()) },
                        ),
                    ),
            )
            .branch(Update::filter_callback_query().endpoint(match_callback_query)),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

use std::sync::Arc;
use budgetgram::handlers::callback::match_callback_query;
use budgetgram::handlers::settings;
use budgetgram::proto::callback::v1::UpdateCategory;
use budgetgram::repositories;
use budgetgram::services;
use budgetgram::telegram::{Command, Dialog, HandlerResult, State};
use sqlx::postgres::PgPoolOptions;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting budgetgram bot...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            format!(
                "postgres://{}:{}@{}:{}/{}?sslmode=disable",
                std::env::var("DB_USERNAME").unwrap(),
                std::env::var("DB_PASSWORD").unwrap(),
                std::env::var("DB_HOST").unwrap(),
                std::env::var("DB_PORT").unwrap(),
                std::env::var("DB_DATABASE").unwrap()
            )
            .as_str(),
        )
        .await
        .unwrap();

    let categories_repository = repositories::categories::Categories::new(pool.clone());
    let categories_service = services::categories::Categories::new(categories_repository);

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
    .dependencies(dptree::deps![
        InMemStorage::<State>::new(),
        categories_service as Arc<dyn services::categories::Service>
    ])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

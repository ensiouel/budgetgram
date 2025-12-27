use budgetgram::handlers;
use budgetgram::handlers::callback::match_callback_query;
use budgetgram::handlers::settings;
use budgetgram::repositories;
use budgetgram::services;
use budgetgram::telegram::{Command, State};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    pretty_env_logger::init();

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

    let transactions_repository = repositories::transactions::Transactions::new(pool.clone());
    let transactions_service = services::transactions::Transactions::new(transactions_repository);

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
                        dptree::case![State::CreateCategory {
                            answer_message_id,
                            callback
                        }]
                        .endpoint(handlers::categories::message_handlers::create_category),
                    )
                    .branch(
                        dptree::case![State::None]
                        .endpoint(handlers::transactions::message_handlers::create_transaction),
                    ),
            )
            .branch(Update::filter_callback_query().endpoint(match_callback_query)),
    )
    .dependencies(dptree::deps![
        InMemStorage::<State>::new(),
        categories_service as Arc<dyn services::categories::Service>,
        transactions_service as Arc<dyn services::transactions::Service>
    ])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

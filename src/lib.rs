pub mod proto {
    pub mod callback {
        pub mod v1 {
            include!("gen/callback/v1/callback.v1.rs");
        }
    }
}

pub mod handlers;
pub mod models;
pub mod repositories;
pub mod telegram;
pub mod services;
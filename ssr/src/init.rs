
use std::{
    env,
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use axum_extra::extract::cookie::Key;
use leptos::LeptosOptions;
use leptos_router::RouteListing;

use crate::{
    auth::server_impl::store::KVStoreImpl,
    state::{canisters::Canisters, server::AppState},
};


fn init_cookie_key() -> Key {
    let cookie_key_raw = {
        #[cfg(not(feature = "local-bin"))]
        {
            let cookie_key_str = env::var("COOKIE_KEY").expect("`COOKIE_KEY` is required!");
            hex::decode(cookie_key_str).expect("Invalid `COOKIE_KEY` (must be length 128 hex)")
        }
        #[cfg(feature = "local-bin")]
        {
            use rand_chacha::rand_core::{OsRng, RngCore};
            let mut cookie_key = [0u8; 64];
            OsRng.fill_bytes(&mut cookie_key);
            cookie_key.to_vec()
        }
    };
    Key::from(&cookie_key_raw)
}


pub struct AppStateRes {
    pub app_state: AppState,
   
}

pub struct AppStateBuilder {
    leptos_options: LeptosOptions,
    routes: Vec<RouteListing>,
   
}

impl AppStateBuilder {
    pub fn new(leptos_options: LeptosOptions, routes: Vec<RouteListing>) -> Self {
        Self {
            leptos_options,
            routes,
        }
    }

    async fn init_kv(&mut self) -> KVStoreImpl {
        #[cfg(feature = "redis-kv")]
        {
            use crate::auth::server_impl::store::redis_kv::RedisKV;
            let redis_url: String;
            #[cfg(feature = "local-bin")]
            {
                redis_url = "redis://127.0.0.1:6379".to_string();
            }
            #[cfg(not(feature = "local-bin"))]
            {
                redis_url = env::var("REDIS_URL").expect("`REDIS_URL` is required!");
            }
            KVStoreImpl::Redis(RedisKV::new(&redis_url).await.unwrap())
        }

        #[cfg(not(feature = "redis-kv"))]
        {
            use crate::auth::server_impl::store::redb_kv::ReDBKV;
            KVStoreImpl::ReDB(ReDBKV::new().expect("Failed to initialize ReDB"))
        }
    }

    pub async fn build(mut self) -> AppStateRes {
        let kv = self.init_kv().await;
        

        let app_state = AppState {
            leptos_options: self.leptos_options,
            canisters: Canisters::default(),
            routes: self.routes,
            
            kv,
            cookie_key: init_cookie_key(),
        };

        AppStateRes {
            app_state,
        }
    }
}

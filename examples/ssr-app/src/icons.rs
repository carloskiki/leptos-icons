use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_icons::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::sync::atomic::{AtomicI32, Ordering};
        use broadcaster::BroadcastChannel;
        static COUNT: AtomicI32 = AtomicI32::new(0);

        lazy_static::lazy_static! {
            pub static ref COUNT_CHANNEL: BroadcastChannel<i32> = BroadcastChannel::new();
        }

        pub fn register_server_functions() {
            _ = GetServerIcon::register();
            _ = SetServerIcon::register();
        }

    }
}

#[server(GetServerCount, "/api")]
pub async fn get_server_count() -> Result<i32, ServerFnError> {
    Ok(COUNT.load(Ordering::Relaxed))
}

#[component]
pub fn Icons() -> impl IntoView {
    view! {
        <Icon icon=AiIcon::AiAppleFilled />
    }
}

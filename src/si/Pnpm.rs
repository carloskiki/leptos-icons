#[cfg(feature = "SiPnpm")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiPnpm")]
/// *This icon requires the feature* `SiPnpm` *to be enabled*.
#[component]
pub fn Pnpm(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 0v7.5h7.5V0zm8.25 0v7.5h7.498V0zm8.25 0v7.5H24V0zM8.25 8.25v7.5h7.498v-7.5zm8.25 0v7.5H24v-7.5zM0 16.5V24h7.5v-7.5zm8.25 0V24h7.498v-7.5zm8.25 0V24H24v-7.5z" /></svg>
   }
}
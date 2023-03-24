#[cfg(feature = "ImTwitch")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImTwitch")]
/// *This icon requires the feature* `ImTwitch` *to be enabled*.
#[component]
pub fn Twitch(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M1.5 0l-1.5 2.5v11.5h4v2h2l2-2h2.5l4.5-4.5v-9.5h-13.5zM13 8.5l-2.5 2.5h-2.5l-2 2v-2h-3v-9h10v6.5z" /><path fill="#000000" d="M9.5 4h1.5v4h-1.5v-4z" /><path fill="#000000" d="M6.5 4h1.5v4h-1.5v-4z" /></svg>
   }
}
#[cfg(feature = "ImTextWidth")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImTextWidth")]
/// *This icon requires the feature* `ImTextWidth` *to be enabled*.
#[component]
pub fn TextWidth(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M4 14v2l-3-2.5 3-2.5v2h8v-2l3 2.5-3 2.5v-2zM13 1v4l-1-2h-3v7h2v1h-6v-1h2v-7h-3l-1 2v-4z" /></svg>
   }
}
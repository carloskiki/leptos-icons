#[cfg(feature = "ImPilcrow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImPilcrow")]
/// *This icon requires the feature* `ImPilcrow` *to be enabled*.
#[component]
pub fn Pilcrow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M6 0h8v2h-2v14h-2v-14h-2v14h-2v-8c-2.209 0-4-1.791-4-4s1.791-4 4-4z" /></svg>
   }
}
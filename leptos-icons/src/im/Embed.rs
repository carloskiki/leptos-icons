#[cfg(feature = "ImEmbed")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImEmbed")]
/// *This icon requires the feature* `ImEmbed` *to be enabled*.
#[component]
pub fn Embed(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M9 11.5l1.5 1.5 5-5-5-5-1.5 1.5 3.5 3.5z" /><path fill="#000000" d="M7 4.5l-1.5-1.5-5 5 5 5 1.5-1.5-3.5-3.5z" /></svg>
   }
}
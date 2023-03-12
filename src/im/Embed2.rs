#[cfg(feature = "ImEmbed2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImEmbed2")]
/// *This icon requires the feature* `ImEmbed2` *to be enabled*.
#[component]
pub fn Embed2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="20" height="16" viewBox="0 0 20 16"><path fill="#000000" d="M13 11.5l1.5 1.5 5-5-5-5-1.5 1.5 3.5 3.5z" /><path fill="#000000" d="M7 4.5l-1.5-1.5-5 5 5 5 1.5-1.5-3.5-3.5z" /><path fill="#000000" d="M10.958 2.352l1.085 0.296-3 11-1.085-0.296 3-11z" /></svg>
   }
}
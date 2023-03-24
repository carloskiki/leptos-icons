#[cfg(feature = "ImClearFormatting")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImClearFormatting")]
/// *This icon requires the feature* `ImClearFormatting` *to be enabled*.
#[component]
pub fn ClearFormatting(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 14h9v2h-9zM14 2h-4.727l-2.871 11h-2.067l2.871-11h-4.205v-2h11zM14.528 16l-2.028-2.028-2.028 2.028-0.972-0.972 2.028-2.028-2.028-2.028 0.972-0.972 2.028 2.028 2.028-2.028 0.972 0.972-2.028 2.028 2.028 2.028z" /></svg>
   }
}
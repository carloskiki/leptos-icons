#[cfg(feature = "ImExit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImExit")]
/// *This icon requires the feature* `ImExit` *to be enabled*.
#[component]
pub fn Exit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M12 10v-2h-5v-2h5v-2l3 3zM11 9v4h-5v3l-6-3v-13h11v5h-1v-4h-8l4 2v9h4v-3z" /></svg>
   }
}
#[cfg(feature = "ImEnter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImEnter")]
/// *This icon requires the feature* `ImEnter` *to be enabled*.
#[component]
pub fn Enter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M6 8h-5v-2h5v-2l3 3-3 3zM16 0v13l-6 3v-3h-6v-4h1v3h5v-9l4-2h-9v4h-1v-5z" /></svg>
   }
}
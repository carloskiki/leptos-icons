#[cfg(feature = "ImMenu2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImMenu2")]
/// *This icon requires the feature* `ImMenu2` *to be enabled*.
#[component]
pub fn Menu2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="22" height="16" viewBox="0 0 22 16"><path fill="#000000" d="M0 3h14v3h-14v-3zM0 7h14v3h-14v-3zM0 11h14v3h-14v-3z" /><path fill="#000000" d="M15.5 9l3 3 3-3z" /><path fill="#000000" d="M21.5 8l-3-3-3 3z" /></svg>
   }
}
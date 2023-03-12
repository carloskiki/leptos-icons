#[cfg(feature = "FiUser")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiUser")]
/// *This icon requires the feature* `FiUser` *to be enabled*.
#[component]
pub fn User(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" /><circle cx="12" cy="7" r="4" /></svg>
   }
}
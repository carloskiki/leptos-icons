#[cfg(feature = "FiMusic")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiMusic")]
/// *This icon requires the feature* `FiMusic` *to be enabled*.
#[component]
pub fn Music(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18V5l12-2v13" /><circle cx="6" cy="18" r="3" /><circle cx="18" cy="16" r="3" /></svg>
   }
}
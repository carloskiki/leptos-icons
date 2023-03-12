#[cfg(feature = "FiMessageSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiMessageSquare")]
/// *This icon requires the feature* `FiMessageSquare` *to be enabled*.
#[component]
pub fn MessageSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" /></svg>
   }
}
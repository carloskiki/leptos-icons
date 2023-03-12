#[cfg(feature = "FiLogIn")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiLogIn")]
/// *This icon requires the feature* `FiLogIn` *to be enabled*.
#[component]
pub fn LogIn(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" /><polyline points="10 17 15 12 10 7" /><line x1="15" y1="12" x2="3" y2="12" /></svg>
   }
}
#[cfg(feature = "FiAward")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiAward")]
/// *This icon requires the feature* `FiAward` *to be enabled*.
#[component]
pub fn Award(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="8" r="7" /><polyline points="8.21 13.89 7 23 12 20 17 23 15.79 13.88" /></svg>
   }
}
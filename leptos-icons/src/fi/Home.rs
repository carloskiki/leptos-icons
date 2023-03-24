#[cfg(feature = "FiHome")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiHome")]
/// *This icon requires the feature* `FiHome` *to be enabled*.
#[component]
pub fn Home(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" /><polyline points="9 22 9 12 15 12 15 22" /></svg>
   }
}
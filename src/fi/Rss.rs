#[cfg(feature = "FiRss")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiRss")]
/// *This icon requires the feature* `FiRss` *to be enabled*.
#[component]
pub fn Rss(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 11a9 9 0 0 1 9 9" /><path d="M4 4a16 16 0 0 1 16 16" /><circle cx="5" cy="19" r="1" /></svg>
   }
}
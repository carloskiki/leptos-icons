#[cfg(feature = "FiTrendingDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiTrendingDown")]
/// *This icon requires the feature* `FiTrendingDown` *to be enabled*.
#[component]
pub fn TrendingDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="23 18 13.5 8.5 8.5 13.5 1 6" /><polyline points="17 18 23 18 23 12" /></svg>
   }
}
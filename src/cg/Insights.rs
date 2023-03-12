#[cfg(feature = "CgInsights")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgInsights")]
/// *This icon requires the feature* `CgInsights` *to be enabled*.
#[component]
pub fn Insights(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M15 8H19V20H5V10H9V4H15V8ZM13 6H11V18H13V6ZM15 10V18H17V10H15ZM9 12V18H7V12H9Z" fill="currentColor" /></svg>
   }
}
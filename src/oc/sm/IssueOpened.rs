#[cfg(feature = "OcSmIssueOpened")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmIssueOpened")]
/// *This icon requires the feature* `OcSmIssueOpened` *to be enabled*.
#[component]
pub fn IssueOpened(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M8 9.5a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3Z" /><path d="M8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0ZM1.5 8a6.5 6.5 0 1 0 13 0 6.5 6.5 0 0 0-13 0Z" /></svg>
   }
}
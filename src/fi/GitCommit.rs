#[cfg(feature = "FiGitCommit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiGitCommit")]
/// *This icon requires the feature* `FiGitCommit` *to be enabled*.
#[component]
pub fn GitCommit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4" /><line x1="1.05" y1="12" x2="7" y2="12" /><line x1="17.01" y1="12" x2="22.96" y2="12" /></svg>
   }
}
#[cfg(feature = "BiRegularCollapseAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCollapseAlt")]
/// *This icon requires the feature* `BiRegularCollapseAlt` *to be enabled*.
#[component]
pub fn CollapseAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M2 15h7v7h2v-9H2v2zM15 2h-2v9h9V9h-7V2z" /></svg>
   }
}
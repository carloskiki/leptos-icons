#[cfg(feature = "BiRegularFilter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularFilter")]
/// *This icon requires the feature* `BiRegularFilter` *to be enabled*.
#[component]
pub fn Filter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7 11h10v2H7zM4 7h16v2H4zm6 8h4v2h-4z" /></svg>
   }
}
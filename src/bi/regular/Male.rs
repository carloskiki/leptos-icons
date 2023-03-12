#[cfg(feature = "BiRegularMale")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMale")]
/// *This icon requires the feature* `BiRegularMale` *to be enabled*.
#[component]
pub fn Male(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="4" r="2" /><path d="M15 7H9a1 1 0 0 0-1 1v7h2v7h4v-7h2V8a1 1 0 0 0-1-1z" /></svg>
   }
}
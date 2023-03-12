#[cfg(feature = "BiRegularDialpadAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDialpadAlt")]
/// *This icon requires the feature* `BiRegularDialpadAlt` *to be enabled*.
#[component]
pub fn DialpadAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="6" r="2" /><circle cx="6" cy="6" r="2" /><circle cx="18" cy="6" r="2" /><circle cx="12" cy="12" r="2" /><circle cx="6" cy="12" r="2" /><circle cx="18" cy="12" r="2" /><circle cx="12" cy="18" r="2" /></svg>
   }
}
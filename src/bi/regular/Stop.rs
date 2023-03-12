#[cfg(feature = "BiRegularStop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularStop")]
/// *This icon requires the feature* `BiRegularStop` *to be enabled*.
#[component]
pub fn Stop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7 7h10v10H7z" /></svg>
   }
}
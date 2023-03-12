#[cfg(feature = "BiSolidVideo")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidVideo")]
/// *This icon requires the feature* `BiSolidVideo` *to be enabled*.
#[component]
pub fn Video(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 7c0-1.103-.897-2-2-2H4c-1.103 0-2 .897-2 2v10c0 1.103.897 2 2 2h12c1.103 0 2-.897 2-2v-3.333L22 17V7l-4 3.333V7z" /></svg>
   }
}
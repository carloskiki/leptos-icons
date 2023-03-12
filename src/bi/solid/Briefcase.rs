#[cfg(feature = "BiSolidBriefcase")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBriefcase")]
/// *This icon requires the feature* `BiSolidBriefcase` *to be enabled*.
#[component]
pub fn Briefcase(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 6h-3V4c0-1.103-.897-2-2-2H9c-1.103 0-2 .897-2 2v2H4c-1.103 0-2 .897-2 2v3h20V8c0-1.103-.897-2-2-2zM9 4h6v2H9V4zm5 10h-4v-2H2v7c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2v-7h-8v2z" /></svg>
   }
}
#[cfg(feature = "BiSolidFileBlank")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFileBlank")]
/// *This icon requires the feature* `BiSolidFileBlank` *to be enabled*.
#[component]
pub fn FileBlank(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 2a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8l-6-6H6zm8 7h-1V4l5 5h-4z" /></svg>
   }
}
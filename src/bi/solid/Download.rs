#[cfg(feature = "BiSolidDownload")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDownload")]
/// *This icon requires the feature* `BiSolidDownload` *to be enabled*.
#[component]
pub fn Download(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 9h-4V3H9v6H5l7 8zM4 19h16v2H4z" /></svg>
   }
}
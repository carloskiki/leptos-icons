#[cfg(feature = "BiSolidMessageAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageAlt")]
/// *This icon requires the feature* `BiSolidMessageAlt` *to be enabled*.
#[component]
pub fn MessageAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18.999 2h-14c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h3.5l3.5 4 3.5-4h3.5c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2z" /></svg>
   }
}
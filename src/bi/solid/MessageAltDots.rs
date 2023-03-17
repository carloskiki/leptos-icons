#[cfg(feature = "BiSolidMessageAltDots")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageAltDots")]
/// *This icon requires the feature* `BiSolidMessageAltDots` *to be enabled*.
#[component]
pub fn MessageAltDots(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 2H5c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h3.5l3.5 4 3.5-4H19c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zM9 12a2 2 0 1 1 .001-4.001A2 2 0 0 1 9 12zm6 0a2 2 0 1 1 .001-4.001A2 2 0 0 1 15 12z" /></svg>
   }
}
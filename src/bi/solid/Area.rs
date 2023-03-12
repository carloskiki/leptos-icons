#[cfg(feature = "BiSolidArea")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidArea")]
/// *This icon requires the feature* `BiSolidArea` *to be enabled*.
#[component]
pub fn Area(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 19a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14zm9-13h6v6h-2V8h-4V6zm-6 6h2v4h4v2H6v-6z" /></svg>
   }
}
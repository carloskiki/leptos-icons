#[cfg(feature = "OcSmCodescanCheckmark")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmCodescanCheckmark")]
/// *This icon requires the feature* `OcSmCodescanCheckmark` *to be enabled*.
#[component]
pub fn CodescanCheckmark(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M10.28 6.28a.75.75 0 1 0-1.06-1.06L6.25 8.19l-.97-.97a.75.75 0 0 0-1.06 1.06l1.5 1.5a.75.75 0 0 0 1.06 0l3.5-3.5Z" /><path d="M7.5 15a7.5 7.5 0 1 1 5.807-2.754l2.473 2.474a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215l-2.474-2.473A7.472 7.472 0 0 1 7.5 15Zm0-13.5a6 6 0 1 0 4.094 10.386.748.748 0 0 1 .293-.292 6.002 6.002 0 0 0 1.117-6.486A6.002 6.002 0 0 0 7.5 1.5Z" /></svg>
   }
}
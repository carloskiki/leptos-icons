#[cfg(feature = "BiSolidBookAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBookAlt")]
/// *This icon requires the feature* `BiSolidBookAlt` *to be enabled*.
#[component]
pub fn BookAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 5v14c0 2.201 1.794 3 3 3h15v-2H6.012C5.55 19.988 5 19.806 5 19s.55-.988 1.012-1H21V4c0-1.103-.897-2-2-2H6c-1.206 0-3 .799-3 3z" /></svg>
   }
}
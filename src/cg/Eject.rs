#[cfg(feature = "CgEject")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgEject")]
/// *This icon requires the feature* `CgEject` *to be enabled*.
#[component]
pub fn Eject(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M16.9498 14.3948L18.364 12.9805L12 6.61658L5.63605 12.9805L7.05026 14.3948L12 9.445L16.9498 14.3948Z" fill="currentColor" /><path d="M6.00014 17.3835H18.0001V15.3835H6.00014V17.3835Z" fill="currentColor" /></svg>
   }
}
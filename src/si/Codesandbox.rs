#[cfg(feature = "SiCodesandbox")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiCodesandbox")]
/// *This icon requires the feature* `SiCodesandbox` *to be enabled*.
#[component]
pub fn Codesandbox(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 24H24V0H0V2.45455H21.5455V21.5455H2.45455V0H0Z" /></svg>
   }
}
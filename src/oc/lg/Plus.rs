#[cfg(feature = "OcLgPlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgPlus")]
/// *This icon requires the feature* `OcLgPlus` *to be enabled*.
#[component]
pub fn Plus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11.75 4.5a.75.75 0 0 1 .75.75V11h5.75a.75.75 0 0 1 0 1.5H12.5v5.75a.75.75 0 0 1-1.5 0V12.5H5.25a.75.75 0 0 1 0-1.5H11V5.25a.75.75 0 0 1 .75-.75Z" /></svg>
   }
}
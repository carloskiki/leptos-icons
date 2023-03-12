#[cfg(feature = "SiWindows11")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiWindows11")]
/// *This icon requires the feature* `SiWindows11` *to be enabled*.
#[component]
pub fn Windows11(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0,0H11.377V11.372H0ZM12.623,0H24V11.372H12.623ZM0,12.623H11.377V24H0Zm12.623,0H24V24H12.623" /></svg>
   }
}
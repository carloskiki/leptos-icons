#[cfg(feature = "SiSaltproject")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiSaltproject")]
/// *This icon requires the feature* `SiSaltproject` *to be enabled*.
#[component]
pub fn Saltproject(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 12v10.56h12L18 12h6V1.44H12L6 12Z" /></svg>
   }
}
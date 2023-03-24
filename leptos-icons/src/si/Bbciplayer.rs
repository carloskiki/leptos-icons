#[cfg(feature = "SiBbciplayer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiBbciplayer")]
/// *This icon requires the feature* `SiBbciplayer` *to be enabled*.
#[component]
pub fn Bbciplayer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M8.315 0H2.382v6.022h5.933V3.506l9.618 8.45-9.618 8.538V8.99H2.382V24h5.933l13.303-12.045Z" /></svg>
   }
}
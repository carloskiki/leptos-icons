#[cfg(feature = "SiOracle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiOracle")]
/// *This icon requires the feature* `SiOracle` *to be enabled*.
#[component]
pub fn Oracle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M16.412 4.412h-8.82a7.588 7.588 0 0 0-.008 15.176h8.828a7.588 7.588 0 0 0 0-15.176zm-.193 12.502H7.786a4.915 4.915 0 0 1 0-9.828h8.433a4.914 4.914 0 1 1 0 9.828z" /></svg>
   }
}
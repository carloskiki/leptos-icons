#[cfg(feature = "RiLogosFillPatreon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiLogosFillPatreon")]
/// *This icon requires the feature* `RiLogosFillPatreon` *to be enabled*.
#[component]
pub fn Patreon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M15 17a7.5 7.5 0 1 1 0-15 7.5 7.5 0 0 1 0 15zM2 2h4v20H2V2z" /></g></svg>
   }
}
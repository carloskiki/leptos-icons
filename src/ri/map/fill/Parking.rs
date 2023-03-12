#[cfg(feature = "RiMapFillParking")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMapFillParking")]
/// *This icon requires the feature* `RiMapFillParking` *to be enabled*.
#[component]
pub fn Parking(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path fill-rule="nonzero" d="M6 3h7a6 6 0 1 1 0 12h-3v6H6V3zm4 4v4h3a2 2 0 1 0 0-4h-3z" /></g></svg>
   }
}
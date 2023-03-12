#[cfg(feature = "RiMapLineParking")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMapLineParking")]
/// *This icon requires the feature* `RiMapLineParking` *to be enabled*.
#[component]
pub fn Parking(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M6 3h7a6 6 0 1 1 0 12H8v6H6V3zm2 2v8h5a4 4 0 1 0 0-8H8z" /></g></svg>
   }
}
#[cfg(feature = "BiSolidEject")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidEject")]
/// *This icon requires the feature* `BiSolidEject` *to be enabled*.
#[component]
pub fn Eject(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m12 6-6 8h12zM6 16h12v2H6z" /></svg>
   }
}
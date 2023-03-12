#[cfg(feature = "BiSolidInfoSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidInfoSquare")]
/// *This icon requires the feature* `BiSolidInfoSquare` *to be enabled*.
#[component]
pub fn InfoSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 4v16a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1V4a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1zm8 3h2v2h-2V7zm0 4h2v6h-2v-6z" /></svg>
   }
}
#[cfg(feature = "BiRegularMove")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMove")]
/// *This icon requires the feature* `BiRegularMove` *to be enabled*.
#[component]
pub fn Move(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 11h-5V6h3l-4-4-4 4h3v5H6V8l-4 4 4 4v-3h5v5H8l4 4 4-4h-3v-5h5v3l4-4-4-4z" /></svg>
   }
}
#[cfg(feature = "FaSolidGripLines")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidGripLines")]
/// *This icon requires the feature* `FaSolidGripLines` *to be enabled*.
#[component]
pub fn GripLines(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M32 288c-17.7 0-32 14.3-32 32s14.3 32 32 32l384 0c17.7 0 32-14.3 32-32s-14.3-32-32-32L32 288zm0-128c-17.7 0-32 14.3-32 32s14.3 32 32 32l384 0c17.7 0 32-14.3 32-32s-14.3-32-32-32L32 160z" /></svg>
   }
}
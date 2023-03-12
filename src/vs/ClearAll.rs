#[cfg(feature = "VsClearAll")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsClearAll")]
/// *This icon requires the feature* `VsClearAll` *to be enabled*.
#[component]
pub fn ClearAll(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M10 12.6l.7.7 1.6-1.6 1.6 1.6.8-.7L13 11l1.7-1.6-.8-.8-1.6 1.7-1.6-1.7-.7.8 1.6 1.6-1.6 1.6zM1 4h14V3H1v1zm0 3h14V6H1v1zm8 2.5V9H1v1h8v-.5zM9 13v-1H1v1h8z" /></svg>
   }
}
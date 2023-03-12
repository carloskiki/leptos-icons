#[cfg(feature = "BiRegularSort")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSort")]
/// *This icon requires the feature* `BiRegularSort` *to be enabled*.
#[component]
pub fn Sort(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M8 16H4l6 6V2H8zm6-11v17h2V8h4l-6-6z" /></svg>
   }
}
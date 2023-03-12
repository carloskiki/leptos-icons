#[cfg(feature = "FaSolidDiamond")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidDiamond")]
/// *This icon requires the feature* `FaSolidDiamond` *to be enabled*.
#[component]
pub fn Diamond(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M284.3 11.7c-15.6-15.6-40.9-15.6-56.6 0l-216 216c-15.6 15.6-15.6 40.9 0 56.6l216 216c15.6 15.6 40.9 15.6 56.6 0l216-216c15.6-15.6 15.6-40.9 0-56.6l-216-216z" /></svg>
   }
}
#[cfg(feature = "IoBan")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBan")]
/// *This icon requires the feature* `IoBan` *to be enabled*.
#[component]
pub fn Ban(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><circle fill="none" stroke="#000" stroke-miterlimit="10" stroke-width="48" cx="256" cy="256" r="200" /><line stroke="#000" stroke-miterlimit="10" stroke-width="48" x1="114.58" y1="114.58" x2="397.42" y2="397.42" /></svg>
   }
}
#[cfg(feature = "FaSolidSquareFull")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidSquareFull")]
/// *This icon requires the feature* `FaSolidSquareFull` *to be enabled*.
#[component]
pub fn SquareFull(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M0 0H512V512H0V0z" /></svg>
   }
}
#[cfg(feature = "FaRegularSquareFull")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaRegularSquareFull")]
/// *This icon requires the feature* `FaRegularSquareFull` *to be enabled*.
#[component]
pub fn SquareFull(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M512 0V512H0V0H512zM464 48H48V464H464V48z" /></svg>
   }
}
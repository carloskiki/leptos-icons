#[cfg(feature = "TbMath")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMath")]
/// *This icon requires the feature* `TbMath` *to be enabled*.
#[component]
pub fn Math(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-math" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M19 5h-7l-4 14l-3 -6h-2" /><path d="M14 13l6 6" /><path d="M14 19l6 -6" /></svg>
   }
}
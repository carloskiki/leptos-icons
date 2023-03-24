#[cfg(feature = "TbNumber8")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbNumber8")]
/// *This icon requires the feature* `TbNumber8` *to be enabled*.
#[component]
pub fn Number8(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-number-8" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 8m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /><path d="M12 16m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /></svg>
   }
}
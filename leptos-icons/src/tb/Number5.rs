#[cfg(feature = "TbNumber5")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbNumber5")]
/// *This icon requires the feature* `TbNumber5` *to be enabled*.
#[component]
pub fn Number5(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-number-5" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M8 20h4a4 4 0 1 0 0 -8h-4v-8h8" /></svg>
   }
}
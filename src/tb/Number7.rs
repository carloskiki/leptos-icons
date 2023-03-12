#[cfg(feature = "TbNumber7")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbNumber7")]
/// *This icon requires the feature* `TbNumber7` *to be enabled*.
#[component]
pub fn Number7(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-number-7" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M8 4h8l-4 16" /></svg>
   }
}
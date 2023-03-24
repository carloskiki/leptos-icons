#[cfg(feature = "TbNumber1")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbNumber1")]
/// *This icon requires the feature* `TbNumber1` *to be enabled*.
#[component]
pub fn Number1(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-number-1" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M13 20v-16l-5 5" /></svg>
   }
}
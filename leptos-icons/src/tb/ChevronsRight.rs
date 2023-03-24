#[cfg(feature = "TbChevronsRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbChevronsRight")]
/// *This icon requires the feature* `TbChevronsRight` *to be enabled*.
#[component]
pub fn ChevronsRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-chevrons-right" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7 7l5 5l-5 5" /><path d="M13 7l5 5l-5 5" /></svg>
   }
}
#[cfg(feature = "TbBrandD3")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBrandD3")]
/// *This icon requires the feature* `TbBrandD3` *to be enabled*.
#[component]
pub fn BrandD3(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brand-d3" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 4h1.8c3.976 0 7.2 3.582 7.2 8s-3.224 8 -7.2 8h-1.8" /><path d="M12 4h5.472c1.948 0 3.528 1.79 3.528 4s-1.58 4 -3.528 4" /><path d="M17.472 12h-2.472" /><path d="M17.472 12h-2.352" /><path d="M17.472 12c1.948 0 3.528 1.79 3.528 4s-1.58 4 -3.528 4h-5.472" /></svg>
   }
}
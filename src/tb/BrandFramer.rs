#[cfg(feature = "TbBrandFramer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBrandFramer")]
/// *This icon requires the feature* `TbBrandFramer` *to be enabled*.
#[component]
pub fn BrandFramer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brand-framer" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M6 15h12l-12 -12h12v6h-12v6l6 6v-6" /></svg>
   }
}
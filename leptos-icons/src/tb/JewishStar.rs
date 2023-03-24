#[cfg(feature = "TbJewishStar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbJewishStar")]
/// *This icon requires the feature* `TbJewishStar` *to be enabled*.
#[component]
pub fn JewishStar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-jewish-star" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 2l3 5h6l-3 5l3 5h-6l-3 5l-3 -5h-6l3 -5l-3 -5h6z" /></svg>
   }
}
#[cfg(feature = "TbNorthStar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbNorthStar")]
/// *This icon requires the feature* `TbNorthStar` *to be enabled*.
#[component]
pub fn NorthStar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-north-star" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 12h18" /><path d="M12 21v-18" /><path d="M7.5 7.5l9 9" /><path d="M7.5 16.5l9 -9" /></svg>
   }
}
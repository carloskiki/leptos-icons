#[cfg(feature = "RiMapFillGoblet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMapFillGoblet")]
/// *This icon requires the feature* `RiMapFillGoblet` *to be enabled*.
#[component]
pub fn Goblet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M11 19v-5.111L3 5V3h18v2l-8 8.889V19h5v2H6v-2h5zM7.49 7h9.02l1.8-2H5.69l1.8 2z" /></g></svg>
   }
}
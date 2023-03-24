#[cfg(feature = "TbChartLine")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbChartLine")]
/// *This icon requires the feature* `TbChartLine` *to be enabled*.
#[component]
pub fn ChartLine(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-chart-line" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 19l16 0" /><path d="M4 15l4 -6l4 2l4 -5l4 4" /></svg>
   }
}
#[cfg(feature = "RiBusinessFillBarChartHorizontal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiBusinessFillBarChartHorizontal")]
/// *This icon requires the feature* `RiBusinessFillBarChartHorizontal` *to be enabled*.
#[component]
pub fn BarChartHorizontal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 3v4H3V3h9zm4 14v4H3v-4h13zm6-7v4H3v-4h19z" /></g></svg>
   }
}
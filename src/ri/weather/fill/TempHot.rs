#[cfg(feature = "RiWeatherFillTempHot")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiWeatherFillTempHot")]
/// *This icon requires the feature* `RiWeatherFillTempHot` *to be enabled*.
#[component]
pub fn TempHot(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M8 10.255V5a4 4 0 1 1 8 0v5.255a7 7 0 1 1-8 0zm3 1.871A4.002 4.002 0 0 0 12 20a4 4 0 0 0 1-7.874V5h-2v7.126z" /></g></svg>
   }
}
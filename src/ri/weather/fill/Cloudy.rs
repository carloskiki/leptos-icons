#[cfg(feature = "RiWeatherFillCloudy")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiWeatherFillCloudy")]
/// *This icon requires the feature* `RiWeatherFillCloudy` *to be enabled*.
#[component]
pub fn Cloudy(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M9 20.986a8.5 8.5 0 1 1 7.715-12.983A6.5 6.5 0 0 1 17 20.981V21H9v-.014z" /></g></svg>
   }
}
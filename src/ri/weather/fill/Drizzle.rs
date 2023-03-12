#[cfg(feature = "RiWeatherFillDrizzle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiWeatherFillDrizzle")]
/// *This icon requires the feature* `RiWeatherFillDrizzle` *to be enabled*.
#[component]
pub fn Drizzle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M11 18v3H9v-3a8 8 0 1 1 7.458-10.901A5.5 5.5 0 1 1 17.5 18H11zm2 2h2v3h-2v-3z" /></g></svg>
   }
}
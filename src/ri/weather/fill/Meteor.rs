#[cfg(feature = "RiWeatherFillMeteor")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiWeatherFillMeteor")]
/// *This icon requires the feature* `RiWeatherFillMeteor` *to be enabled*.
#[component]
pub fn Meteor(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M21 1v12A9 9 0 1 1 7.375 5.278L14 1.453v2.77L21 1zm-9 7a5 5 0 1 0 0 10 5 5 0 0 0 0-10z" /></g></svg>
   }
}
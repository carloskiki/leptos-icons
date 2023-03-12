#[cfg(feature = "SiMicrosoft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiMicrosoft")]
/// *This icon requires the feature* `SiMicrosoft` *to be enabled*.
#[component]
pub fn Microsoft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 0v11.408h11.408V0zm12.594 0v11.408H24V0zM0 12.594V24h11.408V12.594zm12.594 0V24H24V12.594z" /></svg>
   }
}
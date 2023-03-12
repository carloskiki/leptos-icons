#[cfg(feature = "SiMicrostrategy")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiMicrostrategy")]
/// *This icon requires the feature* `SiMicrostrategy` *to be enabled*.
#[component]
pub fn Microstrategy(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M9.095 2.572h5.827v18.856H9.096zM0 2.572h5.825v18.856H.001zm18.174 0v18.854H24V8.33z" /></svg>
   }
}
#[cfg(feature = "SiRadar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiRadar")]
/// *This icon requires the feature* `SiRadar` *to be enabled*.
#[component]
pub fn Radar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 0L2.197 23.975 12 19.952 21.803 24z" /></svg>
   }
}
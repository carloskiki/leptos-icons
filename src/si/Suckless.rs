#[cfg(feature = "SiSuckless")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiSuckless")]
/// *This icon requires the feature* `SiSuckless` *to be enabled*.
#[component]
pub fn Suckless(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 4h24v4H4v2h20v10H0v-4h20v-2H0z" /></svg>
   }
}
#[cfg(feature = "SiPlatformdotsh")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiPlatformdotsh")]
/// *This icon requires the feature* `SiPlatformdotsh` *to be enabled*.
#[component]
pub fn Platformdotsh(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M24 0H0v9.541h24V0zM24 20.755H0V24h24v-3.245zM0 12.618h24v4.892H0v-4.892z" /></svg>
   }
}
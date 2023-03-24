#[cfg(feature = "IoLogoHackernews")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLogoHackernews")]
/// *This icon requires the feature* `IoLogoHackernews` *to be enabled*.
#[component]
pub fn LogoHackernews(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M32,32V480H480V32ZM281.67,282.83v84H235v-84l-77-140H213l46.32,97.54,44.33-97.54h52.73Z" /></svg>
   }
}
#[cfg(feature = "SiAsciinema")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiAsciinema")]
/// *This icon requires the feature* `SiAsciinema` *to be enabled*.
#[component]
pub fn Asciinema(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M1.61 0V24L22.39 12L1.61 0M5.76 7.2L10.06 9.68L5.76 12.16V7.2M12.55 11.12L14.08 12L5.76 16.8V15.04L12.55 11.12Z" /></svg>
   }
}
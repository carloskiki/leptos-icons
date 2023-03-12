#[cfg(feature = "SiGitter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiGitter")]
/// *This icon requires the feature* `SiGitter` *to be enabled*.
#[component]
pub fn Gitter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M8.501 4.001H10.5V24H8.501V4.001zm6.999 0V24h-2V4.001h2zM3.5 0h2.001v15H3.5V0zm15 4.001h2V15h-2V4.001z" /></svg>
   }
}
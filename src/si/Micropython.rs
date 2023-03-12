#[cfg(feature = "SiMicropython")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiMicropython")]
/// *This icon requires the feature* `SiMicropython` *to be enabled*.
#[component]
pub fn Micropython(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M0 0h11.509v18.737h.982V0H24v24h-5.263V5.263h-.983V24H6.246V5.263l-.983.035V24H0zm22.246 19.509h-1.404v2.386h1.404z" /></svg>
   }
}
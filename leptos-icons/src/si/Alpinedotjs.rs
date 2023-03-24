#[cfg(feature = "SiAlpinedotjs")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiAlpinedotjs")]
/// *This icon requires the feature* `SiAlpinedotjs` *to be enabled*.
#[component]
pub fn Alpinedotjs(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="m24 12-5.72 5.746-5.724-5.741 5.724-5.75L24 12zM5.72 6.254 0 12l5.72 5.746h11.44L5.72 6.254z" /></svg>
   }
}
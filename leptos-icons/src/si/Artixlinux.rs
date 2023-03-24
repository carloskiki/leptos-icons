#[cfg(feature = "SiArtixlinux")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiArtixlinux")]
/// *This icon requires the feature* `SiArtixlinux` *to be enabled*.
#[component]
pub fn Artixlinux(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 0L7.873 8.462l11.358 6.363zM6.626 11.018L.295 24l18.788-7.762zm13.846 6.352l-5.926 3.402L23.706 24Z" /></svg>
   }
}
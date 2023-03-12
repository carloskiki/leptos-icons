#[cfg(feature = "IoClipboardOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoClipboardOutline")]
/// *This icon requires the feature* `IoClipboardOutline` *to be enabled*.
#[component]
pub fn ClipboardOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M336,64h32a48,48,0,0,1,48,48V432a48,48,0,0,1-48,48H144a48,48,0,0,1-48-48V112a48,48,0,0,1,48-48h32" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><rect x="176" y="32" width="160" height="64" rx="26.13" ry="26.13" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}
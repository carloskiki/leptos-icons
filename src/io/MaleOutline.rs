#[cfg(feature = "IoMaleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoMaleOutline")]
/// *This icon requires the feature* `IoMaleOutline` *to be enabled*.
#[component]
pub fn MaleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="216" cy="296" r="152" fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" /><polyline points="448 160 448 64 352 64" fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" /><line x1="324" y1="188" x2="448" y2="64" fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" /></svg>
   }
}
#[cfg(feature = "WiGaleWarning")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "WiGaleWarning")]
/// *This icon requires the feature* `WiGaleWarning` *to be enabled*.
#[component]
pub fn GaleWarning(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" id="Layer_1" x="0px" y="0px" viewBox="0 0 30 30" style="enable-background:new 0 0 30 30;" xml:space="preserve"><path d="M10.67,24.6V7.45h1.03V24.6H10.67z M12.4,22.44v-7.41l8.65,3.69L12.4,22.44z M12.4,14.86V7.45l8.65,3.69L12.4,14.86z" /></svg>
   }
}
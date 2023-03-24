#[cfg(feature = "ImPowerCord")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImPowerCord")]
/// *This icon requires the feature* `ImPowerCord` *to be enabled*.
#[component]
pub fn PowerCord(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M16 4.414l-1.414-1.414-2.793 2.793-1.586-1.586 2.793-2.793-1.414-1.414-2.793 2.793-1.793-1.793-1.354 1.353 8 8 1.354-1.353-1.793-1.793 2.793-2.793z" /><path fill="#000000" d="M12.407 10.528l-6.935-6.935c-1.497 1.795-3.196 4.57-2.022 6.957l-2.066 2.066c-0.486 0.486-0.486 1.282 0 1.768l0.232 0.232c0.486 0.486 1.282 0.486 1.768 0l2.066-2.066c2.387 1.174 5.161-0.524 6.957-2.022z" /></svg>
   }
}
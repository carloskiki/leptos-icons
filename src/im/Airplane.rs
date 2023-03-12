#[cfg(feature = "ImAirplane")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImAirplane")]
/// *This icon requires the feature* `ImAirplane` *to be enabled*.
#[component]
pub fn Airplane(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M12 9.999l-2.857-2.857 6.857-5.143-2-2-8.571 3.429-2.698-2.699c-0.778-0.778-1.864-0.964-2.414-0.414s-0.364 1.636 0.414 2.414l2.698 2.698-3.429 8.572 2 2 5.144-6.857 2.857 2.857v4h2l1-3 3-1v-2l-4 0z" /></svg>
   }
}
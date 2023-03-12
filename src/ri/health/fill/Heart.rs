#[cfg(feature = "RiHealthFillHeart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiHealthFillHeart")]
/// *This icon requires the feature* `RiHealthFillHeart` *to be enabled*.
#[component]
pub fn Heart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M12.001 4.529c2.349-2.109 5.979-2.039 8.242.228 2.262 2.268 2.34 5.88.236 8.236l-8.48 8.492-8.478-8.492c-2.104-2.356-2.025-5.974.236-8.236 2.265-2.264 5.888-2.34 8.244-.228z" /></g></svg>
   }
}
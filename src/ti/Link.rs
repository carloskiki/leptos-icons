#[cfg(feature = "TiLink")]
use leptos::*;
#[cfg(feature = "TiLink")]
///This icon requires the feature `TiLink` to be enabled.
#[component]
pub fn Link(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.277 6.321c-.43-.43-1.126-.43-1.556 0l-1.721 1.722-.308-.308c-1.168-1.168-3.216-1.168-4.384 0l-4.172 4.172c-.584.584-.906 1.363-.906 2.192s.322 1.608.906 2.192l.308.308-1.722 1.722c-.43.43-.43 1.126 0 1.556.215.215.496.322.778.322s.563-.107.778-.322l1.722-1.722.308.308c.584.584 1.362.906 2.192.906s1.608-.322 2.192-.906l4.172-4.172c.584-.584.906-1.362.906-2.192s-.322-1.608-.906-2.192l-.308-.308 1.722-1.722c.429-.43.429-1.126-.001-1.556zm-2.969 6.414l-4.172 4.172c-.168.168-.402.253-.636.253s-.468-.084-.636-.253l-.308-.308.722-.722c.43-.43.43-1.126 0-1.556-.215-.215-.496-.322-.778-.322s-.563.107-.778.322l-.722.722-.308-.308c-.168-.168-.261-.395-.261-.636s.093-.468.261-.636l4.172-4.172c.168-.168.394-.261.636-.261s.468.093.636.261l.308.308-.722.722c-.43.43-.43 1.126 0 1.556.215.215.496.322.778.322s.563-.107.778-.322l.722-.722.308.308c.168.168.261.395.261.636s-.093.468-.261.636z"
        /> < title > { title } < / title > < / svg >
    }
}

#[cfg(feature = "TiSocialAtCircular")]
use leptos::*;
#[cfg(feature = "TiSocialAtCircular")]
///This icon requires the feature `TiSocialAtCircular` to be enabled.
#[component]
pub fn SocialAtCircular(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M11.844 7.5c-2.481 0-4.438 2.019-4.438 4.5s2.05 4.5 4.531 4.5c.908 0 1.799-.27 2.547-.778.228-.155.295-.466.139-.694-.155-.229-.462-.287-.691-.132-.58.396-1.258.604-1.965.604-1.93 0-3.499-1.57-3.499-3.5s1.446-3.5 3.376-3.5 3.375 1.57 3.375 3.5v.25c0 .414-.336.75-.75.75s-.75-.336-.75-.75v-1.75c0-.276-.099-.5-.375-.5-.205 0-.318.124-.396.301-.303-.188-.628-.301-1.01-.301-1.104 0-1.984.896-1.984 2s.904 2 2.008 2c.562 0 1.073-.235 1.438-.609.319.369.664.609 1.192.609.965 0 1.627-.785 1.627-1.75v-.25c0-2.481-1.894-4.5-4.375-4.5zm.125 5.5c-.551 0-1-.449-1-1s.449-1 1-1 1 .449 1 1-.449 1-1 1zM12 21c-4.963 0-9-4.037-9-9s4.037-9 9-9 9 4.037 9 9-4.037 9-9 9zm0-16c-3.859 0-7 3.141-7 7s3.141 7 7 7 7-3.141 7-7-3.141-7-7-7z"
        /></ g > < title > { title } < / title > < / svg >
    }
}

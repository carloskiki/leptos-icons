#[cfg(feature = "RiLogosLineMedium")]
use leptos::*;
#[cfg(feature = "RiLogosLineMedium")]
///This icon requires the feature `RiLogosLineMedium` to be enabled.
#[component]
pub fn Medium(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M4 3h16a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm1 2v14h14V5H5zm12.3 10.94l.955.954v.05h-4.921v-.05l1.004-.954c.1-.1.15-.2.15-.351V9.664c0-.252 0-.603.051-.904l-3.314 8.285h-.05L7.76 9.412c-.05-.2-.1-.2-.15-.3v5.02c-.051.352 0 .653.15.955l1.356 1.807v.05H5.5v-.05l1.356-1.858c.15-.3.2-.652.15-.954V8.56c0-.251-.05-.553-.25-.753L5.851 6.55V6.5h3.515l2.912 6.478L14.84 6.5h3.415v.05l-.954 1.105c-.1.1-.15.251-.15.351v7.633c0 .1.05.251.15.301z"
        /></ g > < title > { title } < / title > < / svg >
    }
}

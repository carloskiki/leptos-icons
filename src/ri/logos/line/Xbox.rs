#[cfg(feature = "RiLogosLineXbox")]
use leptos::*;
#[cfg(feature = "RiLogosLineXbox")]
///This icon requires the feature `RiLogosLineXbox` to be enabled.
#[component]
pub fn Xbox(
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
        "M4.797 15.485c1.124-2.52 3.2-5.44 4.487-6.962-1.248-1.246-2.162-1.931-2.818-2.3A7.977 7.977 0 0 0 4 12c0 1.25.286 2.432.797 3.485zm4.051-10.84C10.448 5.05 12 5.959 12 5.959v-.005s1.552-.904 3.151-1.31A7.974 7.974 0 0 0 12 4c-1.12 0-2.185.23-3.152.645zm8.686 1.578c-.655.37-1.568 1.055-2.816 2.3 1.287 1.523 3.362 4.441 4.486 6.961A7.968 7.968 0 0 0 20 12c0-2.27-.946-4.32-2.466-5.777zm.408 11.133c-1.403-2.236-4.09-4.944-5.942-6.343-1.85 1.4-4.539 4.108-5.941 6.345A7.98 7.98 0 0 0 12 20a7.98 7.98 0 0 0 5.942-2.644zM12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10z"
        /></ g > < title > { title } < / title > < / svg >
    }
}

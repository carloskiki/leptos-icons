#[cfg(feature = "RiDesignFillContrastDrop2")]
use leptos::*;
#[cfg(feature = "RiDesignFillContrastDrop2")]
///This icon requires the feature `RiDesignFillContrastDrop2` to be enabled.
#[component]
pub fn ContrastDrop2(
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
        "M5.636 6.636L12 .272l6.364 6.364a9 9 0 1 1-12.728 0zM12 3.101L7.05 8.05A6.978 6.978 0 0 0 5 13h14a6.978 6.978 0 0 0-2.05-4.95L12 3.1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}

#[cfg(feature = "RiDesignLineInkBottle")]
use leptos::*;
#[cfg(feature = "RiDesignLineInkBottle")]
///This icon requires the feature `RiDesignLineInkBottle` to be enabled.
#[component]
pub fn InkBottle(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M16 9l4.371 1.749c.38.151.629.52.629.928V21c0 .552-.448 1-1 1H4c-.552 0-1-.448-1-1v-9.323c0-.409.249-.777.629-.928L8 9h8zm-.385 2h-7.23L5 12.354V20h14v-1H8v-5h11v-1.646L15.615 11zM16 3c.552 0 1 .448 1 1v4H7V4c0-.552.448-1 1-1h8zm-1 2H9v1h6V5z"
        /></ g > < title > { title } < / title > < / svg >
    }
}

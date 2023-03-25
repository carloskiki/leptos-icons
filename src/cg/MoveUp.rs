#[cfg(feature = "CgMoveUp")]
use leptos::*;
#[cfg(feature = "CgMoveUp")]
///This icon requires the feature `CgMoveUp` to be enabled.
#[component]
pub fn MoveUp(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 19.071H15V11.071H17V19.071Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 19.071H7V11.071H9V19.071Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.9999 19.071H10.9999V9.07109H7.96454L11.9644 5L15.9644 9.07109H12.9999V19.071Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}

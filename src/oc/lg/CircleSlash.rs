#[cfg(feature = "OcLgCircleSlash")]
use leptos::*;
#[cfg(feature = "OcLgCircleSlash")]
///This icon requires the feature `OcLgCircleSlash` to be enabled.
#[component]
pub fn CircleSlash(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 1c6.075 0 11 4.925 11 11s-4.925 11-11 11S1 18.075 1 12 5.925 1 12 1ZM5.834 19.227A9.464 9.464 0 0 0 12 21.5a9.5 9.5 0 0 0 9.5-9.5 9.464 9.464 0 0 0-2.273-6.166ZM2.5 12a9.464 9.464 0 0 0 2.273 6.166L18.166 4.773A9.463 9.463 0 0 0 12 2.5 9.5 9.5 0 0 0 2.5 12Z"
        /> < title > { title } < / title > < / svg >
    }
}

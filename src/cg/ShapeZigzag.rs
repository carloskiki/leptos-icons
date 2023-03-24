#[cfg(feature = "CgShapeZigzag")]
use leptos::*;
#[cfg(feature = "CgShapeZigzag")]
///This icon requires the feature `CgShapeZigzag` to be enabled.
#[component]
pub fn ShapeZigzag(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M2.31212 9L1 10.5094L4.77355 13.7897L6.28297 15.1018L7.59509 13.5924L9.13456 11.8214L11.3988 13.7897L12.9082 15.1018L14.2203 13.5924L15.7584 11.823L18.0209 13.7897L19.5303 15.1018L20.8424 13.5924L22.8106 11.3283L21.3012 10.0162L19.333 12.2803L15.5594 9L14.2473 10.5094L14.249 10.5109L12.7109 12.2803L8.93736 9L8.05395 10.0163L6.08567 12.2803L2.31212 9Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}

#[cfg(feature = "VsCloud")]
use leptos::*;
#[cfg(feature = "VsCloud")]
///This icon requires the feature `VsCloud` to be enabled.
#[component]
pub fn Cloud(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M11.957 6h.05a2.99 2.99 0 0 1 2.116.879 3.003 3.003 0 0 1 0 4.242 2.99 2.99 0 0 1-2.117.879v-.013L12 12H4.523a3.486 3.486 0 0 1-2.628-1.16 3.502 3.502 0 0 1 1.958-5.78 3.462 3.462 0 0 1 1.468.04 3.486 3.486 0 0 1 3.657-2.06A3.479 3.479 0 0 1 11.957 6zM5 11h7.01a1.994 1.994 0 0 0 1.992-2 2.002 2.002 0 0 0-1.996-2h-.914l-.123-.857a2.49 2.49 0 0 0-2.126-2.122A2.478 2.478 0 0 0 6.231 5.5l-.333.762-.809-.189A2.49 2.49 0 0 0 4.523 6c-.662 0-1.297.263-1.764.732A2.503 2.503 0 0 0 4.523 11H5z"
        /> < title > { title } < / title > < / svg >
    }
}

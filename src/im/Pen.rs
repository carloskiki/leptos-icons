#[cfg(feature = "ImPen")]
use leptos::*;
#[cfg(feature = "ImPen")]
///This icon requires the feature `ImPen` to be enabled.
#[component]
pub fn Pen(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M15.909 4.561l-4.47-4.47c-0.146-0.146-0.338-0.113-0.427 0.073l-0.599 1.248 4.175 4.175 1.248-0.599c0.186-0.089 0.219-0.282 0.073-0.427z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M9.615 2.115l-4.115 0.343c-0.273 0.034-0.501 0.092-0.58 0.449-0 0-0 0.001-0 0.001-1.116 5.36-4.92 10.591-4.92 10.591l0.896 0.896 4.25-4.25c-0.094-0.196-0.146-0.415-0.146-0.647 0-0.828 0.672-1.5 1.5-1.5s1.5 0.672 1.5 1.5-0.672 1.5-1.5 1.5c-0.232 0-0.451-0.053-0.647-0.146l-4.25 4.25 0.896 0.896c0 0 5.231-3.804 10.591-4.92 0-0 0.001-0 0.001-0 0.357-0.078 0.415-0.306 0.449-0.58l0.343-4.115-4.269-4.269z"
        /> < title > { title } < / title > < / svg >
    }
}

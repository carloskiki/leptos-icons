#[cfg(feature = "BiRegularBong")]
use leptos::*;
#[cfg(feature = "BiRegularBong")]
///This icon requires the feature `BiRegularBong` to be enabled.
#[component]
pub fn Bong(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m19.293 8.293-2.069 2.069A7.017 7.017 0 0 0 15 8.681V4h1V2H8v2h1v4.681A7.01 7.01 0 0 0 5 15c0 3.859 3.141 7 7 7s7-3.141 7-7a6.958 6.958 0 0 0-.652-2.934l2.359-2.359-1.414-1.414zm-8.959 1.998.666-.235V4h2v6.056l.666.235A5.006 5.006 0 0 1 16.886 14H7.114a5.006 5.006 0 0 1 3.22-3.709zM12 20a5.007 5.007 0 0 1-4.898-4h9.797A5.009 5.009 0 0 1 12 20z"
        /> < title > { title } < / title > < / svg >
    }
}

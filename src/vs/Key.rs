#[cfg(feature = "VsKey")]
use leptos::*;
#[cfg(feature = "VsKey")]
///This icon requires the feature `VsKey` to be enabled.
#[component]
pub fn Key(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M11.351 1.091a4.528 4.528 0 0 1 3.44 3.16c.215.724.247 1.49.093 2.23a4.583 4.583 0 0 1-4.437 3.6c-.438 0-.874-.063-1.293-.19l-.8.938-.379.175H7v1.5l-.5.5H5v1.5l-.5.5h-3l-.5-.5v-2.307l.146-.353L6.12 6.87a4.464 4.464 0 0 1-.2-1.405 4.528 4.528 0 0 1 5.431-4.375zm1.318 7.2a3.568 3.568 0 0 0 1.239-2.005l.004.005A3.543 3.543 0 0 0 9.72 2.08a3.576 3.576 0 0 0-2.8 3.4c-.01.456.07.908.239 1.33l-.11.543L2 12.404v1.6h2v-1.5l.5-.5H6v-1.5l.5-.5h1.245l.876-1.016.561-.14a3.47 3.47 0 0 0 1.269.238 3.568 3.568 0 0 0 2.218-.795zm-.838-2.732a1 1 0 1 0-1.662-1.11 1 1 0 0 0 1.662 1.11z"
        /> < title > { title } < / title > < / svg >
    }
}

#[cfg(feature = "SiUpcloud")]
use leptos::*;
#[cfg(feature = "SiUpcloud")]
///This icon requires the feature `SiUpcloud` to be enabled.
#[component]
pub fn Upcloud(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M22.1155 12.9964a1.8845 1.8845 0 110 3.769H8.6643V11.87h.888v4.0072h12.5849a1.0078 1.0078 0 00.9964-.9964 1.0317 1.0317 0 00-1.0397-.9964H10.397v-.888zm-8.6859-5.7617H8.6643v1.9927h.888V8.1011h3.899a1.0078 1.0078 0 01.9964.9964 1.0653 1.0653 0 01-1.018 1.0397H1.8845a1.8845 1.8845 0 100 3.769h5.8917v-.8881H1.8845a.9964.9964 0 010-1.9928h11.5668a1.8999 1.8999 0 001.8844-1.8845 1.9375 1.9375 0 00-1.9061-1.9061z"
        /> < title > { title } < / title > < / svg >
    }
}

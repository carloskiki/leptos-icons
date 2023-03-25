#[cfg(feature = "BiRegularCut")]
use leptos::*;
#[cfg(feature = "BiRegularCut")]
///This icon requires the feature `BiRegularCut` to be enabled.
#[component]
pub fn Cut(
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
        "M10 6.5C10 4.57 8.43 3 6.5 3S3 4.57 3 6.5 4.57 10 6.5 10a3.45 3.45 0 0 0 1.613-.413l2.357 2.528-2.318 2.318A3.46 3.46 0 0 0 6.5 14C4.57 14 3 15.57 3 17.5S4.57 21 6.5 21s3.5-1.57 3.5-3.5c0-.601-.166-1.158-.434-1.652l2.269-2.268L17 19.121a3 3 0 0 0 2.121.879H22L9.35 8.518c.406-.572.65-1.265.65-2.018zM6.5 8C5.673 8 5 7.327 5 6.5S5.673 5 6.5 5 8 5.673 8 6.5 7.327 8 6.5 8zm0 11c-.827 0-1.5-.673-1.5-1.5S5.673 16 6.5 16s1.5.673 1.5 1.5S7.327 19 6.5 19z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "m17 4.879-3.707 4.414 1.414 1.414L22 4h-2.879A3 3 0 0 0 17 4.879z" /> < title >
        { title } < / title > < / svg >
    }
}

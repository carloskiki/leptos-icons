#[cfg(feature = "BiLogosMediumOld")]
use leptos::*;
#[cfg(feature = "BiLogosMediumOld")]
///This icon requires the feature `BiLogosMediumOld` to be enabled.
#[component]
pub fn MediumOld(
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
        "M21.957 6.73a.073.073 0 0 0-.019-.097c-.004-.004-.008-.008-.012-.01l-.006-.002-6.033-3.018a.729.729 0 0 0-.292-.067.692.692 0 0 0-.586.317l-3.474 5.646 4.359 7.083 6.063-9.852zm-1.064 13.543c.611.306 1.107.092 1.107-.478V8.483l-5.809 9.439 4.702 2.351zM9.622 8.213v6.426l5.711 2.855zm-7.068 9.558 5.19 2.595a.845.845 0 0 0 .374.098c.323 0 .549-.25.549-.669V6.701a.067.067 0 0 0-.035-.059L2.739 3.697a.669.669 0 0 0-.3-.079c-.258 0-.439.199-.439.534v12.723c0 .341.25.744.554.896z"
        /> < title > { title } < / title > < / svg >
    }
}

#[cfg(feature = "OcLgPackage")]
use leptos::*;
#[cfg(feature = "OcLgPackage")]
///This icon requires the feature `OcLgPackage` to be enabled.
#[component]
pub fn Package(
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
        "M12.876.64V.639l8.25 4.763c.541.313.875.89.875 1.515v9.525a1.75 1.75 0 0 1-.875 1.516l-8.25 4.762a1.748 1.748 0 0 1-1.75 0l-8.25-4.763a1.75 1.75 0 0 1-.875-1.515V6.917c0-.625.334-1.202.875-1.515L11.126.64a1.748 1.748 0 0 1 1.75 0Zm-1 1.298L4.251 6.34l7.75 4.474 7.75-4.474-7.625-4.402a.248.248 0 0 0-.25 0Zm.875 19.123 7.625-4.402a.25.25 0 0 0 .125-.216V7.639l-7.75 4.474ZM3.501 7.64v8.803c0 .09.048.172.125.216l7.625 4.402v-8.947Z"
        /> < title > { title } < / title > < / svg >
    }
}

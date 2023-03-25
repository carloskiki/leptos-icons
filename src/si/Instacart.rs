#[cfg(feature = "SiInstacart")]
use leptos::*;
#[cfg(feature = "SiInstacart")]
///This icon requires the feature `SiInstacart` to be enabled.
#[component]
pub fn Instacart(
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
        "M15.629 9.619c1.421 1.429 2.58 3.766 1.917 5.152-1.778 3.715-15.04 10.226-16.169 9.1C.252 22.746 6.768 9.476 10.481 7.697c1.388-.66 3.724.51 5.152 1.92l-.005.014v-.012zm7.028-1.566c-.231-.855-.821-1.717-1.7-1.82-1.61-.186-4.151 2.663-3.971 3.339.181.69 3.766 1.875 5.1.915.691-.494.781-1.56.556-2.414l.015-.02zM17.666.158c1.198.324 2.407 1.148 2.551 2.382.261 2.259-3.732 5.819-4.68 5.564-.948-.251-2.618-5.284-1.269-7.162.695-.972 2.201-1.106 3.399-.788v.004h-.001z"
        /> < title > { title } < / title > < / svg >
    }
}

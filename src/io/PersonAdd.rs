#[cfg(feature = "IoPersonAdd")]
use leptos::*;
#[cfg(feature = "IoPersonAdd")]
///This icon requires the feature `IoPersonAdd` to be enabled.
#[component]
pub fn PersonAdd(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M288,256c52.79,0,99.43-49.71,104-110.82,2.27-30.7-7.36-59.33-27.12-80.6C345.33,43.57,318,32,288,32c-30.24,0-57.59,11.5-77,32.38-19.63,21.11-29.2,49.8-27,80.78C188.49,206.28,235.12,256,288,256Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M495.38,439.76c-8.44-46.82-34.79-86.15-76.19-113.75C382.42,301.5,335.83,288,288,288s-94.42,13.5-131.19,38c-41.4,27.6-67.75,66.93-76.19,113.75-1.93,10.73.69,21.34,7.19,29.11A30.94,30.94,0,0,0,112,480H464a30.94,30.94,0,0,0,24.21-11.13C494.69,461.1,497.31,450.49,495.38,439.76Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M104,288V248h40a16,16,0,0,0,0-32H104V176a16,16,0,0,0-32,0v40H32a16,16,0,0,0,0,32H72v40a16,16,0,0,0,32,0Z"
        /> < title > { title } < / title > < / svg >
    }
}

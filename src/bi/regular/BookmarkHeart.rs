#[cfg(feature = "BiRegularBookmarkHeart")]
use leptos::*;
#[cfg(feature = "BiRegularBookmarkHeart")]
///This icon requires the feature `BiRegularBookmarkHeart` to be enabled.
#[component]
pub fn BookmarkHeart(
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
        "M20 22V4c0-1.103-.897-2-2-2H6c-1.103 0-2 .897-2 2v18l8-4.572L20 22zM6 10V4h12v14.553l-6-3.428-6 3.428V10z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.409 9.512c.361-.372.585-.888.585-1.456s-.223-1.083-.585-1.456a1.962 1.962 0 0 0-1.412-.603S13.001 5.994 12 7.026c-1.001-1.032-1.997-1.029-1.997-1.029-.552 0-1.051.23-1.412.603-.362.373-.585.887-.585 1.456s.223 1.084.585 1.456L12 13.203l3.409-3.691z"
        /> < title > { title } < / title > < / svg >
    }
}

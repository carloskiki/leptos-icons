#[cfg(feature = "CgExpand")]
use leptos::*;
#[cfg(feature = "CgExpand")]
///This icon requires the feature `CgExpand` to be enabled.
#[component]
pub fn Expand(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.3062 16.5933L12.2713 18.593L5.2724 18.4708L5.39457 11.4719L7.39426 11.5068L7.33168 15.092L15.2262 7.46833L11.6938 7.40668L11.7287 5.40698L18.7277 5.52915L18.6055 12.5281L16.6058 12.4932L16.6693 8.85507L8.72095 16.5307L12.3062 16.5933Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}

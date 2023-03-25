#[cfg(feature = "BiRegularPaperPlane")]
use leptos::*;
#[cfg(feature = "BiRegularPaperPlane")]
///This icon requires the feature `BiRegularPaperPlane` to be enabled.
#[component]
pub fn PaperPlane(
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
        "M20.56,3.34a1,1,0,0,0-1-.08l-17,8a1,1,0,0,0-.57.92,1,1,0,0,0,.6.9L8,15.45v6.72L13.84,18l4.76,2.08a.93.93,0,0,0,.4.09,1,1,0,0,0,.52-.15,1,1,0,0,0,.48-.79l1-15A1,1,0,0,0,20.56,3.34ZM18.1,17.68l-5.27-2.31L16,9.17,8.35,13.42,5.42,12.13,18.89,5.79Z"
        /> < title > { title } < / title > < / svg >
    }
}

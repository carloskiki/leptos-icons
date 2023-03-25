#[cfg(feature = "OcSmFeedStar")]
use leptos::*;
#[cfg(feature = "OcSmFeedStar")]
///This icon requires the feature `OcSmFeedStar` to be enabled.
#[component]
pub fn FeedStar(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M8 16A8 8 0 1 1 8 0a8 8 0 0 1 0 16Zm.252-12.932a.476.476 0 0 0-.682.195l-1.2 2.432-2.684.39a.477.477 0 0 0-.266.816l1.944 1.892-.46 2.674a.479.479 0 0 0 .694.504L8 10.709l2.4 1.261a.478.478 0 0 0 .694-.504l-.458-2.673L12.578 6.9a.479.479 0 0 0-.265-.815l-2.685-.39-1.2-2.432a.473.473 0 0 0-.176-.195Z"
        /> < title > { title } < / title > < / svg >
    }
}

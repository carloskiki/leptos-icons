#[cfg(feature = "CgArrowLongUpR")]
use leptos::*;
#[cfg(feature = "CgArrowLongUpR")]
///This icon requires the feature `CgArrowLongUpR` to be enabled.
#[component]
pub fn ArrowLongUpR(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M7.79337 4.60945L12.0679 0.398956L16.2785 4.67358L14.8536 6.07708L13.0499 4.24596L12.9809 16.1324L16.2068 19.3584L11.9642 23.601L7.72156 19.3584L10.981 16.0989L11.0501 4.20883L9.19686 6.0343L7.79337 4.60945ZM11.9642 20.7726L13.3784 19.3584L11.9642 17.9442L10.55 19.3584L11.9642 20.7726Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}

#[cfg(feature = "IoBagSharp")]
use leptos::*;
#[cfg(feature = "IoBagSharp")]
///This icon requires the feature `IoBagSharp` to be enabled.
#[component]
pub fn BagSharp(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M372,160V148A116.13,116.13,0,0,0,258.89,32c-1,0-1.92,0-2.89,0s-1.93,0-2.89,0A116.13,116.13,0,0,0,140,148v12H52a4,4,0,0,0-4,4V464a16,16,0,0,0,16,16H448a16,16,0,0,0,16-16V164a4,4,0,0,0-4-4Zm-40,0H180V149c0-41.84,33.41-76.56,75.25-77A76.08,76.08,0,0,1,332,148Z"
        /> < title > { title } < / title > < / svg >
    }
}

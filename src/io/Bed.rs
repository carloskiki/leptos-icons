#[cfg(feature = "IoBed")]
use leptos::*;
#[cfg(feature = "IoBed")]
///This icon requires the feature `IoBed` to be enabled.
#[component]
pub fn Bed(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M432,230.7a79.44,79.44,0,0,0-32-6.7H112a79.51,79.51,0,0,0-32,6.69h0A80.09,80.09,0,0,0,32,304V416a16,16,0,0,0,32,0v-8a8.1,8.1,0,0,1,8-8H440a8.1,8.1,0,0,1,8,8v8a16,16,0,0,0,32,0V304A80.09,80.09,0,0,0,432,230.7Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M376,80H136a56,56,0,0,0-56,56v72a4,4,0,0,0,5.11,3.84A95.5,95.5,0,0,1,112,208h4.23a4,4,0,0,0,4-3.55A32,32,0,0,1,152,176h56a32,32,0,0,1,31.8,28.45,4,4,0,0,0,4,3.55h24.46a4,4,0,0,0,4-3.55A32,32,0,0,1,304,176h56a32,32,0,0,1,31.8,28.45,4,4,0,0,0,4,3.55H400a95.51,95.51,0,0,1,26.89,3.85A4,4,0,0,0,432,208V136A56,56,0,0,0,376,80Z"
        /> < title > { title } < / title > < / svg >
    }
}

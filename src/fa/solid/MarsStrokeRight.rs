#[cfg(feature = "FaSolidMarsStrokeRight")]
use leptos::*;
#[cfg(feature = "FaSolidMarsStrokeRight")]
///This icon requires the feature `FaSolidMarsStrokeRight` to be enabled.
#[component]
pub fn MarsStrokeRight(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M192 368a112 112 0 1 0 0-224 112 112 0 1 0 0 224zm174.4-88C354.7 365.8 281.1 432 192 432C94.8 432 16 353.2 16 256S94.8 80 192 80c89.1 0 162.7 66.2 174.4 152H400V176c0-13.3 10.7-24 24-24s24 10.7 24 24v56h32V176c0-9.7 5.8-18.5 14.8-22.2s19.3-1.7 26.2 5.2l80 80c9.4 9.4 9.4 24.6 0 33.9l-80 80c-6.9 6.9-17.2 8.9-26.2 5.2s-14.8-12.5-14.8-22.2V280H448v56c0 13.3-10.7 24-24 24s-24-10.7-24-24V280H366.4z"
        /> < title > { title } < / title > < / svg >
    }
}

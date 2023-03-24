#[cfg(feature = "SiPicartodottv")]
use leptos::*;
#[cfg(feature = "SiPicartodottv")]
///This icon requires the feature `SiPicartodottv` to be enabled.
#[component]
pub fn Picartodottv(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12c6.628 0 12-5.373 12-12S18.628 0 12 0zM7.08 4.182h2.781c.233 0 .42.21.42.47v14.696c0 .26-.187.47-.42.47h-2.78c-.233 0-.42-.21-.42-.47V4.652c0-.26.187-.47.42-.47zm4.664 0a.624.624 0 0 1 .326.091c.355.209 7.451 4.42 8.057 4.78a.604.604 0 0 1 0 1.039c-.436.264-7.558 4.495-8.074 4.789a.577.577 0 0 1-.873-.512v-1.812c0-1.712 2.962-2.201 3.398-2.465a.604.604 0 0 0 0-1.04c-.605-.36-3.398-.746-3.398-2.452V4.79c0-.334.251-.605.564-.61z"
        /> < title > { title } < / title > < / svg >
    }
}

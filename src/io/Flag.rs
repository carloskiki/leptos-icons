#[cfg(feature = "IoFlag")]
use leptos::*;
#[cfg(feature = "IoFlag")]
///This icon requires the feature `IoFlag` to be enabled.
#[component]
pub fn Flag(
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
        "M80,480a16,16,0,0,1-16-16V68.13A24,24,0,0,1,75.9,47.41C88,40.38,112.38,32,160,32c37.21,0,78.83,14.71,115.55,27.68C305.12,70.13,333.05,80,352,80a183.84,183.84,0,0,0,71-14.5,18,18,0,0,1,25,16.58V301.44a20,20,0,0,1-12,18.31c-8.71,3.81-40.51,16.25-84,16.25-24.14,0-54.38-7.14-86.39-14.71C229.63,312.79,192.43,304,160,304c-36.87,0-55.74,5.58-64,9.11V464A16,16,0,0,1,80,480Z"
        /> < title > { title } < / title > < / svg >
    }
}

#[cfg(feature = "SiEventbrite")]
use leptos::*;
#[cfg(feature = "SiEventbrite")]
///This icon requires the feature `SiEventbrite` to be enabled.
#[component]
pub fn Eventbrite(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10.542 5.81c2.653-.6 5.3.487 6.775 2.54L5.591 11c.405-2.479 2.298-4.591 4.951-5.19zm6.84 9.746a6.47 6.47 0 0 1-3.919 2.634c-2.67.604-5.335-.501-6.804-2.582l11.763-2.657 1.915-.433L24 11.691a11.57 11.57 0 0 0-.305-2.333C22.205 3.04 15.76-.9 9.303.558 2.846 2.017-1.18 8.322.31 14.642c1.491 6.319 7.935 10.259 14.392 8.8 3.805-.86 6.765-3.402 8.25-6.638z"
        /> < title > { title } < / title > < / svg >
    }
}

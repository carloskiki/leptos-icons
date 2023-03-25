#[cfg(feature = "RiHealthLineSurgicalMask")]
use leptos::*;
#[cfg(feature = "RiHealthLineSurgicalMask")]
///This icon requires the feature `RiHealthLineSurgicalMask` to be enabled.
#[component]
pub fn SurgicalMask(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0H24V24H0z" />< path d
        =
        "M12.485 3.121l7.758 1.94c.445.11.757.51.757.97V7h1c1.1 0 2 .9 2 2v3c0 1.657-1.343 3-3 3h-.421c-.535 1.35-1.552 2.486-2.896 3.158l-4.789 2.395c-.563.281-1.225.281-1.788 0l-4.79-2.395C4.974 17.486 3.957 16.35 3.422 15H3c-1.657 0-3-1.343-3-3V9c0-1.105.895-2 2-2h1v-.97c0-.458.312-.858.757-.97l7.758-1.939c.318-.08.652-.08.97 0zM12 5.061l-7 1.75v5.98c0 1.516.856 2.9 2.211 3.579L12 18.764l4.789-2.394C18.144 15.692 19 14.307 19 12.792v-5.98l-7-1.75zM3 9H2v3c0 .552.448 1 1 1V9zm19 0h-1v4c.552 0 1-.448 1-1V9z"
        /></ g > < title > { title } < / title > < / svg >
    }
}

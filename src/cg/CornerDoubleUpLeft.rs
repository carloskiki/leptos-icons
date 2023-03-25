#[cfg(feature = "CgCornerDoubleUpLeft")]
use leptos::*;
#[cfg(feature = "CgCornerDoubleUpLeft")]
///This icon requires the feature `CgCornerDoubleUpLeft` to be enabled.
#[component]
pub fn CornerDoubleUpLeft(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.25045 7.78369L7.83624 6.36948L3.59363 10.6121L7.83627 14.8547L9.25048 13.4405L6.42205 10.6121L9.25045 7.78369Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.4932 13.4405L12.0789 14.8547L7.83627 10.6121L12.0789 6.36948L13.4931 7.78369L11.6463 9.63049L16.4063 9.63049C18.6155 9.63049 20.4063 11.4214 20.4063 13.6305L20.4063 17.6305L18.4063 17.6305L18.4063 13.6305C18.4063 12.5259 17.5109 11.6305 16.4063 11.6305L11.6831 11.6305L13.4932 13.4405Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}

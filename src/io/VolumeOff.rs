#[cfg(feature = "IoVolumeOff")]
use leptos::*;
#[cfg(feature = "IoVolumeOff")]
///This icon requires the feature `IoVolumeOff` to be enabled.
#[component]
pub fn VolumeOff(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M344,416a23.92,23.92,0,0,1-14.21-4.69c-.23-.16-.44-.33-.66-.51l-91.46-74.9H168a24,24,0,0,1-24-24V200.07a24,24,0,0,1,24-24h69.65l91.46-74.9c.22-.18.43-.35.66-.51A24,24,0,0,1,368,120V392a24,24,0,0,1-24,24Z"
        /> < title > { title } < / title > < / svg >
    }
}

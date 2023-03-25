#[cfg(feature = "VsVerifiedFilled")]
use leptos::*;
#[cfg(feature = "VsVerifiedFilled")]
///This icon requires the feature `VsVerifiedFilled` to be enabled.
#[component]
pub fn VerifiedFilled(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.38 14.72H7.67L6 13H3.53L3 12.5V10.08L1.31 8.36004V7.65004L3 5.93004V3.50004L3.53 3.00004H6L7.67 1.29004H8.38L10.1 3.00004H12.53L13.03 3.49004V5.93004L14.74 7.65004V8.36004L13 10.08V12.5L12.5 13H10.1L8.38 14.72ZM6.73004 10.4799H7.44004L11.21 6.71L10.5 6L7.09004 9.41991L5.71 8.03984L5 8.74984L6.73004 10.4799Z"
        /> < title > { title } < / title > < / svg >
    }
}

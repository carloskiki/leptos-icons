#[cfg(feature = "SiPowerautomate")]
use leptos::*;
#[cfg(feature = "SiPowerautomate")]
///This icon requires the feature `SiPowerautomate` to be enabled.
#[component]
pub fn Powerautomate(
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
        "m20.079 7.235 3.666 4.097c.34.38.34.956 0 1.336l-7.63 8.528c-.19.213-.462.334-.748.334H7.289l12.79-14.295ZM6.789 21.53H1.005c-.867 0-1.326-1.025-.748-1.671L15.748 2.545c.139.057.265.145.367.259l3.631 4.058a.478.478 0 0 0-.039.039L6.916 21.197a.497.497 0 0 0-.127.333Zm8.356-19.06-8.192 9.155L.257 4.141c-.578-.646-.119-1.671.748-1.671h14.14Z"
        /> < title > { title } < / title > < / svg >
    }
}

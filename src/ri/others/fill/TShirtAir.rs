#[cfg(feature = "RiOthersFillTShirtAir")]
use leptos::*;
#[cfg(feature = "RiOthersFillTShirtAir")]
///This icon requires the feature `RiOthersFillTShirtAir` to be enabled.
#[component]
pub fn TShirtAir(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M12.707 17.793C13.534 18.62 14.295 19 15 19c.378 0 .68-.067 1.237-.276l.392-.152C17.679 18.15 18.209 18 19 18c1.214 0 2.379.545 3.486 1.58l.221.213-1.414 1.414C20.466 20.38 19.705 20 19 20c-.378 0-.68.067-1.237.276l-.392.152c-1.05.421-1.58.572-2.371.572-1.214 0-2.379-.545-3.486-1.58l-.221-.213 1.414-1.414zM9 3a3 3 0 0 0 6 0h6a1 1 0 0 1 1 1v7a1 1 0 0 1-1 1h-9a2 2 0 0 0-1.995 1.85L10 14v7H6a1 1 0 0 1-1-1l-.001-8.001L3 12a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h6zm3.707 10.793C13.534 14.62 14.295 15 15 15c.378 0 .68-.067 1.237-.276l.392-.152C17.679 14.15 18.209 14 19 14c1.214 0 2.379.545 3.486 1.58l.221.213-1.414 1.414C20.466 16.38 19.705 16 19 16c-.378 0-.68.067-1.237.276l-.392.152c-1.05.421-1.58.572-2.371.572-1.214 0-2.379-.545-3.486-1.58l-.221-.213 1.414-1.414z"
        /></ g > < title > { title } < / title > < / svg >
    }
}

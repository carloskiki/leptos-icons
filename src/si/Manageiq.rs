#[cfg(feature = "SiManageiq")]
use leptos::*;
#[cfg(feature = "SiManageiq")]
///This icon requires the feature `SiManageiq` to be enabled.
#[component]
pub fn Manageiq(
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
        "M12.095.1C5.718.094.544 5.26.538 11.637v.022c0 2.069.547 4.005 1.496 5.683l2.869-2.868a7.685 7.685 0 0 1-.54-2.815c0-4.262 3.47-7.73 7.732-7.73s7.732 3.468 7.732 7.73-3.47 7.732-7.732 7.732a7.685 7.685 0 0 1-2.6-.46L6.596 21.83a11.515 11.515 0 0 0 5.499 1.388c2.316 0 4.467-.686 6.275-1.856l2.393 2.392L24 20.512l-2.349-2.349c1.262-1.852 2-4.09 2-6.505C23.66 5.269 18.452.078 12.096.101L12.095.1zm0 9.34c-1.225 0-2.214.991-2.214 2.217s.989 2.215 2.214 2.215a2.216 2.216 0 1 0 0-4.432zm-4.24 3.368C7.57 13.09.273 20.39 0 20.662L3.24 23.9l7.855-7.855-3.24-3.238v.001z"
        /> < title > { title } < / title > < / svg >
    }
}

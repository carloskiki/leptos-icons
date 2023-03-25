#[cfg(feature = "SiIos")]
use leptos::*;
#[cfg(feature = "SiIos")]
///This icon requires the feature `SiIos` to be enabled.
#[component]
pub fn Ios(
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
        "M1.1 6.05C.486 6.05 0 6.53 0 7.13A1.08 1.08 0 0 0 1.1 8.21C1.72 8.21 2.21 7.73 2.21 7.13C2.21 6.53 1.72 6.05 1.1 6.05M8.71 6.07C5.35 6.07 3.25 8.36 3.25 12C3.25 15.67 5.35 17.95 8.71 17.95C12.05 17.95 14.16 15.67 14.16 12C14.16 8.36 12.05 6.07 8.71 6.07M19.55 6.07C17.05 6.07 15.27 7.45 15.27 9.5C15.27 11.13 16.28 12.15 18.4 12.64L19.89 13C21.34 13.33 21.93 13.81 21.93 14.64C21.93 15.6 20.96 16.28 19.58 16.28C18.17 16.28 17.11 15.59 17 14.53H15C15.08 16.65 16.82 17.95 19.46 17.95C22.25 17.95 24 16.58 24 14.4C24 12.69 23 11.72 20.68 11.19L19.35 10.89C17.94 10.55 17.36 10.1 17.36 9.34C17.36 8.38 18.24 7.74 19.54 7.74C20.85 7.74 21.75 8.39 21.85 9.46H23.81C23.76 7.44 22.09 6.07 19.55 6.07M8.71 7.82C10.75 7.82 12.06 9.45 12.06 12C12.06 14.57 10.75 16.2 8.71 16.2C6.65 16.2 5.35 14.57 5.35 12C5.35 9.45 6.65 7.82 8.71 7.82M.111 9.31V17.76H2.1V9.31H.11Z"
        /> < title > { title } < / title > < / svg >
    }
}

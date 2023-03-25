#[cfg(feature = "HiLgOutlineFire")]
use leptos::*;
#[cfg(feature = "HiLgOutlineFire")]
///This icon requires the feature `HiLgOutlineFire` to be enabled.
#[component]
pub fn Fire(
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
        "M15.3622 5.21385C18.2427 6.50093 20.25 9.391 20.25 12.7499C20.25 17.3062 16.5563 20.9999 12 20.9999C7.44365 20.9999 3.75 17.3062 3.75 12.7499C3.75 10.5378 4.62058 8.52914 6.03781 7.0477C6.8043 8.11811 7.82048 8.99755 9.00121 9.60089C9.04632 6.82521 10.348 4.35503 12.3621 2.73438C13.1255 3.75813 14.1379 4.61845 15.3622 5.21385Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 18.0003C14.0711 18.0003 15.75 16.3214 15.75 14.2503C15.75 12.347 14.3321 10.7749 12.4949 10.5327C11.4866 11.4372 10.7862 12.6781 10.5703 14.0789C9.78769 13.8876 9.06529 13.5428 8.43682 13.0782C8.31559 13.4469 8.25 13.841 8.25 14.2503C8.25 16.3214 9.92893 18.0003 12 18.0003Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}

#[cfg(feature = "HiLgOutlineSignal")]
use leptos::*;
#[cfg(feature = "HiLgOutlineSignal")]
///This icon requires the feature `HiLgOutlineSignal` to be enabled.
#[component]
pub fn Signal(
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
        "M9.34835 14.6514C7.88388 13.1869 7.88388 10.8126 9.34835 9.34811M14.6517 9.34811C16.1161 10.8126 16.1161 13.1869 14.6517 14.6514M7.22703 16.7727C4.59099 14.1367 4.59099 9.86283 7.22703 7.22679M16.773 7.22679C19.409 9.86283 19.409 14.1367 16.773 16.7727M5.10571 18.8941C1.2981 15.0864 1.2981 8.91308 5.10571 5.10547M18.8943 5.10547C22.7019 8.91308 22.7019 15.0864 18.8943 18.8941M12 11.9998H12.0075V12.0073H12V11.9998ZM12.375 11.9998C12.375 12.2069 12.2071 12.3748 12 12.3748C11.7929 12.3748 11.625 12.2069 11.625 11.9998C11.625 11.7927 11.7929 11.6248 12 11.6248C12.2071 11.6248 12.375 11.7927 12.375 11.9998Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}

#[cfg(feature = "ImFileZip")]
use leptos::*;
#[cfg(feature = "ImFileZip")]
///This icon requires the feature `ImFileZip` to be enabled.
#[component]
pub fn FileZip(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M14.341 3.579c-0.347-0.473-0.831-1.027-1.362-1.558s-1.085-1.015-1.558-1.362c-0.806-0.591-1.197-0.659-1.421-0.659h-7.75c-0.689 0-1.25 0.561-1.25 1.25v13.5c0 0.689 0.561 1.25 1.25 1.25h11.5c0.689 0 1.25-0.561 1.25-1.25v-9.75c0-0.224-0.068-0.615-0.659-1.421v0 0zM12.271 2.729c0.48 0.48 0.856 0.912 1.134 1.271h-2.406v-2.405c0.359 0.278 0.792 0.654 1.271 1.134v0 0zM14 14.75c0 0.136-0.114 0.25-0.25 0.25h-11.5c-0.135 0-0.25-0.114-0.25-0.25v-13.5c0-0.135 0.115-0.25 0.25-0.25 0 0 7.749-0 7.75 0v3.5c0 0.276 0.224 0.5 0.5 0.5h3.5v9.75z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M4 1h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M6 2h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M4 3h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M6 4h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M4 5h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M6 6h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M4 7h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d = "M6 8h2v1h-2v-1z" />< path
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M4 13.25c0 0.412 0.338 0.75 0.75 0.75h2.5c0.412 0 0.75-0.338 0.75-0.75v-2.5c0-0.412-0.338-0.75-0.75-0.75h-1.25v-1h-2v4.25zM7 12v1h-2v-1h2z"
        /> < title > { title } < / title > < / svg >
    }
}

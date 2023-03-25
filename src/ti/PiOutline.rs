#[cfg(feature = "TiPiOutline")]
use leptos::*;
#[cfg(feature = "TiPiOutline")]
///This icon requires the feature `TiPiOutline` to be enabled.
#[component]
pub fn PiOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.121 7.121c-.566-.567-1.32-.879-2.121-.879s-1.555.312-2.121.879c-.233.233-.546.362-.879.362s-.646-.129-.879-.362c-1.366-1.366-3.185-2.118-5.121-2.118s-3.755.752-5.121 2.118c-.567.567-.879 1.32-.879 2.121s.312 1.555.879 2.121c.566.567 1.32.879 2.121.879v4.758c0 1.654 1.346 3 3 3s3-1.346 3-3c0 1.654 1.346 3 3 3s3-1.346 3-3v-4.166c.784-.356 1.501-.851 2.12-1.47.568-.567.88-1.321.88-2.122s-.312-1.554-.879-2.121zm-1.414 2.828c-.768.767-1.715 1.245-2.707 1.437v5.614c0 .553-.447 1-1 1s-1-.447-1-1v-5.614c-.992-.191-1.939-.67-2.707-1.437-.374-.374-.821-.623-1.293-.775v7.826c0 .553-.447 1-1 1s-1-.447-1-1v-7.827c-.473.152-.919.402-1.293.776-.195.196-.451.293-.707.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414 1.021-1.021 2.364-1.532 3.707-1.532s2.685.511 3.707 1.532c.633.632 1.463.948 2.293.948.831 0 1.661-.316 2.293-.948.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.024 0 1.414z"
        /> < title > { title } < / title > < / svg >
    }
}

#[cfg(feature = "BiRegularAlarmOff")]
use leptos::*;
#[cfg(feature = "BiRegularAlarmOff")]
///This icon requires the feature `BiRegularAlarmOff` to be enabled.
#[component]
pub fn AlarmOff(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m20.292 6.708-3.01-3 1.412-1.417 3.01 3zm1.415 13.585-2.287-2.287C20.409 16.563 21 14.838 21 13c0-4.879-4.121-9-9-9-1.838 0-3.563.591-5.006 1.58L5.91 4.496l.788-.79-1.416-1.412-.786.788-.789-.789-1.414 1.414 18 18 1.414-1.414zM12 6c3.794 0 7 3.206 7 7 0 1.292-.387 2.507-1.027 3.559L15.414 14H17v-2h-3.586L13 11.586V8h-2v1.586L8.441 7.027C9.493 6.387 10.708 6 12 6zM4.305 8.426A8.792 8.792 0 0 0 3 13c0 4.879 4.121 9 9 9a8.792 8.792 0 0 0 4.574-1.305l-1.461-1.461A6.801 6.801 0 0 1 12 20c-3.794 0-7-3.206-7-7 0-1.111.281-2.169.766-3.113L4.305 8.426z"
        /> < title > { title } < / title > < / svg >
    }
}

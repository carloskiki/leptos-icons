#[cfg(feature = "VsGitPullRequestNewChanges")]
use leptos::*;
#[cfg(feature = "VsGitPullRequestNewChanges")]
///This icon requires the feature `VsGitPullRequestNewChanges` to be enabled.
#[component]
pub fn GitPullRequestNewChanges(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M13.7099 4.29L10.7099 1.29L9.99994 1H3.99994L2.99994 2V14L3.99994 15H9.35418C9.03018 14.714 8.75287 14.3764 8.53513 14H3.99994V2H9.99994L12.9999 5V8.126C13.3547 8.21731 13.6904 8.35606 13.9999 8.53509V5L13.7099 4.29ZM8.12602 11H6V12H8C8 11.6547 8.04375 11.3196 8.12602 11ZM6 6H8V4H9V6H11V7H9V9H8V7H6V6Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 9C11.4067 9 10.8266 9.17595 10.3333 9.50559C9.83994 9.83524 9.45543 10.3038 9.22836 10.8519C9.0013 11.4001 8.94189 12.0033 9.05765 12.5853C9.1734 13.1672 9.45912 13.7018 9.87868 14.1213C10.2982 14.5409 10.8328 14.8266 11.4147 14.9424C11.9967 15.0581 12.5999 14.9987 13.1481 14.7716C13.6962 14.5446 14.1648 14.1601 14.4944 13.6667C14.8241 13.1734 15 12.5933 15 12C14.999 11.2047 14.6826 10.4422 14.1202 9.87976C13.5578 9.31736 12.7954 9.00098 12 9Z"
        /> < title > { title } < / title > < / svg >
    }
}

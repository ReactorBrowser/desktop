use iced_wry::WebViewConfig;
use wry::{Rect, WebViewBuilder, dpi};

const ERROR_HTML: &str = r##"
    <div style="font-family: system-ui, -apple-system, sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; background: #1e1e24; color: #ffffff; flex-direction: column; text-align: center; margin: 0;">
        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="#ef4444" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="margin-bottom: 24px;">
            <circle cx="12" cy="12" r="10"></circle>
            <line x1="12" y1="8" x2="12" y2="12"></line>
            <line x1="12" y1="16" x2="12.01" y2="16"></line>
        </svg>
        <h1 style="margin: 0 0 12px 0; font-size: 24px; font-weight: 600;">Page Not Reached</h1>
        <p style="margin: 0 0 32px 0; font-size: 15px; color: #a1a1aa; max-width: 320px; line-height: 1.5;">We couldn't connect to the server. Please check your internet connection or verify the address.</p>
        <button onclick="window.history.back()" style="background: #4f46e5; color: white; border: none; padding: 10px 24px; border-radius: 6px; font-size: 14px; cursor: pointer; font-weight: 500;">Go Back</button>
    </div>
"##;

pub fn create_webview_config(url: &str) -> WebViewConfig {
    let init_script = format!(
        r#"
        (function() {{
            function sendIpc(type, payload) {{
                if (window.ipc && window.ipc.postMessage) {{
                    window.ipc.postMessage(JSON.stringify({{ type, payload }}));
                }}
            }}

            // Custom Error Page Interception
            const href = window.location.href;
            if (href.startsWith('chrome-error://') || href.startsWith('edge://')) {{
                document.documentElement.innerHTML = `{error_html}`;
                sendIpc('load_finish', '');
                return;
            }}

            // Ensure scripts only run in the top-level window frame
            if (window !== window.top) return;

            // Initial Load Signals
            sendIpc('load_start', '');
            sendIpc('url', href);

            // HTML5 History Navigation Hook (SPAs)
            const pushState = history.pushState;
            history.pushState = function() {{
                pushState.apply(history, arguments);
                sendIpc('url', window.location.href);
            }};

            window.addEventListener('popstate', () => {{
                sendIpc('url', window.location.href);
            }});

            // Title Tracking (Fast DOMContentLoaded + MutationObserver)
            window.addEventListener('DOMContentLoaded', () => {{
                sendIpc('title', document.title);

                const titleEl = document.querySelector('title');
                if (titleEl) {{
                    let lastTitle = document.title;
                    new MutationObserver(() => {{
                        if (document.title !== lastTitle) {{
                            lastTitle = document.title;
                            sendIpc('title', document.title);
                        }}
                    }}).observe(titleEl, {{ childList: true, characterData: true, subtree: true }});
                }}
            }});

            window.addEventListener('load', () => {{
                sendIpc('url', window.location.href);
                sendIpc('title', document.title);
                sendIpc('load_finish', '');
            }});

            // Fallback safety timeout for SPAs or stalled loads
            setTimeout(() => {{
                sendIpc('title', document.title);
                sendIpc('load_finish', '');
            }}, 10000);
        }})();
        "#,
        error_html = ERROR_HTML
    );

    WebViewConfig::default()
        .url(url)
        .devtools(true)
        .customize(move |builder| {
            *builder = std::mem::replace(builder, WebViewBuilder::new())
                .with_bounds(Rect {
                    position: dpi::LogicalPosition::new(-10000.0, -10000.0).into(),
                    size: dpi::LogicalSize::new(1.0, 1.0).into(),
                })
                .with_initialization_script(&init_script);
        })
}

use dioxus::document;
use dioxus::prelude::*;
use std::rc::Rc;

#[cfg(feature = "web")]
use wasm_bindgen::JsCast;

const BUUP_ICON_SVG: Asset = asset!("assets/buup-icon.svg");
const APPLE_TOUCH_ICON: Asset = asset!("assets/apple-touch-icon.png");
const FAVICON_32: Asset = asset!("assets/favicon-32x32.png");
const FAVICON_16: Asset = asset!("assets/favicon-16x16.png");
const SITE_MANIFEST: Asset = asset!("assets/site.webmanifest");

fn main() {
    dioxus::launch(App);
}

// Components
#[component]
fn App() -> Element {
    // Read preferences from localStorage during initialization
    #[cfg(feature = "web")]
    let initial_theme = {
        use js_sys::{global, Function, Object};
        use wasm_bindgen::JsCast;

        let localStorage = js_sys::Reflect::get(&global(), &"localStorage".into())
            .ok()
            .and_then(|val| val.dyn_into::<Object>().ok());

        if let Some(storage) = localStorage {
            let get_item = js_sys::Reflect::get(&storage, &"getItem".into())
                .ok()
                .and_then(|val| val.dyn_into::<Function>().ok());

            if let Some(get_fn) = get_item {
                let theme_result = get_fn.call1(&storage, &"buup_dark_mode".into());
                if let Ok(theme_val) = theme_result {
                    if !theme_val.is_null() {
                        let theme_str = theme_val.as_string().unwrap_or_default();
                        theme_str == "true"
                    } else {
                        // Default to system preference if not found
                        js_sys::eval("window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches")
                            .map(|v| v.as_bool().unwrap_or(false))
                            .unwrap_or(false)
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    };

    #[cfg(not(feature = "web"))]
    let initial_theme = false;

    // Read the initially saved transformer ID if any
    #[cfg(feature = "web")]
    let initial_transformer_id = {
        use js_sys::{global, Function, Object};
        use wasm_bindgen::JsCast;

        let localStorage = js_sys::Reflect::get(&global(), &"localStorage".into())
            .ok()
            .and_then(|val| val.dyn_into::<Object>().ok());

        if let Some(storage) = localStorage {
            let get_item = js_sys::Reflect::get(&storage, &"getItem".into())
                .ok()
                .and_then(|val| val.dyn_into::<Function>().ok());

            if let Some(get_fn) = get_item {
                let id_result = get_fn.call1(&storage, &"buup_transformer_id".into());
                if let Ok(id_val) = id_result {
                    if !id_val.is_null() {
                        id_val
                            .as_string()
                            .unwrap_or_else(|| "base64encode".to_string())
                    } else {
                        "base64encode".to_string()
                    }
                } else {
                    "base64encode".to_string()
                }
            } else {
                "base64encode".to_string()
            }
        } else {
            "base64encode".to_string()
        }
    };

    #[cfg(not(feature = "web"))]
    let initial_transformer_id = "base64encode".to_string();

    // Initialize signals with saved values
    let mut is_dark_mode = use_signal(|| initial_theme);
    let mut input = use_signal(|| "Hello, world!".to_string());
    let mut current_transformer = use_signal(|| {
        Rc::new(
            buup::transformer_from_id(&initial_transformer_id)
                .unwrap_or_else(|_| buup::transformer_from_id("base64encode").unwrap()),
        )
    });
    let mut show_transformer_menu = use_signal(|| false);
    let mut transformer_category = use_signal(|| "all");
    let mut search_query = use_signal(String::new);
    let mut show_copy_success = use_signal(|| false);

    // Initialize categories signal with values from the core library
    let categorized = buup::categorized_transformers();
    let transformers = buup::all_transformers();

    // Get all categories for the menu
    let mut categories: Vec<_> = categorized.keys().collect();
    // Sort categories alphabetically for consistent ordering
    categories.sort_by_key(|c| c.to_string());

    // Save preferences when they change
    use_effect(move || {
        #[cfg(feature = "web")]
        {
            let dark_mode = is_dark_mode();
            let transformer_id = current_transformer().id();

            let js_code = format!(
                r#"
                try {{
                    localStorage.setItem('buup_dark_mode', '{}');
                    localStorage.setItem('buup_transformer_id', '{}');
                }} catch (e) {{
                    console.error('Failed to save preferences:', e);
                }}
                "#,
                dark_mode, transformer_id
            );

            let _ = js_sys::eval(&js_code);
        }
    });

    // Add JavaScript click handler for closing menu when clicking outside
    #[cfg(feature = "web")]
    use_effect(move || {
        // Set up the click outside detection with plain JavaScript
        let js_code = r#"
            // Define a global click handler function
            function handleOutsideClick(event) {
                // Only run if the menu is open
                const menu = document.querySelector('.transformer-menu');
                if (!menu) return;
                
                const button = document.querySelector('.current-transformer');
                
                // If the click is outside both the menu and button, close the menu
                if (!menu.contains(event.target) && !button.contains(event.target)) {
                    // Create a custom event for closing the menu
                    document.dispatchEvent(new CustomEvent('buup:close-menu'));
                }
            }
            
            // Attach the handler to the document
            document.addEventListener('mousedown', handleOutsideClick);
        "#;

        // Run the script once to set up the event listener
        let _ = js_sys::eval(js_code);

        // Add a handler to close the menu on the custom event
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                // Create a closure that will close the transformer menu
                let mut menu_signal = show_transformer_menu;
                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                    menu_signal.set(false);
                })
                    as Box<dyn FnMut()>);

                // Attach the event listener
                let _ = document.add_event_listener_with_callback(
                    "buup:close-menu",
                    closure.as_ref().unchecked_ref(),
                );

                // Prevent the closure from being dropped
                closure.forget();
            }
        }
    });

    // Apply transformation and get output
    let output = match current_transformer().transform(&input()) {
        Ok(result) => result,
        Err(err) => err.to_string(),
    };

    // Clone output for use in the clipboard function
    let output_for_clipboard = output.clone();

    // Function to copy to clipboard
    let copy_to_clipboard = move |_| {
        // Show the success indicator
        show_copy_success.set(true);

        // Copy to clipboard using a simple JS function
        #[cfg(feature = "web")]
        {
            // Create a JavaScript function to copy text
            let js_code = format!(
                r#"
                (function() {{
                    // Create temporary textarea
                    const el = document.createElement('textarea');
                    el.value = {};
                    el.style.position = 'absolute';
                    el.style.left = '-9999px';
                    document.body.appendChild(el);
                    el.select();
                    document.execCommand('copy');
                    document.body.removeChild(el);
                    return true;
                }})()
                "#,
                serde_json::to_string(&output_for_clipboard).unwrap()
            );

            // Execute the JS function
            let _ = js_sys::eval(&js_code);
        }

        // Reset the success indicator after 2 seconds
        let mut success_copy = show_copy_success;
        let timeout_callback = move || {
            success_copy.set(false);
        };

        #[cfg(feature = "web")]
        {
            // Set a timeout to hide the success message after 2 seconds
            gloo_timers::callback::Timeout::new(2000, timeout_callback).forget();
        }
    };

    // Theme colors
    let theme = if is_dark_mode() {
        Theme {
            bg: "#000000",
            surface: "#1C1C1E",
            text: "#FFFFFF",
            text_secondary: "rgba(255, 255, 255, 0.7)",
            border: "#38383A",
            accent: "#0A84FF",
            hover: "#2C2C2E",
        }
    } else {
        Theme {
            bg: "#FFFFFF",
            surface: "#F5F5F7",
            text: "#000000",
            text_secondary: "rgba(0, 0, 0, 0.7)",
            border: "#D2D2D7",
            accent: "#0066CC",
            hover: "#E8E8ED",
        }
    };

    // Function to swap between encoder/decoder pairs
    let swap_transform = move |_| {
        if let Some(inverse) = buup::inverse_transformer(&**current_transformer()) {
            // First, get the current output value
            let current_output = match current_transformer().transform(&input()) {
                Ok(result) => result,
                Err(err) => err.to_string(),
            };

            // Set the output as the new input
            input.set(current_output);

            // Switch to the inverse transformer
            current_transformer.set(Rc::new(inverse));
        }
    };

    // Filter transformers based on selected category and search query
    let filtered_transformers = {
        let category_filtered = if transformer_category() == "all" {
            transformers.to_vec()
        } else {
            // Parse the category string to TransformerCategory enum
            if let Ok(category) = transformer_category().parse::<buup::TransformerCategory>() {
                // Get transformers for the selected category
                categorized.get(&category).unwrap().clone()
            } else {
                // Fallback to all transformers if category parsing fails
                transformers.to_vec()
            }
        };

        // If search query is empty, show all transformers for the selected category
        if search_query().is_empty() {
            category_filtered
        } else {
            // Filter transformers based on search query (match name or description)
            let search_lower = search_query().to_lowercase();
            category_filtered
                .into_iter()
                .filter(|transformer| {
                    transformer.name().to_lowercase().contains(&search_lower)
                        || transformer
                            .description()
                            .to_lowercase()
                            .contains(&search_lower)
                        || transformer.id().to_lowercase().contains(&search_lower)
                })
                .collect::<Vec<_>>()
        }
    };

    rsx! {
        document::Title { "Buup - Text Utility Belt" }
        document::Link {
            rel: "apple-touch-icon",
            sizes: "180x180",
            href: APPLE_TOUCH_ICON
        }
        document::Link {
            rel: "icon",
            type: "image/png",
            sizes: "32x32",
            href: FAVICON_32
        }
        document::Link {
            rel: "icon",
            type: "image/png",
            sizes: "16x16",
            href: FAVICON_16
        }
        document::Link {
            rel: "manifest",
            href: SITE_MANIFEST
        }
        document::Meta {
            name: "apple-mobile-web-app-capable",
            content: "yes"
        }
        document::Meta {
            name: "apple-mobile-web-app-status-bar-style",
            content: "default"
        }
        document::Meta {
            name: "apple-mobile-web-app-title",
            content: "Buup"
        }
        document::Meta {
            name: "theme-color",
            content: "#FFE0E9"
        }

        // The main app content starts here
        style { {format!(r#"
            * {{ 
                margin: 0; 
                padding: 0; 
                box-sizing: border-box; 
                -webkit-font-smoothing: antialiased;
                -moz-osx-font-smoothing: grayscale;
            }}
            
            @keyframes fadeIn {{
                from {{ opacity: 0; }}
                to {{ opacity: 1; }}
            }}
            
            @keyframes slideIn {{
                from {{ transform: translateY(-10px); opacity: 0; }}
                to {{ transform: translateY(0); opacity: 1; }}
            }}
            
            body {{ 
                font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "SF Pro Icons", "Helvetica Neue", sans-serif;
                background: {bg}; 
                color: {text}; 
                transition: background 0.3s ease, color 0.3s ease;
                min-height: 100vh;
                height: 100vh;
                width: 100vw;
                overflow: hidden;
            }}
            
            button, select, input {{
                font-family: inherit;
                font-size: inherit;
            }}
            
            .container {{ 
                max-width: 1200px; 
                margin: 0 auto; 
                padding: 2rem;
                height: 100vh;
                display: flex;
                flex-direction: column;
            }}
            
            .header {{ 
                display: flex; 
                justify-content: space-between; 
                align-items: center;
                margin-bottom: 2rem;
                animation: fadeIn 0.5s ease;
            }}
            
            .app-title {{ 
                font-size: 1.5rem; 
                font-weight: 600; 
                letter-spacing: -0.02em;
                display: flex;
                align-items: center;
            }}
            
            .app-title img {{
                transition: transform 0.3s ease;
            }}
            
            .app-title:hover img {{
                transform: rotate(10deg);
            }}
            
            .controls {{ 
                display: flex; 
                gap: 0.75rem;
            }}
            
            .icon-button {{ 
                background: transparent;
                color: {text};
                border: none;
                width: 2.5rem;
                height: 2.5rem;
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
                cursor: pointer;
                transition: background 0.2s ease;
                font-size: 1.2rem;
            }}
            
            .icon-button:hover {{ 
                background: {hover};
            }}
            
            .transformer-selector {{ 
                position: relative;
                width: 100%;
                margin-bottom: 1.5rem;
                animation: slideIn 0.5s ease;
            }}
            
            .current-transformer {{ 
                display: flex;
                align-items: center;
                justify-content: space-between;
                background: {surface};
                border: 1px solid {border};
                border-radius: 0.75rem;
                padding: 1rem 1.25rem;
                cursor: pointer;
                transition: border-color 0.2s ease, background 0.2s ease;
            }}
            
            .current-transformer:hover {{ 
                border-color: {accent};
            }}
            
            .transformer-name {{ 
                font-size: 1.125rem;
                font-weight: 500;
            }}
            
            .transformer-description {{ 
                font-size: 0.875rem;
                color: {text_secondary};
                margin-top: 0.25rem;
            }}
            
            .arrow-icon {{ 
                font-size: 1rem;
                transition: transform 0.3s ease;
            }}
            
            .arrow-icon.open {{ 
                transform: rotate(180deg);
            }}
            
            .transformer-menu {{ 
                position: absolute;
                top: calc(100% + 0.5rem);
                left: 0;
                right: 0;
                background: {surface};
                border: 1px solid {border};
                border-radius: 0.75rem;
                box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
                z-index: 10;
                max-height: 400px;
                overflow-y: auto;
                animation: fadeIn 0.2s ease;
            }}
            
            .search-container {{
                padding: 0.75rem 1rem;
                border-bottom: 1px solid {border};
            }}
            
            .search-input {{
                width: 100%;
                padding: 0.6rem 1rem;
                border-radius: 0.5rem;
                border: 1px solid {border};
                background: {bg};
                color: {text};
                font-size: 0.9rem;
                transition: border-color 0.2s ease, box-shadow 0.2s ease;
                outline: none;
            }}
            
            .search-input:focus {{
                border-color: {accent};
                box-shadow: 0 0 0 2px rgba(10, 132, 255, 0.3);
            }}
            
            .transformer-categories {{ 
                display: flex;
                overflow-x: auto;
                padding: 0.75rem 1rem;
                border-bottom: 1px solid {border};
                gap: 0.5rem;
            }}
            
            .category-button {{ 
                padding: 0.5rem 0.75rem;
                background: transparent;
                border: none;
                border-radius: 1rem;
                font-size: 0.875rem;
                color: {text_secondary};
                cursor: pointer;
                white-space: nowrap;
                transition: background 0.2s ease, color 0.2s ease;
            }}
            
            .category-button:hover {{ 
                background: {hover};
            }}
            
            .category-button.active {{ 
                background: {accent};
                color: white;
            }}
            
            .transformer-list {{ 
                padding: 0.5rem;
            }}
            
            .transformer-option {{ 
                padding: 0.75rem 1rem;
                cursor: pointer;
                border-radius: 0.5rem;
                transition: background 0.2s ease;
            }}
            
            .transformer-option:hover {{ 
                background: {hover};
            }}
            
            .transformer-option.active {{ 
                background: {hover};
            }}
            
            .option-name {{ 
                font-weight: 500;
                margin-bottom: 0.25rem;
            }}
            
            .option-description {{ 
                font-size: 0.75rem;
                color: {text_secondary};
            }}
            
            .no-results {{
                padding: 1rem;
                text-align: center;
                color: {text_secondary};
                font-size: 0.9rem;
            }}
            
            .panels {{ 
                display: grid;
                grid-template-columns: 1fr auto 1fr;
                gap: 1rem; /* Consistent gap for both desktop and mobile */
                flex: 1;
                min-height: 0;
                animation: slideIn 0.7s ease;
                align-items: stretch; /* Stretch children to full height */
            }}
            
            .panel {{ 
                flex: 1; 
                background: {surface}; 
                border-radius: 0.75rem; 
                border: 1px solid {border}; 
                display: flex; 
                flex-direction: column;
                max-height: 700px; /* Increased from 500px */
                overflow: hidden;
            }}
            
            .panel-header {{ 
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 0.75rem 1rem;
                border-bottom: 1px solid {border};
            }}
            
            .panel-title {{ 
                font-size: 0.875rem;
                font-weight: 500;
            }}
            
            .panel-actions {{ 
                display: flex;
                gap: 0.5rem;
            }}
            
            .action-button {{ 
                border: none;
                background: transparent;
                color: {text_secondary};
                cursor: pointer;
                width: 1.75rem;
                height: 1.75rem;
                display: flex;
                align-items: center;
                justify-content: center;
                border-radius: 0.375rem;
                transition: background 0.2s ease, color 0.2s ease;
            }}
            
            .action-button:hover {{ 
                background: {hover};
                color: {text};
            }}
            
            .copy-button {{ 
                background: transparent;
                color: {text_secondary};
                border: none;
                display: flex;
                align-items: center;
                justify-content: center;
                width: 2rem;
                height: 2rem;
                border-radius: 4px;
                cursor: pointer;
                transition: all 0.2s ease;
                position: relative;
            }}
            
            .copy-button:hover {{ 
                color: {accent};
                background-color: rgba(0, 0, 0, 0.03);
            }}
            
            .dark .copy-button:hover {{
                background-color: rgba(255, 255, 255, 0.05);
            }}
            
            .copy-button:active {{ 
                transform: scale(0.95);
            }}
            
            .copy-button svg {{
                width: 18px;
                height: 18px;
                transition: fill 0.2s ease;
            }}
            
            .copy-button svg path {{
                fill: {text_secondary};
            }}
            
            .copy-button:hover svg path {{
                fill: {accent};
            }}
            
            .dark .copy-button svg path {{
                fill: rgba(255, 255, 255, 0.7);
            }}
            
            .dark .copy-button:hover svg path {{
                fill: {accent};
            }}
            
            .copy-success {{
                position: absolute;
                top: 10px;
                right: 10px;
                background: {accent};
                color: white;
                font-size: 0.75rem;
                padding: 0.25rem 0.5rem;
                border-radius: 4px;
                opacity: 0;
                transform: translateY(10px);
                transition: opacity 0.2s ease, transform 0.2s ease;
                pointer-events: none;
                white-space: nowrap;
                z-index: 10;
            }}
            
            .copy-success.visible {{
                opacity: 1;
                transform: translateY(0);
            }}
            
            .textarea-container {{ 
                flex: 1; 
                position: relative;
                display: flex;
                overflow: hidden;
                max-height: 650px; /* Increased from 450px */
            }}
            
            .textarea {{ 
                width: 100%; 
                height: 100%;
                padding: 1rem; 
                background: transparent; 
                border: none; 
                color: {text}; 
                resize: none; 
                font-family: ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, Liberation Mono, monospace;
                font-size: 0.9rem; 
                line-height: 1.5; 
                overflow-y: auto;
                flex: 1;
            }}
            
            .swap-button-container {{ 
                display: flex; 
                justify-content: center;
                align-items: center;
                width: 80px; 
                flex-shrink: 0;
                max-height: 700px; /* Increased from 500px to match panels */
                align-self: stretch; /* Stretch to full height of row */
            }}
            
            .swap-button {{ 
                display: flex;
                align-items: center;
                justify-content: center;
                background: {surface};
                color: {accent};
                border: 1px solid {border};
                border-radius: 0.75rem;
                padding: 0 1rem; 
                margin: 0;
                cursor: pointer;
                font-size: 1.5rem;
                transition: background 0.2s ease, color 0.2s ease, border-color 0.2s ease;
                min-width: 3rem;
                width: 50px; /* Fixed width for desktop */
                height: 100%; /* Full height button */
            }}
            
            .swap-button:hover {{ 
                background: {accent};
                color: white;
                border-color: {accent};
            }}
            
            .swap-button:active {{ 
                transform: scale(0.98);
            }}
            
            .placeholder {{ 
                position: absolute;
                top: 1rem;
                left: 1rem;
                color: {text_secondary};
                pointer-events: none;
                transition: opacity 0.2s ease;
                opacity: 0;
                font-family: "SF Mono", "Menlo", monospace;
                font-size: 0.9375rem;
            }}
            
            textarea:placeholder-shown + .placeholder {{ 
                opacity: 1; 
            }}
            
            /* Scrollbar styles */
            ::-webkit-scrollbar {{ width: 8px; height: 8px; }}
            ::-webkit-scrollbar-track {{ background: transparent; }}
            ::-webkit-scrollbar-thumb {{ 
                background: {border}; 
                border-radius: 4px; 
            }}
            ::-webkit-scrollbar-thumb:hover {{ background: {text_secondary}; }}
            
            @media (max-width: 768px) {{
                .container {{ 
                    padding: 1rem; 
                }}
                
                /* Restructure panels for mobile */
                .panels {{ 
                    display: flex;
                    flex-direction: column;
                    gap: 1rem; /* Consistent gap value */
                }}
                
                /* Make panels larger on mobile */
                .panel {{
                    min-height: 200px; /* Add minimum height for panels on mobile */
                }}
                
                /* Make swap button full width on mobile */
                .swap-button-container {{
                    width: 100%; /* Full width on mobile */
                    height: 48px; /* Fixed height on mobile */
                }}
                
                .swap-button {{ 
                    width: 100%; /* Full width button on mobile */
                    height: 48px; /* Match container height */
                    border-radius: 0.75rem;
                    font-size: 1.5rem;
                }}
            }}
            
            /* Footer styles */
            .footer {{
                margin-top: 1.5rem;
                padding: 1rem 0;
                font-size: 0.875rem;
                color: {text_secondary};
                text-align: center;
                border-top: 1px solid {border};
            }}
            
            .footer a {{
                color: {accent};
                text-decoration: none;
                transition: opacity 0.2s ease;
            }}
            
            .footer a:hover {{
                opacity: 0.8;
            }}
            
            .heart {{
                color: #e25555;
                display: inline-block;
                margin: 0 0.2rem;
            }}
        "#, bg = theme.bg, text = theme.text, surface = theme.surface, 
            text_secondary = theme.text_secondary, border = theme.border,
            accent = theme.accent, hover = theme.hover)} }

        div { class: if is_dark_mode() { "container dark" } else { "container" },
            // Header section
            div { class: "header",
                div { class: "app-title",
                    img {
                        src: BUUP_ICON_SVG,
                        alt: "Buup logo",
                        style: "width: 30px; height: 30px; margin-right: 10px;"
                    }
                    "Buup"
                }
                div { class: "controls",
                    button {
                        class: "icon-button",
                        onclick: move |_| is_dark_mode.set(!is_dark_mode()),
                        if is_dark_mode() { "☀️" } else { "🌙" }
                    }
                }
            }

            // Transformer selector
            div { class: "transformer-selector",
                div {
                    class: "current-transformer",
                    tabindex: "0",
                    onclick: move |evt| {
                        // Toggle menu visibility
                        show_transformer_menu.set(!show_transformer_menu());

                        // Prevent default to ensure click works properly on mobile
                        evt.stop_propagation();

                        #[cfg(feature = "web")]
                        {
                            // If opening the menu, focus the search input
                            if !show_transformer_menu() {
                                // Use JS to ensure the button stays focusable
                                let js_code = r#"
                                    setTimeout(() => {
                                        const button = document.querySelector('.current-transformer');
                                        if (button) {
                                            button.focus();
                                        }
                                    }, 10);
                                "#;
                                let _ = js_sys::eval(js_code);
                            } else {
                                // If opening the menu, focus the search input
                                let js_code = r#"
                                    setTimeout(() => {
                                        const searchInput = document.querySelector('.search-input');
                                        if (searchInput) {
                                            searchInput.focus();
                                        }
                                    }, 10);
                                "#;
                                let _ = js_sys::eval(js_code);
                            }
                        }
                    },

                    div {
                        div { class: "transformer-name", "{current_transformer().name()}" }
                        div { class: "transformer-description", "{current_transformer().description()}" }
                    }

                    div {
                        class: if show_transformer_menu() { "arrow-icon open" } else { "arrow-icon" },
                        "▼"
                    }
                }

                // Dropdown menu for transformer selection
                {if show_transformer_menu() {
                    rsx! {
                        div {
                            class: "transformer-menu",
                            tabindex: "0",
                            onblur: move |evt| {
                                // Only close if the related target is not within the menu structure
                                #[cfg(feature = "web")]
                                {
                                    let js_code = r#"
                                        const menu = document.querySelector('.transformer-menu');
                                        const button = document.querySelector('.current-transformer');
                                        const related = document.activeElement;
                                        
                                        // Only close if focus is moving outside our components
                                        if (menu && button && related) {
                                            if (!menu.contains(related) && !button.contains(related)) {
                                                return true; // Close the menu
                                            }
                                        }
                                        return false; // Keep menu open
                                    "#;

                                    if let Ok(result) = js_sys::eval(js_code) {
                                        if let Some(close) = result.as_bool() {
                                            if close {
                                                show_transformer_menu.set(false);
                                            }
                                        }
                                    }
                                }

                                #[cfg(not(feature = "web"))]
                                {
                                    show_transformer_menu.set(false);
                                }

                                evt.stop_propagation();
                            },

                            // Search input
                            div { class: "search-container",
                                input {
                                    class: "search-input",
                                    r#type: "text",
                                    placeholder: "Search transformations...",
                                    value: "{search_query}",
                                    oninput: move |evt| search_query.set(evt.value().clone()),
                                    autofocus: true,
                                    onmounted: move |_| {
                                        #[cfg(feature = "web")]
                                        {
                                            // Focus the search input when mounted
                                            let js_code = r#"
                                                setTimeout(() => {
                                                    const searchInput = document.querySelector('.search-input');
                                                    if (searchInput) {
                                                        searchInput.focus();
                                                    }
                                                }, 50);
                                            "#;
                                            let _ = js_sys::eval(js_code);
                                        }
                                    },
                                }
                            }

                            // Categories
                            div { class: "transformer-categories",
                                button {
                                    class: if transformer_category() == "all" { "category-button active" } else { "category-button" },
                                    onclick: move |_| transformer_category.set("all"),
                                    "All"
                                }
                                {categories.iter().map(|category| {
                                    let category_str = category.to_string();
                                    rsx! {
                                        button {
                                            class: if transformer_category() == category_str { "category-button active" } else { "category-button" },
                                            onclick: move |_| {
                                                // Use a match on the actual string value
                                                let cat_str = match &*category_str {
                                                    "encoders" => "encoders",
                                                    "decoders" => "decoders",
                                                    "crypto" => "crypto",
                                                    "formatters" => "formatters",
                                                    "compression" => "compression",
                                                    "others" => "others",
                                                    _ => "all", // Fallback
                                                };
                                                transformer_category.set(cat_str);
                                            },
                                            "{&category.to_string()}"
                                        }
                                    }
                                })}
                            }

                            // Transformer list
                            div { class: "transformer-list",
                                {if filtered_transformers.is_empty() {
                                    rsx! {
                                        div { class: "no-results",
                                            "No transformations found"
                                        }
                                    }
                                } else {
                                    rsx! {
                                        {filtered_transformers.iter().map(|transformer| {
                                            let id = transformer.id();
                                            let name = transformer.name();
                                            let description = transformer.description();
                                            let is_current = current_transformer().id() == id;

                                            rsx! {
                                                div {
                                                    key: "{id}",
                                                    class: if is_current { "transformer-option active" } else { "transformer-option" },
                                                    onclick: move |evt| {
                                                        current_transformer.set(Rc::new(buup::transformer_from_id(id).unwrap()));
                                                        show_transformer_menu.set(false);
                                                        search_query.set(String::new());

                                                        // Stop event propagation to prevent issues
                                                        evt.stop_propagation();

                                                        #[cfg(feature = "web")]
                                                        {
                                                            // Use JS to ensure focus returns to the transformer selector
                                                            let js_code = r#"
                                                                setTimeout(() => {
                                                                    const button = document.querySelector('.current-transformer');
                                                                    if (button) {
                                                                        button.focus();
                                                                    }
                                                                }, 10);
                                                            "#;
                                                            let _ = js_sys::eval(js_code);
                                                        }
                                                    },

                                                    div { class: "option-name", "{name}" }
                                                    div { class: "option-description", "{description}" }
                                                }
                                            }
                                        })}
                                    }
                                }}
                            }
                        }
                    }
                } else {
                    rsx! {}
                }}
            }

            // Input/Output panels
            div { class: "panels",
                // Input panel
                div { class: "panel",
                    div { class: "panel-header",
                        div { class: "panel-title", "Input" }
                        div { class: "panel-actions",
                            button {
                                class: "action-button",
                                title: "Clear input",
                                onclick: move |_| input.set("".to_string()),
                                "✕"
                            }
                        }
                    }
                    div { class: "textarea-container",
                        textarea {
                            class: "textarea",
                            value: "{input}",
                            oninput: move |evt| input.set(evt.value().clone()),
                            placeholder: "",
                        }
                        div { class: "placeholder", "Enter text to transform..." }
                    }
                }

                // Swap button
                div { class: "swap-button-container",
                    button {
                        class: "swap-button",
                        onclick: swap_transform,
                        title: "Swap transformation",
                        "⇄"
                    }
                }

                // Output panel
                div { class: "panel",
                    div { class: "panel-header",
                        div { class: "panel-title", "Output" }
                        div { class: "panel-actions",
                            button {
                                class: "copy-button",
                                title: "Copy to clipboard",
                                onclick: copy_to_clipboard,
                                // Add data attribute for clipboard.js to use
                                "data-clipboard-text": "{output}",
                                svg {
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path {
                                        d: "M16 1H4C2.9 1 2 1.9 2 3V17H4V3H16V1ZM19 5H8C6.9 5 6 5.9 6 7V21C6 22.1 6.9 23 8 23H19C20.1 23 21 22.1 21 21V7C21 5.9 20.1 5 19 5ZM19 21H8V7H19V21Z"
                                    }
                                }
                                div {
                                    class: if show_copy_success() { "copy-success visible" } else { "copy-success" },
                                    "Copied!"
                                }
                            }
                        }
                    }
                    div { class: "textarea-container",
                        textarea {
                            class: "textarea",
                            value: "{output}",
                            readonly: true,
                            placeholder: "",
                        }
                        div { class: "placeholder", "Output will appear here..." }
                    }
                }
            }

            // Footer
            div { class: "footer",
                span {
                    {"Made with "}
                    span { class: "heart", "❤" }
                    {" by "}
                    a {
                        href: "https://benletchford.com",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "Ben Letchford"
                    }
                    {" "}
                }
                span {
                    a {
                        href: format!("{}/releases/tag/v{}", env!("CARGO_PKG_REPOSITORY"), env!("CARGO_PKG_VERSION")),
                        target: "_blank",
                        rel: "noopener noreferrer",
                        {format!("({}-{})", env!("CARGO_PKG_VERSION"), env!("BUUP_WEB_GIT_HASH"))}
                    }
                }
            }
        }
    }
}

struct Theme {
    bg: &'static str,
    surface: &'static str,
    text: &'static str,
    text_secondary: &'static str,
    border: &'static str,
    accent: &'static str,
    hover: &'static str,
}

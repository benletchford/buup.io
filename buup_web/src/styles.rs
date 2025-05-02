use crate::Theme;

pub fn generate_css(theme: &Theme) -> String {
    format!(
        r#"
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
                min-height: 100svh; /* Use small viewport height to account for mobile toolbars */
                width: 100vw;
            }}
            
            button, select, input {{
                font-family: inherit;
                font-size: inherit;
            }}
            
            .container {{ 
                max-width: 1200px; 
                margin: 0 auto; 
                padding: 2rem;
                min-height: 100svh;
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
                max-height: 650px; /* Keep max-height for desktop */
            }}
            
            .textarea {{ 
                width: 100%; 
                max-height: 100%;
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
                    /* Ensure container still respects viewport height */
                    min-height: 100svh; 
                }}
                
                /* Restructure panels for mobile */
                .panels {{ 
                    display: flex;
                    flex-direction: column;
                    gap: 1rem; /* Consistent gap value */
                    flex: 1; /* Ensure panels try to fill remaining space */
                    min-height: 0; /* Allow panels container to shrink */
                }}
                
                /* Make panels larger on mobile */
                .panel {{
                    /* Removed min-height: 200px; Allow panels to shrink based on content */
                    max-height: none; /* Remove desktop max-height on mobile */
                    flex-shrink: 1; /* Allow panels to shrink if needed */
                    min-height: 100px; /* Add a smaller min-height for better structure */
                }}

                .textarea-container {{
                    max-height: none; /* Remove desktop max-height */
                    min-height: 50px; /* Ensure textarea is at least minimally visible */
                    overflow: auto; /* Ensure scrolling within container is possible */
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
        "#,
        bg = theme.bg,
        text = theme.text,
        surface = theme.surface,
        text_secondary = theme.text_secondary,
        border = theme.border,
        accent = theme.accent,
        hover = theme.hover
    )
}

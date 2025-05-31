use gtk::CssProvider;

pub fn setup_styling() {
    let provider = CssProvider::new();
    provider.load_from_string(
        "
        @keyframes fadeIn {
            from { opacity: 0; transform: scale(0.95); }
            to { opacity: 1; transform: scale(1.0); }
        }
        
        window {
            background: linear-gradient(135deg,
                rgb(15, 15, 20) 0%,
                rgb(25, 25, 35) 50%,
                rgb(20, 20, 30) 100%);
            border-radius: 16px;
            border: 2px solid rgb(80, 80, 100);
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
            animation: fadeIn 0.2s ease-out;
        }
        
        window.csd {
            border-radius: 16px;
            border: 2px solid rgb(80, 80, 100);
        }
        
        .title-text {
            color: #f1ff5e;
            font-size: 12pt;
            font-weight: 700;
            margin: 8px 0;
            text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
        }
        
        button {
            background: linear-gradient(135deg,
                rgb(40, 40, 55) 0%,
                rgb(55, 55, 75) 50%,
                rgb(45, 45, 65) 100%);
            border: 1px solid rgb(90, 90, 110);
            border-radius: 12px;
            color: rgb(255, 255, 255);
            font-weight: 600;
            font-size: 10px;
            margin: 3px;
            padding: 8px 6px;
            min-width: 85px;
            min-height: 45px;
            transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
            text-shadow: 0 1px 2px rgba(0, 0, 0, 0.7);
        }
        
        button:hover {
            background: linear-gradient(135deg,
                rgb(60, 60, 85) 0%,
                rgb(75, 75, 105) 50%,
                rgb(65, 65, 95) 100%);
            border-color: rgb(120, 120, 140);
            transform: translateY(-1px);
            box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
            color: rgb(255, 255, 255);
        }
        
        button:active {
            background: linear-gradient(135deg,
                rgb(80, 80, 105) 0%,
                rgb(95, 95, 125) 50%,
                rgb(85, 85, 115) 100%);
            transform: translateY(0px) scale(0.98);
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
        }
        
        button:focus {
            outline: 2px solid #1aff28;
            outline-offset: 2px;
        }
        
        .shortcut-hint {
            margin-top: 2px;
            font-size: 11px;
        }
        
        .help-label {
            color: rgba(200, 200, 200, 0.8);
            font-size: 11px;  /* Increased from 8px to 11px */
            margin: 6px 0;
        }
        
        .help-content {
            color: rgba(220, 220, 220, 0.95);
            font-size: 11px;  /* Increased from 10px to 11px (+1pt) */
            margin: 15px;
            font-family: monospace;
        }
    ",
    );

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

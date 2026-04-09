use dioxus::document;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TripCreationInput {
    pub name: String,
    pub date_range: String,
}

pub async fn prompt_trip_creation(default_name: &str) -> Option<TripCreationInput> {
    let default_name_js = serde_json::to_string(default_name).ok()?;

    let script = format!(
        r#"
        (async () => {{
          const fallback = {default_name_js};
          const overlay = document.createElement('div');
          overlay.style.cssText = 'position:fixed;inset:0;background:rgba(0,0,0,0.45);display:flex;align-items:center;justify-content:center;z-index:99999;padding:16px;';

          const card = document.createElement('div');
          card.style.cssText = 'width:100%;max-width:380px;background:#fff;border-radius:14px;padding:16px;box-shadow:0 10px 40px rgba(0,0,0,0.25);font-family:-apple-system,BlinkMacSystemFont,Segoe UI,Roboto,sans-serif;';
          card.innerHTML = `
            <div style="font-size:16px;font-weight:600;margin-bottom:10px;">New Trip</div>
            <label style="display:block;font-size:12px;color:#666;margin-bottom:4px;">Trip name</label>
            <input id="trip-name" type="text" value="${{fallback.replace(/"/g, '&quot;')}}" style="width:100%;box-sizing:border-box;padding:10px;border:1px solid #ddd;border-radius:8px;margin-bottom:10px;" />

            <label style="display:block;font-size:12px;color:#666;margin-bottom:4px;">Start date</label>
            <input id="trip-start" type="date" style="width:100%;box-sizing:border-box;padding:10px;border:1px solid #ddd;border-radius:8px;margin-bottom:10px;" />

            <label style="display:block;font-size:12px;color:#666;margin-bottom:4px;">End date</label>
            <input id="trip-end" type="date" style="width:100%;box-sizing:border-box;padding:10px;border:1px solid #ddd;border-radius:8px;margin-bottom:14px;" />

            <div style="display:flex;gap:8px;justify-content:flex-end;">
              <button id="trip-cancel" style="padding:8px 12px;border:1px solid #ddd;background:#fff;border-radius:8px;">Cancel</button>
              <button id="trip-create" style="padding:8px 12px;border:1px solid #0ea5e9;background:#0ea5e9;color:#fff;border-radius:8px;">Create</button>
            </div>
          `;

          overlay.appendChild(card);
          document.body.appendChild(overlay);

          const cleanup = () => {{
            if (overlay.parentNode) overlay.parentNode.removeChild(overlay);
          }};

          const formatDateRange = (start, end) => {{
            if (!start || !end) return 'Dates TBD';
            const fmt = new Intl.DateTimeFormat('en-US', {{ month: 'short', day: 'numeric', year: 'numeric' }});
            const s = new Date(start + 'T00:00:00');
            const e = new Date(end + 'T00:00:00');
            if (Number.isNaN(s.getTime()) || Number.isNaN(e.getTime())) return 'Dates TBD';
            return `${{fmt.format(s)}} - ${{fmt.format(e)}}`;
          }};

          const resolveCancel = () => {{
            cleanup();
            dioxus.send('');
          }};

          overlay.addEventListener('click', (ev) => {{
            if (ev.target === overlay) resolveCancel();
          }});

          card.querySelector('#trip-cancel')?.addEventListener('click', resolveCancel);

          card.querySelector('#trip-create')?.addEventListener('click', () => {{
            const name = (card.querySelector('#trip-name')?.value || '').trim() || fallback;
            const start = (card.querySelector('#trip-start')?.value || '').trim();
            const end = (card.querySelector('#trip-end')?.value || '').trim();
            const date_range = formatDateRange(start, end);

            cleanup();
            dioxus.send(JSON.stringify({{ name, date_range }}));
          }});
        }})();
        "#
    );

    let mut eval = document::eval(&script);
    let raw = eval.recv::<String>().await.ok()?;
    if raw.trim().is_empty() {
        return None;
    }

    serde_json::from_str::<TripCreationInput>(&raw).ok()
}

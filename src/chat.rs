use std::{error::Error, time::Duration};
use headless_chrome::browser;

pub fn get_chats(tab: &browser::Tab, num: i64) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    println!("Searching for chat...");
    let search_bar = tab.wait_for_element(".x1hx0egp")?;
    search_bar.click()?;
    search_bar.type_into(&num.to_string())?;

    // Wait for search results
    std::thread::sleep(Duration::from_secs(2));

    tab.evaluate(r#"
        function getElementHeight(selector) {
            const element = document.querySelector(selector);
            return element ? element.getBoundingClientRect().height : 0;
        }

        const x = getElementWidth('header');
        const y = getElementHeight('._ai07._aoq2._ai01');
        const z = getElementHeight('.x1pi30zi');
        const a = getElementHeight('.x10l6tqk.xh8yej3.x1g42fcv');

        function getElementWidth(selector) {
            const element = document.querySelector(selector);
            return element ? element.getBoundingClientRect().width : 0;
        }

        const width = getElementHeight('header')/2;

        const b = x + y + z + a + 30;
        const div = document.elementFromPoint(width , b);
        div.focus();
        console.log(div)
        // Simulate a mouse click
        div.dispatchEvent(new MouseEvent('mousedown', { bubbles: true, clientX: x, clientY: y }));
        div.dispatchEvent(new MouseEvent('mouseup', { bubbles: true, clientX: x, clientY: y }));
        div.dispatchEvent(new MouseEvent('click', { bubbles: true, clientX: x, clientY: y }));
    "#, true)?;

    let chatEle = tab.wait_for_element_with_custom_timeout("._amjv", Duration::from_millis(160000));
    match chatEle {
        Ok(element) => {
            println!("Successfully Accessed Chats");
        },
        Err(e) => {
            println!("{}", e);
        }
    }

    let eval_result = tab.evaluate(r#"
    (function extractMessages() {
        try {
            const parentDiv = document.querySelector('div.x3psx0u.xwib8y2.xkhd6sd.xrmvbpv');
            const messages = [];
            
            if (!parentDiv) {
                console.error('Parent div not found!');
                return JSON.stringify({ error: 'Parent div not found' });
            }
            
            const childRows = parentDiv.querySelectorAll('div[role="row"]');
            
            if (childRows.length > 0) {
                childRows.forEach((row) => {
                    const messageIn = row.querySelector('div.message-in');
                    const messageOut = row.querySelector('div.message-out');
                    const messageType = messageIn ? 'receive' : messageOut ? 'send' : null;
                    
                    const targetSpan = row.querySelector('span._ao3e.copyable-text');
                    if (targetSpan) {
                        const childSpans = targetSpan.querySelectorAll('span');
                        if (childSpans.length > 0) {
                            const combinedText = Array.from(childSpans)
                                .map((childSpan) => childSpan.textContent.trim())
                                .join(' ');
                            messages.push([combinedText, messageType]);
                        }
                    } else {
                        messages.push(['This is a document', messageType]);
                    }
                });
            }
            
            console.log('Extracted messages:', messages);
            return JSON.stringify(messages);
        } catch (error) {
            console.error('Error extracting messages:', error);
            return JSON.stringify({ error: error.message });
        }
    })();
    "#, true)?;

    let json_str = match eval_result.value {
        Some(v) => v.as_str()
            .ok_or("JavaScript result is not a string")?
            .to_string(),
        None => {
            if let Some(preview) = eval_result.preview {
                let properties = preview.properties.iter()
                    .filter_map(|prop| prop.value.as_ref())
                    .map(|s| s.clone())
                    .collect::<Vec<String>>();
                let properties_string = properties.join(",");
                format!("[{}]", properties_string)
            } else {
                return Err("No value or preview available".into());
            }
        }
    };

    if let Ok(error_obj) = serde_json::from_str::<serde_json::Value>(&json_str) {
        if let Some(error) = error_obj.get("error") {
            return Err(error.as_str()
                .unwrap_or("Unknown JavaScript error")
                .into());
        }
    }

    let messages: Vec<Vec<String>> = serde_json::from_str(&json_str)?;
    let messages = messages.into_iter()
        .filter(|arr| arr.len() == 2)
        .map(|arr| {
            let clean_str = |s: String| {
                s.replace('\u{202f}', " ")
                 .replace('\u{00a0}', " ")
            };
            (clean_str(arr[0].clone()), clean_str(arr[1].clone()))
        })
        .collect();

    Ok(messages)
}

pub fn send_message(tab: &browser::Tab, num: i64, msg: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://web.whatsapp.com/send?phone={}", num);
    tab.navigate_to(&url)?;

    let input = tab.wait_for_element("._ak1i")?;
    println!("Typing The MSG");
    let chat = input.wait_for_element("._ak1q")?;
    chat.click();
    chat.type_into(msg);
    let send = input.wait_for_element("[aria-label=\"Send\"]")?;
    send.click();
    println!("Successfully Sent the Msg");
    
    Ok(())
}
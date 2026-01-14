// Akkurate Content Script

let activeInput = null;
let floatingBtn = null;

function createFloatingButton() {
    const btn = document.createElement('div');
    btn.id = 'akkurate-float-btn';
    btn.textContent = '✨';
    btn.title = 'Check Grammar with Akkurate';
    document.body.appendChild(btn);

    btn.addEventListener('mousedown', (e) => {
        e.preventDefault(); // Prevent losing focus
        if (activeInput) {
            handleCheck(activeInput);
        }
    });

    return btn;
}

function positionButton(rect) {
    if (!floatingBtn) floatingBtn = createFloatingButton();

    // Position at top-right corner of input
    const top = rect.top + window.scrollY;
    const left = rect.right + window.scrollX;

    floatingBtn.style.top = `${top - 15}px`;
    floatingBtn.style.left = `${left - 15}px`;
    floatingBtn.style.display = 'flex';
}

function hideButton() {
    if (floatingBtn) floatingBtn.style.display = 'none';
}

document.addEventListener('focusin', (e) => {
    const target = e.target;
    if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable) {
        // Filter out password fields etc.
        if (target.tagName === 'INPUT' && target.type !== 'text' && target.type !== 'search') return;

        activeInput = target;
        const rect = target.getBoundingClientRect();
        positionButton(rect);
    }
});

document.addEventListener('focusout', (e) => {
    // Delay hiding to allow click on button
    setTimeout(() => {
        if (document.activeElement !== activeInput) {
            hideButton();
            activeInput = null;
        }
    }, 200);
});

// Show result card
function showResultUI(input, data) {
    // Remove existing card if any
    const existing = document.getElementById('akkurate-result-card');
    if (existing) existing.remove();

    const card = document.createElement('div');
    card.id = 'akkurate-result-card';
    card.className = 'akkurate-card';

    // Header
    const header = document.createElement('div');
    header.className = 'akkurate-header';
    header.innerHTML = '<span>Akkurate Check</span><span class="akkurate-close">&times;</span>';
    card.appendChild(header);

    // Content
    const content = document.createElement('div');
    content.className = 'akkurate-content';

    if (data.issues.length > 0) {
        const issuesCount = document.createElement('div');
        issuesCount.className = 'akkurate-issue-count';
        issuesCount.textContent = `Found ${data.issues.length} issues`;
        content.appendChild(issuesCount);

        // Show diff preview (simplified: just show corrected text for now)
        // In a real app we'd highlight changes.
        const preview = document.createElement('div');
        preview.className = 'akkurate-preview';
        preview.textContent = data.corrected_text;
        content.appendChild(preview);

        // Actions
        const actions = document.createElement('div');
        actions.className = 'akkurate-actions';

        const applyBtn = document.createElement('button');
        applyBtn.className = 'akkurate-btn akkurate-apply';
        applyBtn.textContent = 'Apply Fixes';
        applyBtn.onclick = () => {
            if (input.value !== undefined) {
                input.value = data.corrected_text;
            } else {
                input.innerText = data.corrected_text;
            }
            card.remove();
        };

        actions.appendChild(applyBtn);
        content.appendChild(actions);

    } else {
        const noIssues = document.createElement('div');
        noIssues.className = 'akkurate-no-issues';
        noIssues.textContent = 'No issues found! 🎉';
        content.appendChild(noIssues);

        // Auto-close after 2s
        setTimeout(() => card.remove(), 2000);
    }

    card.appendChild(content);

    // Close handler
    header.querySelector('.akkurate-close').onclick = () => card.remove();

    document.body.appendChild(card);

    // Position card near button
    if (floatingBtn) {
        const rect = floatingBtn.getBoundingClientRect();
        card.style.top = `${rect.bottom + 10 + window.scrollY}px`;
        card.style.left = `${rect.left - 200 + window.scrollX}px`; // Shift left to keep on screen
    }
}

// Update loop in handleCheck
async function handleCheck(input) {
    const text = input.value || input.innerText;
    if (!text || text.trim().length === 0) return;

    // Show loading state
    floatingBtn.textContent = '⏳';

    try {
        chrome.storage.local.get(['gemini_key'], (result) => {
            const apiKey = result.gemini_key;
            if (!apiKey) {
                alert("Please set your Gemini API key in the Akkurate extension popup first.");
                floatingBtn.textContent = '✨';
                return;
            }

            chrome.runtime.sendMessage({
                action: "check_grammar",
                text: text,
                apiKey: apiKey
            }, (response) => {
                if (chrome.runtime.lastError) {
                    console.error("Runtime message error:", chrome.runtime.lastError.message);
                    alert("Extension Error: " + chrome.runtime.lastError.message);
                    floatingBtn.textContent = '❌';
                    return;
                }

                floatingBtn.textContent = '✨';

                if (response && response.success) {
                    showResultUI(input, response.data);
                } else {
                    console.error("Check failed", response ? response.error : "Unknown error");
                    alert("Check failed: " + (response ? response.error : "Unknown error"));
                }
            });
        });

    } catch (e) {
        console.error(e);
        floatingBtn.textContent = '❌';
    }
}

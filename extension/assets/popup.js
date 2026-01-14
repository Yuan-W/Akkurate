document.addEventListener('DOMContentLoaded', () => {
    const apiKeyInput = document.getElementById('apiKey');
    const saveBtn = document.getElementById('saveBtn');
    const status = document.getElementById('status');

    // Load saved key
    chrome.storage.local.get(['gemini_key'], (result) => {
        if (result.gemini_key) {
            apiKeyInput.value = result.gemini_key;
        }
    });

    saveBtn.addEventListener('click', () => {
        const key = apiKeyInput.value.trim();
        if (!key) {
            showStatus("API Key cannot be empty", "error");
            return;
        }

        chrome.storage.local.set({ gemini_key: key }, () => {
            showStatus("Settings saved!", "success");
        });
    });

    function showStatus(msg, type) {
        status.textContent = msg;
        status.className = 'status ' + type;
        setTimeout(() => {
            status.textContent = '';
        }, 3000);
    }
});

window.addEventListener('DOMContentLoaded', () => {
    const urlParams = new URLSearchParams(window.location.search);
    const name = urlParams.get('name');

    const greetingElement = document.getElementById('greeting');
    if (name) {
        greetingElement.textContent = `Hello, ${name}!`;
    }
});
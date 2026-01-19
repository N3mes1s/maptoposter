/**
 * MapToPoster Frontend Application
 */

import * as api from './api.js';

// Application State
const state = {
    themes: [],
    selectedTheme: 'feature_based',
    currentJob: null,
    eventSourceCleanup: null
};

// DOM Elements
const elements = {
    // Form elements
    form: document.getElementById('generator-form'),
    city: document.getElementById('city'),
    country: document.getElementById('country'),
    distance: document.getElementById('distance'),
    distanceValue: document.getElementById('distance-value'),
    themeSelector: document.getElementById('theme-selector'),
    generateBtn: document.getElementById('generate-btn'),

    // Progress elements
    progressSection: document.getElementById('progress-section'),
    progressFill: document.getElementById('progress-fill'),
    progressStep: document.getElementById('progress-step'),
    progressPercent: document.getElementById('progress-percent'),
    progressMessage: document.getElementById('progress-message'),

    // Preview elements
    previewSection: document.getElementById('preview-section'),
    posterImage: document.getElementById('poster-image'),
    downloadBtn: document.getElementById('download-btn'),
    newBtn: document.getElementById('new-btn'),

    // Error elements
    errorSection: document.getElementById('error-section'),
    errorMessage: document.getElementById('error-message'),
    retryBtn: document.getElementById('retry-btn')
};

/**
 * Initialize the application
 */
async function init() {
    await loadThemes();
    setupEventListeners();
}

/**
 * Load themes from API
 */
async function loadThemes() {
    try {
        const response = await api.fetchThemes();
        state.themes = response.themes;
        renderThemeSelector();
    } catch (error) {
        console.error('Failed to load themes:', error);
        elements.themeSelector.innerHTML = '<div class="loading-themes">Failed to load themes</div>';
    }
}

/**
 * Render the theme selector grid
 */
function renderThemeSelector() {
    if (state.themes.length === 0) {
        elements.themeSelector.innerHTML = '<div class="loading-themes">No themes available</div>';
        return;
    }

    elements.themeSelector.innerHTML = state.themes.map(theme => {
        const displayName = theme.name.replace(/_/g, ' ');
        const isSelected = theme.id === state.selectedTheme;

        // Create a gradient preview showing bg and road colors
        const style = `background: linear-gradient(135deg, ${theme.bg} 40%, ${theme.road_motorway} 40%, ${theme.road_motorway} 60%, ${theme.road_primary} 60%)`;

        return `
            <div class="theme-card ${isSelected ? 'selected' : ''}"
                 data-theme="${theme.id}"
                 data-name="${displayName}"
                 style="${style}"
                 title="${theme.description || displayName}">
            </div>
        `;
    }).join('');
}

/**
 * Set up event listeners
 */
function setupEventListeners() {
    // Distance slider
    elements.distance.addEventListener('input', (e) => {
        const km = Math.round(e.target.value / 1000);
        elements.distanceValue.textContent = `${km}km`;
    });

    // Theme selection
    elements.themeSelector.addEventListener('click', (e) => {
        const card = e.target.closest('.theme-card');
        if (card) {
            state.selectedTheme = card.dataset.theme;
            document.querySelectorAll('.theme-card').forEach(c =>
                c.classList.toggle('selected', c === card)
            );
        }
    });

    // Form submission
    elements.generateBtn.addEventListener('click', handleGenerate);

    // New poster button
    elements.newBtn.addEventListener('click', resetToForm);
    elements.retryBtn.addEventListener('click', resetToForm);

    // Enter key on inputs
    elements.city.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') elements.country.focus();
    });
    elements.country.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') handleGenerate();
    });
}

/**
 * Handle poster generation
 */
async function handleGenerate() {
    const city = elements.city.value.trim();
    const country = elements.country.value.trim();

    // Validation
    if (!city || !country) {
        showError('Please enter both city and country');
        return;
    }

    // Disable button and show progress
    elements.generateBtn.disabled = true;
    showProgress();

    try {
        // Create poster job
        const response = await api.createPoster({
            city,
            country,
            theme: state.selectedTheme,
            distance: parseInt(elements.distance.value)
        });

        state.currentJob = response.job_id;

        // Stream progress updates
        state.eventSourceCleanup = api.streamJobProgress(response.job_id, {
            onProgress: updateProgress,
            onCompleted: handleCompleted,
            onError: handleError
        });

    } catch (error) {
        handleError({ message: error.message });
    }
}

/**
 * Show the progress section
 */
function showProgress() {
    elements.form.hidden = true;
    elements.progressSection.hidden = false;
    elements.previewSection.hidden = true;
    elements.errorSection.hidden = true;

    // Reset progress
    elements.progressFill.style.width = '0%';
    elements.progressStep.textContent = 'Starting...';
    elements.progressPercent.textContent = '0%';
    elements.progressMessage.textContent = 'Initializing...';
}

/**
 * Update progress display
 */
function updateProgress({ step, percent, message }) {
    elements.progressFill.style.width = `${percent}%`;
    elements.progressStep.textContent = formatStep(step);
    elements.progressPercent.textContent = `${percent}%`;
    elements.progressMessage.textContent = message;
}

/**
 * Format step name for display
 */
function formatStep(step) {
    if (!step) return 'Processing';
    return step.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase());
}

/**
 * Handle job completion
 */
function handleCompleted({ download_url }) {
    // Clean up event source
    if (state.eventSourceCleanup) {
        state.eventSourceCleanup();
        state.eventSourceCleanup = null;
    }

    // Show preview section
    elements.progressSection.hidden = true;
    elements.previewSection.hidden = false;

    // Set image source
    elements.posterImage.src = download_url;

    // Set up download button
    elements.downloadBtn.onclick = () => {
        const link = document.createElement('a');
        link.href = download_url;
        link.download = `${elements.city.value.toLowerCase().replace(/\s+/g, '_')}_${state.selectedTheme}_poster.png`;
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
    };
}

/**
 * Handle errors
 */
function handleError({ message }) {
    // Clean up event source
    if (state.eventSourceCleanup) {
        state.eventSourceCleanup();
        state.eventSourceCleanup = null;
    }

    showError(message);
}

/**
 * Show error message
 */
function showError(message) {
    elements.form.hidden = true;
    elements.progressSection.hidden = true;
    elements.previewSection.hidden = true;
    elements.errorSection.hidden = false;
    elements.errorMessage.textContent = message;
    elements.generateBtn.disabled = false;
}

/**
 * Reset to form view
 */
function resetToForm() {
    // Clean up any active event source
    if (state.eventSourceCleanup) {
        state.eventSourceCleanup();
        state.eventSourceCleanup = null;
    }

    elements.form.hidden = false;
    elements.progressSection.hidden = true;
    elements.previewSection.hidden = true;
    elements.errorSection.hidden = true;
    elements.generateBtn.disabled = false;

    // Reset progress
    elements.progressFill.style.width = '0%';

    state.currentJob = null;
}

// Initialize the app
init();

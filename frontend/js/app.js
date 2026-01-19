/**
 * MapToPoster Frontend Application
 * Modern UI with live preview and enhanced interactions
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
    formPanel: document.getElementById('form-panel'),
    city: document.getElementById('city'),
    country: document.getElementById('country'),
    distance: document.getElementById('distance'),
    distanceValue: document.getElementById('distance-value'),
    themeSelector: document.getElementById('theme-selector'),
    themeCount: document.getElementById('theme-count'),
    generateBtn: document.getElementById('generate-btn'),

    // Quick select buttons
    quickBtns: document.querySelectorAll('.quick-btn'),

    // Preview elements
    previewPanel: document.getElementById('preview-panel'),
    posterMockup: document.getElementById('poster-mockup'),
    mockupCity: document.getElementById('mockup-city'),
    mockupCountry: document.getElementById('mockup-country'),
    posterImage: document.getElementById('poster-image'),
    previewActions: document.getElementById('preview-actions'),
    downloadBtn: document.getElementById('download-btn'),
    newBtn: document.getElementById('new-btn'),

    // Progress elements
    progressOverlay: document.getElementById('progress-overlay'),
    progressLocation: document.getElementById('progress-location'),
    progressRing: document.getElementById('progress-ring'),
    progressPercent: document.getElementById('progress-percent'),
    progressSteps: document.querySelectorAll('.progress-steps .step'),
    progressMessage: document.getElementById('progress-message'),

    // Error elements
    errorOverlay: document.getElementById('error-overlay'),
    errorMessage: document.getElementById('error-message'),
    retryBtn: document.getElementById('retry-btn')
};

/**
 * Initialize the application
 */
async function init() {
    await loadThemes();
    setupEventListeners();
    updateMockupPreview();
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
        elements.themeSelector.innerHTML = `
            <div class="theme-loading">
                <span>Failed to load themes. Please refresh.</span>
            </div>
        `;
    }
}

/**
 * Render the theme selector grid
 */
function renderThemeSelector() {
    if (state.themes.length === 0) {
        elements.themeSelector.innerHTML = `
            <div class="theme-loading">
                <span>No themes available</span>
            </div>
        `;
        return;
    }

    // Update theme count
    elements.themeCount.textContent = `${state.themes.length} available`;

    elements.themeSelector.innerHTML = state.themes.map(theme => {
        const displayName = theme.name.replace(/_/g, ' ');
        const isSelected = theme.id === state.selectedTheme;

        // Create a gradient preview showing bg and road colors
        const style = `background: linear-gradient(135deg, ${theme.bg} 40%, ${theme.road_motorway || theme.road_default} 40%, ${theme.road_motorway || theme.road_default} 60%, ${theme.road_primary || theme.road_default} 60%)`;

        return `
            <div class="theme-card ${isSelected ? 'selected' : ''}"
                 data-theme="${theme.id}"
                 data-name="${displayName}"
                 data-bg="${theme.bg}"
                 data-text="${theme.text}"
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
    elements.distance.addEventListener('input', handleDistanceChange);

    // Theme selection
    elements.themeSelector.addEventListener('click', handleThemeSelect);

    // Quick city selection
    elements.quickBtns.forEach(btn => {
        btn.addEventListener('click', handleQuickSelect);
    });

    // City/Country input for live preview
    elements.city.addEventListener('input', updateMockupPreview);
    elements.country.addEventListener('input', updateMockupPreview);

    // Form submission
    elements.generateBtn.addEventListener('click', handleGenerate);

    // New poster button
    elements.newBtn.addEventListener('click', resetToForm);
    elements.retryBtn.addEventListener('click', resetToForm);

    // Enter key navigation
    elements.city.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            e.preventDefault();
            elements.country.focus();
        }
    });
    elements.country.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            e.preventDefault();
            handleGenerate();
        }
    });

    // Close error overlay on background click
    elements.errorOverlay.addEventListener('click', (e) => {
        if (e.target === elements.errorOverlay) {
            resetToForm();
        }
    });
}

/**
 * Handle distance slider change
 */
function handleDistanceChange(e) {
    const km = Math.round(e.target.value / 1000);
    elements.distanceValue.textContent = `${km} km`;
}

/**
 * Handle theme selection
 */
function handleThemeSelect(e) {
    const card = e.target.closest('.theme-card');
    if (!card) return;

    state.selectedTheme = card.dataset.theme;

    // Update selection visuals
    document.querySelectorAll('.theme-card').forEach(c => {
        c.classList.toggle('selected', c === card);
    });

    // Update mockup colors based on theme
    updateMockupColors(card.dataset.bg, card.dataset.text);
}

/**
 * Handle quick city selection
 */
function handleQuickSelect(e) {
    const btn = e.target.closest('.quick-btn');
    if (!btn) return;

    elements.city.value = btn.dataset.city;
    elements.country.value = btn.dataset.country;
    updateMockupPreview();

    // Visual feedback
    btn.style.transform = 'scale(0.95)';
    setTimeout(() => {
        btn.style.transform = '';
    }, 100);
}

/**
 * Update the mockup preview with current city/country
 */
function updateMockupPreview() {
    const city = elements.city.value.trim() || 'YOUR CITY';
    const country = elements.country.value.trim() || 'COUNTRY';

    elements.mockupCity.textContent = city.toUpperCase();
    elements.mockupCountry.textContent = country.toUpperCase();
}

/**
 * Update mockup colors based on selected theme
 */
function updateMockupColors(bgColor, textColor) {
    if (!bgColor) return;

    const mockup = elements.posterMockup;
    mockup.style.background = `linear-gradient(145deg, ${bgColor} 0%, ${adjustColor(bgColor, -20)} 100%)`;

    // Update text colors
    const mockupCity = elements.mockupCity;
    const mockupCountry = elements.mockupCountry;

    if (textColor) {
        mockupCity.style.color = textColor;
        mockupCountry.style.color = adjustColor(textColor, -30);
    }
}

/**
 * Adjust color brightness
 */
function adjustColor(hex, amount) {
    if (!hex || !hex.startsWith('#')) return hex;

    let color = hex.slice(1);
    if (color.length === 3) {
        color = color.split('').map(c => c + c).join('');
    }

    const num = parseInt(color, 16);
    let r = Math.min(255, Math.max(0, (num >> 16) + amount));
    let g = Math.min(255, Math.max(0, ((num >> 8) & 0x00FF) + amount));
    let b = Math.min(255, Math.max(0, (num & 0x0000FF) + amount));

    return '#' + (0x1000000 + r * 0x10000 + g * 0x100 + b).toString(16).slice(1);
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
    showProgress(city, country);

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
 * Show the progress overlay
 */
function showProgress(city, country) {
    // Update location display
    elements.progressLocation.textContent = `${city}, ${country}`;

    // Reset progress
    setProgressRing(0);
    elements.progressPercent.textContent = '0%';
    elements.progressMessage.textContent = 'Initializing...';

    // Reset step states
    elements.progressSteps.forEach(step => {
        step.classList.remove('active', 'completed');
    });

    // Show overlay
    elements.progressOverlay.hidden = false;
}

/**
 * Set progress ring fill
 */
function setProgressRing(percent) {
    const circumference = 2 * Math.PI * 45; // r = 45
    const offset = circumference - (percent / 100) * circumference;
    elements.progressRing.style.strokeDashoffset = offset;
}

/**
 * Update progress display
 */
function updateProgress({ step, percent, message }) {
    // Update ring and percentage
    setProgressRing(percent);
    elements.progressPercent.textContent = `${percent}%`;
    elements.progressMessage.textContent = message;

    // Update step indicators
    const stepMapping = {
        'geocoding': 'geocoding',
        'fetching_streets': 'fetching_streets',
        'fetching_water': 'fetching_features',
        'fetching_parks': 'fetching_features',
        'rendering_roads': 'rendering',
        'rendering_features': 'rendering',
        'rendering_text': 'rendering',
        'saving': 'finalizing',
        'completed': 'finalizing'
    };

    const currentStepId = stepMapping[step] || step;
    let foundCurrent = false;

    elements.progressSteps.forEach(stepEl => {
        const stepId = stepEl.dataset.step;

        if (stepId === currentStepId) {
            stepEl.classList.remove('completed');
            stepEl.classList.add('active');
            foundCurrent = true;
        } else if (!foundCurrent) {
            stepEl.classList.remove('active');
            stepEl.classList.add('completed');
        } else {
            stepEl.classList.remove('active', 'completed');
        }
    });
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

    // Mark all steps as completed
    elements.progressSteps.forEach(step => {
        step.classList.remove('active');
        step.classList.add('completed');
    });
    setProgressRing(100);
    elements.progressPercent.textContent = '100%';

    // Small delay before showing result
    setTimeout(() => {
        // Hide progress overlay
        elements.progressOverlay.hidden = true;

        // Show the generated poster
        const placeholder = elements.posterMockup.querySelector('.mockup-placeholder');
        if (placeholder) {
            placeholder.hidden = true;
        }

        elements.posterImage.src = download_url;
        elements.posterImage.hidden = false;

        // Show download actions
        elements.previewActions.hidden = false;

        // Set up download button
        elements.downloadBtn.onclick = () => {
            const link = document.createElement('a');
            link.href = download_url;
            link.download = `${elements.city.value.toLowerCase().replace(/\s+/g, '_')}_${state.selectedTheme}_poster.png`;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
        };
    }, 500);
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
 * Show error overlay
 */
function showError(message) {
    elements.progressOverlay.hidden = true;
    elements.errorMessage.textContent = message;
    elements.errorOverlay.hidden = false;
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

    // Hide overlays
    elements.progressOverlay.hidden = true;
    elements.errorOverlay.hidden = true;

    // Reset poster preview
    const placeholder = elements.posterMockup.querySelector('.mockup-placeholder');
    if (placeholder) {
        placeholder.hidden = false;
    }
    elements.posterImage.hidden = true;
    elements.posterImage.src = '';
    elements.previewActions.hidden = true;

    // Re-enable generate button
    elements.generateBtn.disabled = false;

    // Reset progress
    setProgressRing(0);

    // Reset mockup colors
    elements.posterMockup.style.background = '';
    elements.mockupCity.style.color = '';
    elements.mockupCountry.style.color = '';

    state.currentJob = null;
}

// Initialize the app
init();

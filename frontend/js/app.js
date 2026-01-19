/**
 * MapToPoster Frontend Application
 * Modern UI with live preview and enhanced interactions
 */

import * as api from './api.js';

// Popular world cities database with country and flag
const CITIES_DATABASE = [
    // Asia
    { city: 'Tokyo', country: 'Japan', flag: '🇯🇵' },
    { city: 'Kyoto', country: 'Japan', flag: '🇯🇵' },
    { city: 'Osaka', country: 'Japan', flag: '🇯🇵' },
    { city: 'Seoul', country: 'South Korea', flag: '🇰🇷' },
    { city: 'Busan', country: 'South Korea', flag: '🇰🇷' },
    { city: 'Beijing', country: 'China', flag: '🇨🇳' },
    { city: 'Shanghai', country: 'China', flag: '🇨🇳' },
    { city: 'Hong Kong', country: 'China', flag: '🇭🇰' },
    { city: 'Singapore', country: 'Singapore', flag: '🇸🇬' },
    { city: 'Bangkok', country: 'Thailand', flag: '🇹🇭' },
    { city: 'Chiang Mai', country: 'Thailand', flag: '🇹🇭' },
    { city: 'Hanoi', country: 'Vietnam', flag: '🇻🇳' },
    { city: 'Ho Chi Minh City', country: 'Vietnam', flag: '🇻🇳' },
    { city: 'Mumbai', country: 'India', flag: '🇮🇳' },
    { city: 'Delhi', country: 'India', flag: '🇮🇳' },
    { city: 'Bangalore', country: 'India', flag: '🇮🇳' },
    { city: 'Jaipur', country: 'India', flag: '🇮🇳' },
    { city: 'Dubai', country: 'UAE', flag: '🇦🇪' },
    { city: 'Abu Dhabi', country: 'UAE', flag: '🇦🇪' },
    { city: 'Istanbul', country: 'Turkey', flag: '🇹🇷' },
    { city: 'Taipei', country: 'Taiwan', flag: '🇹🇼' },
    { city: 'Kuala Lumpur', country: 'Malaysia', flag: '🇲🇾' },
    { city: 'Jakarta', country: 'Indonesia', flag: '🇮🇩' },
    { city: 'Bali', country: 'Indonesia', flag: '🇮🇩' },
    { city: 'Manila', country: 'Philippines', flag: '🇵🇭' },
    { city: 'Tel Aviv', country: 'Israel', flag: '🇮🇱' },
    { city: 'Jerusalem', country: 'Israel', flag: '🇮🇱' },

    // Europe
    { city: 'London', country: 'UK', flag: '🇬🇧' },
    { city: 'Edinburgh', country: 'UK', flag: '🏴󠁧󠁢󠁳󠁣󠁴󠁿' },
    { city: 'Manchester', country: 'UK', flag: '🇬🇧' },
    { city: 'Paris', country: 'France', flag: '🇫🇷' },
    { city: 'Lyon', country: 'France', flag: '🇫🇷' },
    { city: 'Marseille', country: 'France', flag: '🇫🇷' },
    { city: 'Nice', country: 'France', flag: '🇫🇷' },
    { city: 'Berlin', country: 'Germany', flag: '🇩🇪' },
    { city: 'Munich', country: 'Germany', flag: '🇩🇪' },
    { city: 'Hamburg', country: 'Germany', flag: '🇩🇪' },
    { city: 'Frankfurt', country: 'Germany', flag: '🇩🇪' },
    { city: 'Amsterdam', country: 'Netherlands', flag: '🇳🇱' },
    { city: 'Rotterdam', country: 'Netherlands', flag: '🇳🇱' },
    { city: 'Rome', country: 'Italy', flag: '🇮🇹' },
    { city: 'Venice', country: 'Italy', flag: '🇮🇹' },
    { city: 'Florence', country: 'Italy', flag: '🇮🇹' },
    { city: 'Milan', country: 'Italy', flag: '🇮🇹' },
    { city: 'Naples', country: 'Italy', flag: '🇮🇹' },
    { city: 'Barcelona', country: 'Spain', flag: '🇪🇸' },
    { city: 'Madrid', country: 'Spain', flag: '🇪🇸' },
    { city: 'Seville', country: 'Spain', flag: '🇪🇸' },
    { city: 'Valencia', country: 'Spain', flag: '🇪🇸' },
    { city: 'Lisbon', country: 'Portugal', flag: '🇵🇹' },
    { city: 'Porto', country: 'Portugal', flag: '🇵🇹' },
    { city: 'Vienna', country: 'Austria', flag: '🇦🇹' },
    { city: 'Salzburg', country: 'Austria', flag: '🇦🇹' },
    { city: 'Prague', country: 'Czech Republic', flag: '🇨🇿' },
    { city: 'Budapest', country: 'Hungary', flag: '🇭🇺' },
    { city: 'Warsaw', country: 'Poland', flag: '🇵🇱' },
    { city: 'Krakow', country: 'Poland', flag: '🇵🇱' },
    { city: 'Stockholm', country: 'Sweden', flag: '🇸🇪' },
    { city: 'Copenhagen', country: 'Denmark', flag: '🇩🇰' },
    { city: 'Oslo', country: 'Norway', flag: '🇳🇴' },
    { city: 'Helsinki', country: 'Finland', flag: '🇫🇮' },
    { city: 'Athens', country: 'Greece', flag: '🇬🇷' },
    { city: 'Santorini', country: 'Greece', flag: '🇬🇷' },
    { city: 'Dublin', country: 'Ireland', flag: '🇮🇪' },
    { city: 'Brussels', country: 'Belgium', flag: '🇧🇪' },
    { city: 'Bruges', country: 'Belgium', flag: '🇧🇪' },
    { city: 'Zurich', country: 'Switzerland', flag: '🇨🇭' },
    { city: 'Geneva', country: 'Switzerland', flag: '🇨🇭' },
    { city: 'Moscow', country: 'Russia', flag: '🇷🇺' },
    { city: 'St Petersburg', country: 'Russia', flag: '🇷🇺' },

    // North America
    { city: 'New York', country: 'USA', flag: '🇺🇸' },
    { city: 'Los Angeles', country: 'USA', flag: '🇺🇸' },
    { city: 'San Francisco', country: 'USA', flag: '🇺🇸' },
    { city: 'Chicago', country: 'USA', flag: '🇺🇸' },
    { city: 'Miami', country: 'USA', flag: '🇺🇸' },
    { city: 'Boston', country: 'USA', flag: '🇺🇸' },
    { city: 'Seattle', country: 'USA', flag: '🇺🇸' },
    { city: 'Washington DC', country: 'USA', flag: '🇺🇸' },
    { city: 'Las Vegas', country: 'USA', flag: '🇺🇸' },
    { city: 'New Orleans', country: 'USA', flag: '🇺🇸' },
    { city: 'Austin', country: 'USA', flag: '🇺🇸' },
    { city: 'Denver', country: 'USA', flag: '🇺🇸' },
    { city: 'Portland', country: 'USA', flag: '🇺🇸' },
    { city: 'Nashville', country: 'USA', flag: '🇺🇸' },
    { city: 'Toronto', country: 'Canada', flag: '🇨🇦' },
    { city: 'Vancouver', country: 'Canada', flag: '🇨🇦' },
    { city: 'Montreal', country: 'Canada', flag: '🇨🇦' },
    { city: 'Mexico City', country: 'Mexico', flag: '🇲🇽' },
    { city: 'Cancun', country: 'Mexico', flag: '🇲🇽' },
    { city: 'Havana', country: 'Cuba', flag: '🇨🇺' },

    // South America
    { city: 'Rio de Janeiro', country: 'Brazil', flag: '🇧🇷' },
    { city: 'Sao Paulo', country: 'Brazil', flag: '🇧🇷' },
    { city: 'Buenos Aires', country: 'Argentina', flag: '🇦🇷' },
    { city: 'Lima', country: 'Peru', flag: '🇵🇪' },
    { city: 'Cusco', country: 'Peru', flag: '🇵🇪' },
    { city: 'Bogota', country: 'Colombia', flag: '🇨🇴' },
    { city: 'Cartagena', country: 'Colombia', flag: '🇨🇴' },
    { city: 'Santiago', country: 'Chile', flag: '🇨🇱' },
    { city: 'Montevideo', country: 'Uruguay', flag: '🇺🇾' },

    // Oceania
    { city: 'Sydney', country: 'Australia', flag: '🇦🇺' },
    { city: 'Melbourne', country: 'Australia', flag: '🇦🇺' },
    { city: 'Brisbane', country: 'Australia', flag: '🇦🇺' },
    { city: 'Perth', country: 'Australia', flag: '🇦🇺' },
    { city: 'Auckland', country: 'New Zealand', flag: '🇳🇿' },
    { city: 'Wellington', country: 'New Zealand', flag: '🇳🇿' },

    // Africa
    { city: 'Cairo', country: 'Egypt', flag: '🇪🇬' },
    { city: 'Cape Town', country: 'South Africa', flag: '🇿🇦' },
    { city: 'Johannesburg', country: 'South Africa', flag: '🇿🇦' },
    { city: 'Marrakech', country: 'Morocco', flag: '🇲🇦' },
    { city: 'Casablanca', country: 'Morocco', flag: '🇲🇦' },
    { city: 'Nairobi', country: 'Kenya', flag: '🇰🇪' },
    { city: 'Lagos', country: 'Nigeria', flag: '🇳🇬' },
    { city: 'Accra', country: 'Ghana', flag: '🇬🇭' },
    { city: 'Tunis', country: 'Tunisia', flag: '🇹🇳' },
];

// Application State
const state = {
    themes: [],
    selectedTheme: 'feature_based',
    currentJob: null,
    eventSourceCleanup: null,
    autocompleteIndex: -1,
    searchDebounceTimer: null,
    lastSearchQuery: ''
};

// DOM Elements
const elements = {
    // Form elements
    formPanel: document.getElementById('form-panel'),
    city: document.getElementById('city'),
    country: document.getElementById('country'),
    cityDropdown: document.getElementById('city-dropdown'),
    countryDropdown: document.getElementById('country-dropdown'),
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

    // City/Country input for live preview and autocomplete
    elements.city.addEventListener('input', (e) => {
        updateMockupPreview();
        handleCityAutocomplete(e.target.value);
    });
    elements.country.addEventListener('input', updateMockupPreview);

    // Autocomplete keyboard navigation
    elements.city.addEventListener('keydown', handleAutocompleteKeydown);

    // Close autocomplete when clicking outside
    document.addEventListener('click', (e) => {
        if (!e.target.closest('.autocomplete-wrapper')) {
            closeAutocomplete();
        }
    });

    // Close on focus out (with delay to allow click)
    elements.city.addEventListener('blur', () => {
        setTimeout(closeAutocomplete, 150);
    });

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
 * Handle city autocomplete input
 * Uses local database for instant results, then fetches from API for more options
 */
function handleCityAutocomplete(query) {
    const trimmed = query.trim().toLowerCase();

    // Hide dropdown if query is too short
    if (trimmed.length < 2) {
        closeAutocomplete();
        return;
    }

    // Don't re-search the same query
    if (trimmed === state.lastSearchQuery) {
        return;
    }
    state.lastSearchQuery = trimmed;

    // First, show instant results from local database
    const localMatches = CITIES_DATABASE.filter(item => {
        const cityMatch = item.city.toLowerCase().includes(trimmed);
        const countryMatch = item.country.toLowerCase().includes(trimmed);
        return cityMatch || countryMatch;
    }).slice(0, 5);

    // Render local results immediately
    renderAutocompleteResults(localMatches, trimmed, true);

    // Clear any existing debounce timer
    if (state.searchDebounceTimer) {
        clearTimeout(state.searchDebounceTimer);
    }

    // Debounce API search (300ms delay)
    state.searchDebounceTimer = setTimeout(async () => {
        try {
            const response = await api.searchLocations(query, 8);
            if (response.results && response.results.length > 0) {
                // Convert API results to match local format
                const apiResults = response.results.map(r => ({
                    city: r.city,
                    country: r.country,
                    flag: getCountryFlag(r.country),
                    fromApi: true
                }));

                // Merge local and API results, avoiding duplicates
                const seen = new Set(localMatches.map(m => `${m.city.toLowerCase()}|${m.country.toLowerCase()}`));
                const merged = [...localMatches];

                for (const result of apiResults) {
                    const key = `${result.city.toLowerCase()}|${result.country.toLowerCase()}`;
                    if (!seen.has(key)) {
                        merged.push(result);
                        seen.add(key);
                    }
                }

                renderAutocompleteResults(merged.slice(0, 8), trimmed, false);
            }
        } catch (error) {
            console.warn('API search failed, using local results only:', error);
        }
    }, 300);
}

/**
 * Get a flag emoji for a country (basic mapping for common countries)
 */
function getCountryFlag(country) {
    const countryLower = country.toLowerCase();
    const flagMap = {
        'usa': '🇺🇸', 'united states': '🇺🇸', 'us': '🇺🇸',
        'uk': '🇬🇧', 'united kingdom': '🇬🇧', 'england': '🇬🇧', 'great britain': '🇬🇧',
        'france': '🇫🇷', 'germany': '🇩🇪', 'italy': '🇮🇹', 'spain': '🇪🇸',
        'japan': '🇯🇵', 'china': '🇨🇳', 'india': '🇮🇳', 'brazil': '🇧🇷',
        'australia': '🇦🇺', 'canada': '🇨🇦', 'mexico': '🇲🇽', 'russia': '🇷🇺',
        'netherlands': '🇳🇱', 'belgium': '🇧🇪', 'switzerland': '🇨🇭', 'austria': '🇦🇹',
        'sweden': '🇸🇪', 'norway': '🇳🇴', 'denmark': '🇩🇰', 'finland': '🇫🇮',
        'portugal': '🇵🇹', 'greece': '🇬🇷', 'turkey': '🇹🇷', 'poland': '🇵🇱',
        'czech republic': '🇨🇿', 'czechia': '🇨🇿', 'hungary': '🇭🇺', 'ireland': '🇮🇪',
        'south korea': '🇰🇷', 'singapore': '🇸🇬', 'thailand': '🇹🇭', 'vietnam': '🇻🇳',
        'indonesia': '🇮🇩', 'malaysia': '🇲🇾', 'philippines': '🇵🇭', 'taiwan': '🇹🇼',
        'uae': '🇦🇪', 'united arab emirates': '🇦🇪', 'israel': '🇮🇱', 'egypt': '🇪🇬',
        'south africa': '🇿🇦', 'morocco': '🇲🇦', 'kenya': '🇰🇪', 'nigeria': '🇳🇬',
        'argentina': '🇦🇷', 'chile': '🇨🇱', 'colombia': '🇨🇴', 'peru': '🇵🇪',
        'new zealand': '🇳🇿', 'cuba': '🇨🇺', 'uruguay': '🇺🇾', 'ghana': '🇬🇭',
        'tunisia': '🇹🇳', 'hong kong': '🇭🇰',
    };
    return flagMap[countryLower] || '🌍';
}

/**
 * Render autocomplete results
 */
function renderAutocompleteResults(matches, query, isLoading) {
    if (matches.length === 0 && !isLoading) {
        elements.cityDropdown.innerHTML = `
            <div class="autocomplete-empty">
                No matching cities found. You can still type any city name.
            </div>
        `;
        elements.cityDropdown.classList.add('active');
        return;
    }

    let html = matches.map((item, index) => {
        const highlightedCity = highlightMatch(item.city, query);
        return `
            <div class="autocomplete-item" data-index="${index}" data-city="${item.city}" data-country="${item.country}">
                <span class="flag">${item.flag}</span>
                <span class="city-name">${highlightedCity}</span>
                <span class="country-name">${item.country}</span>
            </div>
        `;
    }).join('');

    if (isLoading && matches.length > 0) {
        html += `<div class="autocomplete-loading">Searching more locations...</div>`;
    }

    elements.cityDropdown.innerHTML = html;

    // Add click handlers to items
    elements.cityDropdown.querySelectorAll('.autocomplete-item').forEach(item => {
        item.addEventListener('click', () => selectAutocompleteItem(item));
    });

    state.autocompleteIndex = -1;
    elements.cityDropdown.classList.add('active');
}

/**
 * Highlight matching text in autocomplete results
 */
function highlightMatch(text, query) {
    const lowerText = text.toLowerCase();
    const index = lowerText.indexOf(query);

    if (index === -1) return text;

    const before = text.slice(0, index);
    const match = text.slice(index, index + query.length);
    const after = text.slice(index + query.length);

    return `${before}<span class="autocomplete-highlight">${match}</span>${after}`;
}

/**
 * Handle keyboard navigation in autocomplete
 */
function handleAutocompleteKeydown(e) {
    const items = elements.cityDropdown.querySelectorAll('.autocomplete-item');
    if (!items.length || !elements.cityDropdown.classList.contains('active')) {
        return;
    }

    switch (e.key) {
        case 'ArrowDown':
            e.preventDefault();
            state.autocompleteIndex = Math.min(state.autocompleteIndex + 1, items.length - 1);
            updateAutocompleteSelection(items);
            break;

        case 'ArrowUp':
            e.preventDefault();
            state.autocompleteIndex = Math.max(state.autocompleteIndex - 1, 0);
            updateAutocompleteSelection(items);
            break;

        case 'Enter':
            if (state.autocompleteIndex >= 0 && items[state.autocompleteIndex]) {
                e.preventDefault();
                selectAutocompleteItem(items[state.autocompleteIndex]);
            }
            break;

        case 'Escape':
            closeAutocomplete();
            break;
    }
}

/**
 * Update visual selection in autocomplete
 */
function updateAutocompleteSelection(items) {
    items.forEach((item, i) => {
        item.classList.toggle('selected', i === state.autocompleteIndex);
    });

    // Scroll selected item into view
    if (state.autocompleteIndex >= 0 && items[state.autocompleteIndex]) {
        items[state.autocompleteIndex].scrollIntoView({ block: 'nearest' });
    }
}

/**
 * Select an item from autocomplete
 */
function selectAutocompleteItem(item) {
    const city = item.dataset.city;
    const country = item.dataset.country;

    elements.city.value = city;
    elements.country.value = country;

    updateMockupPreview();
    closeAutocomplete();

    // Focus the country field briefly, then move to distance
    elements.country.focus();
}

/**
 * Close autocomplete dropdown
 */
function closeAutocomplete() {
    elements.cityDropdown.classList.remove('active');
    state.autocompleteIndex = -1;
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

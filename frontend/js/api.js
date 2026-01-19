/**
 * API client for MapToPoster
 */

const API_BASE = '/api';

/**
 * Fetch all available themes
 */
export async function fetchThemes() {
    const response = await fetch(`${API_BASE}/themes`);
    if (!response.ok) {
        throw new Error('Failed to fetch themes');
    }
    return response.json();
}

/**
 * Create a new poster generation job
 */
export async function createPoster(data) {
    const response = await fetch(`${API_BASE}/posters`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });

    if (!response.ok) {
        const error = await response.json();
        throw new Error(error.detail || 'Failed to create poster');
    }

    return response.json();
}

/**
 * Get job status (polling fallback)
 */
export async function getJobStatus(jobId) {
    const response = await fetch(`${API_BASE}/posters/${jobId}`);
    if (!response.ok) {
        throw new Error('Failed to get job status');
    }
    return response.json();
}

/**
 * Stream job progress using Server-Sent Events
 * @param {string} jobId - The job ID to monitor
 * @param {object} handlers - Event handlers
 * @param {function} handlers.onProgress - Called on progress updates
 * @param {function} handlers.onCompleted - Called when job completes
 * @param {function} handlers.onError - Called on errors
 * @returns {function} Cleanup function to close the connection
 */
export function streamJobProgress(jobId, handlers) {
    const eventSource = new EventSource(`${API_BASE}/jobs/${jobId}/stream`);

    eventSource.addEventListener('progress', (e) => {
        try {
            const data = JSON.parse(e.data);
            handlers.onProgress?.(data);
        } catch (err) {
            console.error('Failed to parse progress event:', err);
        }
    });

    eventSource.addEventListener('completed', (e) => {
        try {
            const data = JSON.parse(e.data);
            handlers.onCompleted?.(data);
        } catch (err) {
            console.error('Failed to parse completed event:', err);
        }
        eventSource.close();
    });

    eventSource.addEventListener('error', (e) => {
        if (e.data) {
            try {
                const data = JSON.parse(e.data);
                handlers.onError?.(data);
            } catch (err) {
                handlers.onError?.({ message: 'Unknown error' });
            }
        } else {
            // Connection error - might be normal closure
            console.log('EventSource connection closed');
        }
        eventSource.close();
    });

    eventSource.onerror = () => {
        // Only report error if not closed normally
        if (eventSource.readyState !== EventSource.CLOSED) {
            handlers.onError?.({ message: 'Connection lost' });
        }
        eventSource.close();
    };

    // Return cleanup function
    return () => eventSource.close();
}

/**
 * Get the download URL for a completed poster
 */
export function getDownloadUrl(jobId) {
    return `${API_BASE}/posters/${jobId}/download`;
}

/**
 * Search for locations using the Nominatim API
 * @param {string} query - Search query (city name, etc.)
 * @param {number} limit - Maximum results (default 8)
 * @returns {Promise<{results: Array, query: string}>}
 */
export async function searchLocations(query, limit = 8) {
    const params = new URLSearchParams({ q: query, limit: limit.toString() });
    const response = await fetch(`${API_BASE}/locations/search?${params}`);

    if (!response.ok) {
        throw new Error('Failed to search locations');
    }

    return response.json();
}

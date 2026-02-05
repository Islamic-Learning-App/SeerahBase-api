export const API_BASE_URL = '/api';

export async function fetchAPI(endpoint: string, options: RequestInit = {}) {
    const url = `${API_BASE_URL}${endpoint}`;
    const headers = {
        'Content-Type': 'application/json',
        ...options.headers,
        // Add API Key
        'x-api-key': 'seerah_secret_key_123',
    };

    const response = await fetch(url, { ...options, headers });

    if (!response.ok) {
        throw new Error(`API Error: ${response.statusText}`);
    }

    // Handle empty responses (like DELETE 204)
    if (response.status === 204) {
        return null;
    }

    return response.json();
}

export const api = {
    getEras: () => fetchAPI('/eras'),
    getEventsByEra: (eraId: number) => fetchAPI(`/eras/${eraId}/events`),
    getEventById: (id: number) => fetchAPI(`/events/${id}`),
    getAllEvents: () => fetchAPI('/events'),
    createEvent: (data: any) => fetchAPI('/events', { method: 'POST', body: JSON.stringify(data) }),
    updateEvent: (id: number, data: any) => fetchAPI(`/events/${id}`, { method: 'PUT', body: JSON.stringify(data) }),
    deleteEvent: (id: number) => fetchAPI(`/events/${id}`, { method: 'DELETE' }),

    getRandomQuiz: () => fetchAPI('/questions/random'),
    createQuestion: (data: any) => fetchAPI('/questions', { method: 'POST', body: JSON.stringify(data) }),
    updateQuestion: (id: number, data: any) => fetchAPI(`/questions/${id}`, { method: 'PUT', body: JSON.stringify(data) }),
    deleteQuestion: (id: number) => fetchAPI(`/questions/${id}`, { method: 'DELETE' }),
};

export interface Category {
  id: number;
  name: string;
  nameBn: string;
  categoryType: string;
  description?: string;
  descriptionBn?: string;
  icon?: string;
  sortOrder: number;
  parentId?: number;
}

export interface Event {
  id: number;
  categoryId?: number;
  title: string;
  titleBn?: string;
  description: string;
  descriptionBn?: string;
  eventDate?: string;
  source?: string;
  imageUrl?: string;
}

export interface Question {
  id: number;
  eventId?: number;
  categoryId?: number;
  questionText: string;
  questionTextBn?: string;
  explanation?: string;
  explanationBn?: string;
  difficultyLevel?: string;
}

export interface AnswerOption {
  id: number;
  questionId: number;
  optionText: string;
  optionTextBn?: string;
  isCorrect: boolean;
}

export interface QuestionWithOptions {
  question: Question;
  options: AnswerOption[];
}

export interface PaginatedResponse<T> {
  data: T[];
  page: number;
  limit: number;
  total: number;
}

// In dev mode, use Vite proxy (/api → localhost:3000)
// In production, hit the Leapcell backend directly
const API_URL = import.meta.env.DEV
  ? "/api"
  : "https://seerabase-api.apn.leapcell.app";
const API_KEY = "seerah-api-key-123";

async function fetchAPI(endpoint: string, options?: RequestInit) {
  const res = await fetch(`${API_URL}${endpoint}`, {
    ...options,
    headers: {
      "Content-Type": "application/json",
      "x-api-key": API_KEY,
      ...options?.headers,
    },
  });

  if (!res.ok) {
    throw new Error(`API Error: ${res.statusText}`);
  }

  // Handle 204 No Content
  if (res.status === 204) return null;

  return res.json();
}

export const api = {
  // Categories
  getCategories: async (type?: string): Promise<Category[]> => {
    const query = type ? `?type=${type}` : "";
    return fetchAPI(`/categories${query}`);
  },

  createCategory: async (data: Partial<Category>): Promise<Category> => {
    return fetchAPI("/categories", {
      method: "POST",
      body: JSON.stringify(data),
    });
  },

  updateCategory: async (id: number, data: Partial<Category>): Promise<Category> => {
    return fetchAPI(`/categories/${id}`, {
      method: "PUT",
      body: JSON.stringify(data),
    });
  },

  deleteCategory: async (id: number): Promise<void> => {
    return fetchAPI(`/categories/${id}`, {
      method: "DELETE",
    });
  },

  getEventsByCategory: async (categoryId: number): Promise<Event[]> => {
    return fetchAPI(`/categories/${categoryId}/events`);
  },

  // Events
  getAllEvents: async (page = 1, limit = 20): Promise<PaginatedResponse<Event>> => {
    return fetchAPI(`/events?page=${page}&limit=${limit}`);
  },

  getEventById: async (id: number): Promise<Event> => {
    return fetchAPI(`/events/${id}`);
  },

  createEvent: async (data: Partial<Event>): Promise<Event> => {
    return fetchAPI("/events", {
      method: "POST",
      body: JSON.stringify(data),
    });
  },

  updateEvent: async (id: number, data: Partial<Event>): Promise<Event> => {
    return fetchAPI(`/events/${id}`, {
      method: "PUT",
      body: JSON.stringify(data),
    });
  },

  deleteEvent: async (id: number): Promise<void> => {
    return fetchAPI(`/events/${id}`, {
      method: "DELETE",
    });
  },

  // Quiz
  getQuestionsByEvent: async (eventId: number): Promise<QuestionWithOptions[]> => {
    return fetchAPI(`/events/${eventId}/quiz`);
  },

  getRandomQuiz: async (): Promise<QuestionWithOptions[]> => {
    return fetchAPI("/quiz/random");
  },

  createQuestion: async (data: any): Promise<QuestionWithOptions> => {
    return fetchAPI("/questions", {
      method: "POST",
      body: JSON.stringify(data),
    });
  },

  deleteQuestion: async (id: number): Promise<void> => {
    return fetchAPI(`/questions/${id}`, {
      method: "DELETE",
    });
  },
};

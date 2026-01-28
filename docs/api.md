# SeerahBase API Documentation

## Overview
This API provides access to the SeerahBase database, serving information about Islamic history events, eras, and related quizzes.

## Authentication
Authentication is required for **writing** data (e.g., creating events). Read operations are public.

**Method**: API Key  
**Header**: `x-api-key`

---

## Endpoints

### History

#### 1. Get All Eras
Retrieves a list of all historical eras.

- **URL**: `/eras`
- **Method**: `GET`
- **Auth Required**: No

**Response**: `200 OK`
```json
[
  {
    "id": 1,
    "name": "Era Name",
    "description": "Description...",
    "start_date": "YYYY-MM-DD",
    "end_date": "YYYY-MM-DD"
  }
]
```

#### 2. Get Events by Era
Retrieves all events belonging to a specific era.

- **URL**: `/eras/{id}/events`
- **Method**: `GET`
- **Auth Required**: No
- **Path Params**:
    - `id`: Integer (Era ID)

**Response**: `200 OK`
```json
[
  {
    "id": 10,
    "era_id": 1,
    "title": "Event Title",
    "description": "Event Description",
    "event_date": "YYYY-MM-DD",
    "source": "Source Reference"
  }
]
```

#### 3. Get All Events
Retrieves all events in the database, ordered by date.

- **URL**: `/events`
- **Method**: `GET`
- **Auth Required**: No

**Response**: `200 OK`
```json
[
  {
    "id": 1,
    "title": "Event Title",
    ...
  }
]
```

#### 4. Create New Event
Adds a new event to the database.

- **URL**: `/events`
- **Method**: `POST`
- **Auth Required**: Yes (`x-api-key`)
- **Body**: `application/json`
```json
{
  "era_id": 1,
  "title": "New Event",
  "description": "Detailed description...",
  "event_date": "2023-10-27",
  "source": "Bukhari"
}
```

**Response**:
- `201 Created`: Returns the created event object.
- `401 Unauthorized`: If API key is missing or invalid.
- `500 Internal Server Error`: If database insertion fails.

---

### Quizzes (MCQ)

#### 5. Get Questions by Event
Retrieves quiz questions related to a specific event.

- **URL**: `/questions/event/{id}`
- **Method**: `GET`
- **Auth Required**: No
- **Path Params**:
    - `id`: Integer (Event ID)

**Response**: `200 OK`
```json
[
  {
    "question": {
      "id": 100,
      "event_id": 10,
      "question_text": "What happened...?",
      ...
    },
    "options": [
      {
        "id": 1001,
        "question_id": 100,
        "option_text": "Option A",
        "is_correct": true
      },
      ...
    ]
  }
]
```

#### 6. Get Random Quiz
Retrieves 5 random questions for a quick quiz.

- **URL**: `/questions/random`
- **Method**: `GET`
- **Auth Required**: No

**Response**: `200 OK`
```json
[
  {
    "question": { ... },
    "options": [ ... ]
  },
  ...
]
```

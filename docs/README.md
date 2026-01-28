# SeerahBase API Documentation

## Overview
This API provides access to the SeerahBase database, serving information about Islamic history events, eras, and related quizzes.

**Base URL**: `http://localhost:3000`  
**Version**: `v1` (Current)

## Authentication
Authentication is required for **writing** data (e.g., creating events). Read operations are public.

- **Mechanism**: API Key
- **Header Name**: `x-api-key`
- **Location**: Request Header

To obtain a key, check the server configuration (`API_KEY` environment variable).

---

## Endpoints

### History

#### 1. Get All Eras
Retrieves a list of all historical eras saved in the database.

- **Endpoint**: `/eras`
- **Method**: `GET`
- **Auth Required**: No

**Request Example**:
```bash
curl http://localhost:3000/eras
```

**Response**: `200 OK`
```json
[
  {
    "id": 1,
    "name": "Makkah Period",
    "description": "The life of the Prophet (SAW) in Makkah",
    "start_date": "0610-01-01",
    "end_date": "0622-01-01"
  }
]
```

#### 2. Get Events by Era
Retrieves all events belonging to a specific historical era.

- **Endpoint**: `/eras/{id}/events`
- **Method**: `GET`
- **Auth Required**: No
- **Path Parameters**:
    - `id` (integer): The unique ID of the Era.

**Request Example**:
```bash
curl http://localhost:3000/eras/1/events
```

**Response**: `200 OK`
```json
[
  {
    "id": 10,
    "era_id": 1,
    "title": "First Revelation",
    "description": "The Prophet (SAW) receives the first revelation in Cave Hira.",
    "event_date": "0610-08-10",
    "source": "Sahih Bukhari"
  }
]
```

#### 3. Get All Events
Retrieves all events in the database, ordered chronologically.

- **Endpoint**: `/events`
- **Method**: `GET`
- **Auth Required**: No

**Request Example**:
```bash
curl http://localhost:3000/events
```

#### 4. Create New Event
Adds a new historical event to the database.

- **Endpoint**: `/events`
- **Method**: `POST`
- **Auth Required**: Yes
- **Headers**:
    - `x-api-key`: `your_secret_key`
    - `Content-Type`: `application/json`

**Request Body (JSON)**:
| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | Yes | Title of the event |
| `description` | String | Yes | Detailed description |
| `era_id` | Integer | No | ID of the related era |
| `event_date` | String | No | Date in `YYYY-MM-DD` format |
| `source` | String | No | Source reference (e.g., Bukhari) |

**Request Example**:
```bash
curl -X POST http://localhost:3000/events \
  -H "x-api-key: secret_key" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "The Migration",
    "description": "Migration from Makkah to Madinah",
    "era_id": 1,
    "event_date": "0622-09-24"
  }'
```

**Success Response**: `201 Created`
Returns the created [Event](#event-schema) object.

**Error Responses**:
- `401 Unauthorized`: Missing or invalid `x-api-key`.
- `500 Internal Server Error`: Database constraint violation or other server error.

---

### Quizzes (MCQ)

#### 5. Get Questions by Event
Retrieves quiz questions related to a specific event.

- **Endpoint**: `/questions/event/{id}`
- **Method**: `GET`
- **Path Parameters**:
    - `id` (integer): The unique Event ID.

**Request Example**:
```bash
curl http://localhost:3000/questions/event/10
```

**Response**: `200 OK`
Returns a list of [QuestionWithOptions](#questionwithoptions-schema).

#### 6. Get Random Quiz
Retrieves 5 random questions for a quick quiz.

- **Endpoint**: `/questions/random`
- **Method**: `GET`

**Request Example**:
```bash
curl http://localhost:3000/questions/random
```

---

## Data Schemas

### Era Schema
| Field | Type | Nullable | Description |
|---|---|---|---|
| `id` | Integer | No | Unique ID |
| `name` | String | No | Era name |
| `description` | String | Yes | Brief description |
| `start_date` | String | Yes | ISO Date (YYYY-MM-DD) |
| `end_date` | String | Yes | ISO Date (YYYY-MM-DD) |

### Event Schema
| Field | Type | Nullable | Description |
|---|---|---|---|
| `id` | Integer | No | Unique ID |
| `era_id` | Integer | Yes | Foreign Key to Era |
| `title` | String | No | Event title |
| `description` | String | No | Full description |
| `event_date` | String | Yes | ISO Date |
| `source` | String | Yes | Source citation |

### QuestionWithOptions Schema
A composite object containing the question and its answer choices.

**Structure**:
```json
{
  "id": 100,             // Question ID (Flattend)
  "event_id": 10,        // Related Event ID
  "question_text": "...",
  "explanation": "...",
  "difficulty_level": "easy",
  "options": [           // Array of AnswerOption
    {
      "id": 1001,
      "question_id": 100,
      "option_text": "Option A",
      "is_correct": true
    }
  ]
}
```

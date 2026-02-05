<script lang="ts">
    import { api } from "$lib/api";

    let activeTab = $state("events"); // 'events' | 'questions'
    let events = $state<any[]>([]);
    let eras = $state<any[]>([]);

    // Event Form State
    let editingEventId = $state<number | null>(null);
    let eventForm = $state({
        title: "",
        description: "",
        era_id: "",
        event_date: "",
        source: "",
    });

    // Question Form State
    let questionForm = $state({
        event_id: "",
        question_text: "",
        explanation: "",
        difficulty_level: "Medium",
        options: [
            { option_text: "", is_correct: false },
            { option_text: "", is_correct: false },
            { option_text: "", is_correct: false },
            { option_text: "", is_correct: false },
        ],
    });

    $effect(() => {
        loadData();
    });

    async function loadData() {
        eras = await api.getEras();
        events = await api.getAllEvents();
    }

    // --- Event Handlers ---

    function resetEventForm() {
        editingEventId = null;
        eventForm = {
            title: "",
            description: "",
            era_id: "",
            event_date: "",
            source: "",
        };
    }

    function editEvent(event: any) {
        editingEventId = event.id;
        eventForm = { ...event, era_id: event.era_id || "" };
    }

    async function saveEvent() {
        const payload = {
            ...eventForm,
            era_id: eventForm.era_id ? Number(eventForm.era_id) : null,
        };

        if (editingEventId) {
            await api.updateEvent(editingEventId, payload);
        } else {
            await api.createEvent(payload);
        }
        await loadData();
        resetEventForm();
    }

    async function deleteEvent(id: number) {
        if (
            confirm("Are you sure? This will delete associated questions too.")
        ) {
            await api.deleteEvent(id);
            await loadData();
        }
    }

    // --- Question Handlers ---

    async function saveQuestion() {
        if (!questionForm.event_id) {
            alert("Please select an event.");
            return;
        }

        const payload = {
            ...questionForm,
            event_id: Number(questionForm.event_id),
            options: questionForm.options.filter((o) => o.option_text), // Filter empty? Or just send all.
        };

        // Ensure one correct answer?
        if (!questionForm.options.some((o) => o.is_correct)) {
            alert("Please mark at least one correct option.");
            return;
        }

        await api.createQuestion(payload);
        alert("Question Created!");
        // Reset form
        questionForm = {
            event_id: questionForm.event_id, // Keep event selected
            question_text: "",
            explanation: "",
            difficulty_level: "Medium",
            options: [
                { option_text: "", is_correct: false },
                { option_text: "", is_correct: false },
                { option_text: "", is_correct: false },
                { option_text: "", is_correct: false },
            ],
        };
    }
</script>

<div class="space-y-6">
    <div class="flex justify-between items-center">
        <h1 class="text-4xl font-bold text-primary">Admin Dashboard</h1>
        <div class="flex space-x-2 bg-secondary p-1 rounded-lg">
            <button
                onclick={() => (activeTab = "events")}
                class="px-4 py-2 rounded-md transition-all {activeTab ===
                'events'
                    ? 'bg-primary text-secondary font-bold'
                    : 'text-gray-400 hover:text-white'}"
            >
                Manage Events
            </button>
            <button
                onclick={() => (activeTab = "questions")}
                class="px-4 py-2 rounded-md transition-all {activeTab ===
                'questions'
                    ? 'bg-primary text-secondary font-bold'
                    : 'text-gray-400 hover:text-white'}"
            >
                Add Questions
            </button>
        </div>
    </div>

    {#if activeTab === "events"}
        <!-- Event Form -->
        <div class="bg-secondary p-6 rounded-xl border border-gray-800">
            <h2 class="text-xl font-bold text-light mb-4">
                {editingEventId ? "Edit Event" : "Create New Event"}
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <input
                    bind:value={eventForm.title}
                    placeholder="Title"
                    class="bg-dark border border-gray-700 p-3 rounded text-white"
                />
                <select
                    bind:value={eventForm.era_id}
                    class="bg-dark border border-gray-700 p-3 rounded text-white"
                >
                    <option value="">Select Era</option>
                    {#each eras as era}
                        <option value={era.id}>{era.name}</option>
                    {/each}
                </select>
                <input
                    bind:value={eventForm.event_date}
                    placeholder="Event Date (e.g. 610 CE)"
                    class="bg-dark border border-gray-700 p-3 rounded text-white"
                />
                <input
                    bind:value={eventForm.source}
                    placeholder="Source (Optional)"
                    class="bg-dark border border-gray-700 p-3 rounded text-white"
                />
                <textarea
                    bind:value={eventForm.description}
                    placeholder="Description"
                    rows="3"
                    class="bg-dark border border-gray-700 p-3 rounded text-white col-span-1 md:col-span-2"
                ></textarea>
            </div>
            <div class="flex justify-end space-x-4 mt-4">
                {#if editingEventId}
                    <button
                        onclick={resetEventForm}
                        class="text-gray-400 hover:text-white">Cancel</button
                    >
                {/if}
                <button
                    onclick={saveEvent}
                    class="bg-primary text-secondary font-bold px-6 py-2 rounded hover:bg-yellow-400"
                >
                    {editingEventId ? "Update Event" : "Create Event"}
                </button>
            </div>
        </div>

        <!-- Event List -->
        <div class="space-y-4">
            {#each events as event}
                <div
                    class="flex justify-between items-center bg-secondary p-4 rounded border border-gray-800 hover:border-gray-700"
                >
                    <div>
                        <h3 class="font-bold text-light">{event.title}</h3>
                        <p class="text-xs text-gray-500">
                            {event.event_date || "No Date"}
                        </p>
                    </div>
                    <div class="flex space-x-2">
                        <button
                            onclick={() => editEvent(event)}
                            class="text-blue-400 hover:text-blue-300"
                            >Edit</button
                        >
                        <button
                            onclick={() => deleteEvent(event.id)}
                            class="text-red-400 hover:text-red-300"
                            >Delete</button
                        >
                    </div>
                </div>
            {/each}
        </div>
    {:else}
        <!-- Question Form -->
        <div class="bg-secondary p-6 rounded-xl border border-gray-800">
            <h2 class="text-xl font-bold text-light mb-4">Add New Question</h2>

            <div class="space-y-4">
                <select
                    bind:value={questionForm.event_id}
                    class="w-full bg-dark border border-gray-700 p-3 rounded text-white"
                >
                    <option value="">Select Related Event</option>
                    {#each events as event}
                        <option value={event.id}>{event.title}</option>
                    {/each}
                </select>

                <input
                    bind:value={questionForm.question_text}
                    placeholder="Question Text"
                    class="w-full bg-dark border border-gray-700 p-3 rounded text-white"
                />

                <textarea
                    bind:value={questionForm.explanation}
                    placeholder="Explanation (Shown after answering)"
                    rows="2"
                    class="w-full bg-dark border border-gray-700 p-3 rounded text-white"
                ></textarea>

                <select
                    bind:value={questionForm.difficulty_level}
                    class="w-full bg-dark border border-gray-700 p-3 rounded text-white"
                >
                    <option value="Easy">Easy</option>
                    <option value="Medium">Medium</option>
                    <option value="Hard">Hard</option>
                </select>

                <div class="space-y-2 mt-4">
                    <h3 class="font-bold text-accent">Options</h3>
                    {#each questionForm.options as option, i}
                        <div class="flex items-center space-x-2">
                            <input
                                type="radio"
                                name="correct"
                                checked={option.is_correct}
                                onchange={() => {
                                    questionForm.options.forEach(
                                        (o, idx) => (o.is_correct = idx === i),
                                    );
                                }}
                            />
                            <input
                                bind:value={option.option_text}
                                placeholder={`Option ${i + 1}`}
                                class="flex-grow bg-dark border border-gray-700 p-2 rounded text-white"
                            />
                        </div>
                    {/each}
                </div>

                <button
                    onclick={saveQuestion}
                    class="w-full bg-primary text-secondary font-bold px-6 py-3 rounded mt-4 hover:bg-yellow-400"
                >
                    Save Question
                </button>
            </div>
        </div>
    {/if}
</div>

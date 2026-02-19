<script lang="ts">
  import { onMount } from "svelte";
  import { api, type Event, type Category } from "$lib/api";

  let activeTab: "events" | "questions" = "events";

  // Data
  let events: Event[] = [];
  let categories: Category[] = [];
  let loading = true;

  // Forms
  let eventForm: Partial<Event> = { title: "", description: "" };
  let editingEventId: number | null = null;

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    loading = true;
    try {
      const [e, c] = await Promise.all([
        api.getAllEvents(1, 100),
        api.getCategories(),
      ]);
      events = e.data;
      categories = c;
    } catch (err) {
      console.error(err);
      alert("Failed to load data");
    } finally {
      loading = false;
    }
  }

  async function handleSubmitEvent() {
    try {
      // Basic validation
      if (!eventForm.categoryId) {
        alert("Please select a category");
        return;
      }

      if (editingEventId) {
        await api.updateEvent(editingEventId, eventForm);
      } else {
        await api.createEvent(eventForm);
      }
      await loadData();
      resetEventForm();
    } catch (err) {
      console.error(err);
      alert("Error saving event");
    }
  }

  async function deleteEvent(id: number) {
    if (!confirm("Are you sure? This will delete all related questions too."))
      return;
    try {
      await api.deleteEvent(id);
      await loadData();
    } catch (err) {
      console.error(err);
      alert("Error deleting event");
    }
  }

  function editEvent(event: Event) {
    editingEventId = event.id;
    eventForm = { ...event };
    activeTab = "events";
  }

  function resetEventForm() {
    editingEventId = null;
    eventForm = { title: "", description: "" };
  }
</script>

<div class="space-y-8">
  <div class="flex justify-between items-center border-b border-white/10 pb-4">
    <h1 class="text-3xl font-bold text-primary">Admin Dashboard</h1>
    <button
      on:click={loadData}
      class="px-4 py-2 bg-white/10 rounded-lg hover:bg-white/20 transition-colors flex items-center gap-2"
      disabled={loading}
    >
      {#if loading}
        <span class="animate-spin">↻</span>
      {:else}
        <span>↻</span>
      {/if}
      Refresh Data
    </button>
  </div>

  <div class="flex gap-4">
    <button
      class="px-4 py-2 rounded-lg {activeTab === 'events'
        ? 'bg-primary text-black font-bold'
        : 'bg-white/10'}"
      on:click={() => (activeTab = "events")}
    >
      Manage Events
    </button>
    <button
      class="px-4 py-2 rounded-lg {activeTab === 'questions'
        ? 'bg-primary text-black font-bold'
        : 'bg-white/10'}"
      on:click={() => (activeTab = "questions")}
    >
      Manage Questions
    </button>
  </div>

  {#if activeTab === "events"}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
      <!-- List -->
      <div class="bg-white/5 p-6 rounded-xl border border-white/10">
        <h2 class="text-xl font-bold mb-4">Existing Events</h2>
        {#if loading}
          <div class="text-center text-gray-500">Loading...</div>
        {:else}
          <div
            class="space-y-4 max-h-[600px] overflow-y-auto pr-2 custom-scrollbar"
          >
            {#each events as event}
              <div
                class="p-4 bg-black/20 rounded-lg flex justify-between items-start group"
              >
                <div>
                  <h3
                    class="font-bold text-white group-hover:text-primary transition-colors"
                  >
                    {event.title}
                  </h3>
                  <div class="text-sm text-gray-400">
                    <span class="text-primary/70"
                      >{event.eventDate || "No date"}</span
                    >
                    • ID: {event.id}
                  </div>
                </div>
                <div
                  class="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity"
                >
                  <button
                    on:click={() => editEvent(event)}
                    class="text-blue-400 hover:text-white px-2 py-1 bg-blue-500/10 rounded"
                    >Edit</button
                  >
                  <button
                    on:click={() => deleteEvent(event.id)}
                    class="text-red-400 hover:text-white px-2 py-1 bg-red-500/10 rounded"
                    >Delete</button
                  >
                </div>
              </div>
            {/each}
            {#if events.length === 0}
              <div class="text-center text-gray-500">
                No events found. Create one!
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Form -->
      <div
        class="bg-white/5 p-6 rounded-xl border border-white/10 h-fit sticky top-24"
      >
        <h2 class="text-xl font-bold mb-4">
          {editingEventId ? "Edit Event" : "Create New Event"}
        </h2>
        <form on:submit|preventDefault={handleSubmitEvent} class="space-y-4">
          <div>
            <label for="event-category" class="block text-sm mb-1 text-gray-400"
              >Category</label
            >
            <select
              id="event-category"
              bind:value={eventForm.categoryId}
              class="w-full bg-black/40 border border-white/10 rounded p-2 text-white focus:border-primary outline-none"
            >
              <option value={undefined}>Select Category</option>
              {#each categories as cat}
                <option value={cat.id}>{cat.name} ({cat.categoryType})</option>
              {/each}
            </select>
          </div>

          <div>
            <label for="event-title" class="block text-sm mb-1 text-gray-400"
              >Title (English)</label
            >
            <input
              id="event-title"
              bind:value={eventForm.title}
              class="w-full bg-black/40 border border-white/10 rounded p-2 text-white focus:border-primary outline-none"
              required
            />
          </div>

          <div>
            <label for="event-title-bn" class="block text-sm mb-1 text-gray-400"
              >Title (Bengali)</label
            >
            <input
              id="event-title-bn"
              bind:value={eventForm.titleBn}
              class="w-full bg-black/40 border border-white/10 rounded p-2 text-white focus:border-primary outline-none"
            />
          </div>

          <div>
            <label
              for="event-description"
              class="block text-sm mb-1 text-gray-400"
              >Description (Markdown supported)</label
            >
            <textarea
              id="event-description"
              bind:value={eventForm.description}
              class="w-full bg-black/40 border border-white/10 rounded p-2 text-white h-32 focus:border-primary outline-none"
              required
            ></textarea>
          </div>

          <div>
            <label
              for="event-description-bn"
              class="block text-sm mb-1 text-gray-400"
              >Description (Bengali)</label
            >
            <textarea
              id="event-description-bn"
              bind:value={eventForm.descriptionBn}
              class="w-full bg-black/40 border border-white/10 rounded p-2 text-white h-24 focus:border-primary outline-none"
            ></textarea>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label for="event-date" class="block text-sm mb-1 text-gray-400"
                >Date (e.g., 610 CE)</label
              >
              <input
                id="event-date"
                bind:value={eventForm.eventDate}
                class="w-full bg-black/40 border border-white/10 rounded p-2 text-white focus:border-primary outline-none"
              />
            </div>
            <div>
              <label for="event-source" class="block text-sm mb-1 text-gray-400"
                >Source</label
              >
              <input
                id="event-source"
                bind:value={eventForm.source}
                class="w-full bg-black/40 border border-white/10 rounded p-2 text-white focus:border-primary outline-none"
              />
            </div>
          </div>

          <div>
            <label for="event-image" class="block text-sm mb-1 text-gray-400"
              >Image URL</label
            >
            <input
              id="event-image"
              bind:value={eventForm.imageUrl}
              class="w-full bg-black/40 border border-white/10 rounded p-2 text-white focus:border-primary outline-none"
              placeholder="https://..."
            />
          </div>

          <div class="flex gap-4 pt-4">
            <button
              type="submit"
              class="bg-primary text-black font-bold py-2 px-6 rounded hover:bg-yellow-400 flex-1 transition-colors"
            >
              {editingEventId ? "Update Event" : "Create Event"}
            </button>
            {#if editingEventId}
              <button
                type="button"
                on:click={resetEventForm}
                class="bg-white/10 py-2 px-4 rounded hover:bg-white/20 transition-colors"
              >
                Cancel
              </button>
            {/if}
          </div>
        </form>
      </div>
    </div>
  {:else}
    <div
      class="text-center text-gray-400 py-12 bg-white/5 rounded-xl border border-white/10"
    >
      <p>Question management interface would go here.</p>
      <p class="text-sm mt-2">
        (Implementation similar to events, linking questions to
        events/categories)
      </p>
    </div>
  {/if}
</div>

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 6px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
    border-radius: 3px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.3);
  }
</style>

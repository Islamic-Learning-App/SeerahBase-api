<script lang="ts">
    import { api } from "$lib/api";

    let eras = $state<any[]>([]);
    let selectedEraId = $state<number | null>(null);
    let events = $state<any[]>([]);
    let loading = $state(true);
    let sortOrder = $state<"asc" | "desc">("asc");

    // Modal State removed
    // let showModal = $state(false);
    // let selectedEvent = $state<any>(null);

    $effect(() => {
        api.getEras().then((data) => {
            eras = data;
            loading = false;
            // Select first era by default if available
            if (data.length > 0) {
                selectEra(data[0].id);
            }
        });
    });

    async function selectEra(id: number) {
        selectedEraId = id;
        const data = await api.getEventsByEra(id);
        events = data;
    }

    let sortedEvents = $derived(
        [...events].sort((a, b) => {
            const dateA = new Date(a.event_date || 0).getTime();
            const dateB = new Date(b.event_date || 0).getTime();
            return sortOrder === "asc" ? dateA - dateB : dateB - dateA;
        }),
    );

    function toggleSort() {
        sortOrder = sortOrder === "asc" ? "desc" : "asc";
    }

    // Simple truncation helper
    function truncate(text: string, length: number = 150) {
        if (!text) return "";
        // If text is short enough, render markdown (links active)
        if (text.length <= length) {
            return parseMarkdown(text);
        }
        // If text is long, truncate raw text to avoid breaking HTML tags from markdown
        // The modal will show the full rendered markdown
        return text.slice(0, length) + "...";
    }

    // Markdown Parser helper
    function parseMarkdown(text: string) {
        if (!text) return "";
        // Replace [text](url) with <a href="url" ...>text</a>
        return text.replace(
            /\[([^\]]+)\]\(([^)]+)\)/g,
            '<a href="$2" target="_blank" class="text-primary hover:text-yellow-300 underline">$1</a>',
        );
    }
</script>

<div class="space-y-8 relative">
    <div class="flex justify-between items-center">
        <h1 class="text-4xl font-bold text-primary">Historical Timeline</h1>
        <button
            onclick={toggleSort}
            class="px-4 py-2 rounded-lg bg-dark border border-gray-700 hover:border-primary transition-colors flex items-center gap-2 text-sm"
        >
            <span
                >Sort: {sortOrder === "asc"
                    ? "Oldest First"
                    : "Newest First"}</span
            >
            <span class="text-primary">{sortOrder === "asc" ? "↑" : "↓"}</span>
        </button>
    </div>

    {#if loading}
        <p class="text-gray-400 animate-pulse">Loading eras...</p>
    {:else}
        <!-- Eras Navigation -->
        <div
            class="flex overflow-x-auto space-x-4 pb-4 scrollbar-thin scrollbar-thumb-primary scrollbar-track-secondary"
        >
            {#each eras as era}
                <button
                    onclick={() => selectEra(era.id)}
                    class="flex-shrink-0 px-6 py-3 rounded-full border transition-all duration-300
            {selectedEraId === era.id
                        ? 'bg-primary text-secondary border-primary font-bold shadow-[0_0_15px_rgba(212,175,55,0.4)]'
                        : 'bg-secondary text-gray-300 border-gray-700 hover:border-gray-500'}"
                >
                    {era.name}
                </button>
            {/each}
        </div>

        <!-- Events Timeline -->
        <div class="relative border-l-2 border-gray-800 ml-4 space-y-12">
            {#if sortedEvents.length === 0}
                <p class="ml-8 text-gray-500 italic">
                    No events found for this era.
                </p>
            {:else}
                {#each sortedEvents as event}
                    <div class="ml-8 relative group">
                        <!-- Dot -->
                        <div
                            class="absolute -left-[41px] top-1 w-5 h-5 rounded-full bg-secondary border-2 border-primary group-hover:bg-primary transition-colors"
                        ></div>

                        <div
                            class="bg-secondary p-6 rounded-xl border border-gray-800 hover:border-gray-700 transition-all shadow-md"
                        >
                            <span
                                class="text-sm font-mono text-primary bg-primary/10 px-2 py-1 rounded mb-2 inline-block"
                            >
                                {event.event_date || "Unknown Date"}
                            </span>
                            <h3 class="text-2xl font-semibold text-light mb-2">
                                {event.title}
                            </h3>

                            <!-- Truncated Description -->
                            <div class="text-gray-400 leading-relaxed">
                                {@html truncate(event.description, 150)}
                            </div>

                            {#if event.description && event.description.length > 150}
                                <a
                                    href={`/events/${event.id}`}
                                    class="mt-3 inline-block text-sm text-primary hover:text-yellow-300 font-medium underline decoration-dotted underline-offset-4"
                                >
                                    Read More &rarr;
                                </a>
                            {/if}

                            {#if event.source}
                                <p
                                    class="text-xs text-gray-600 mt-4 border-t border-gray-800 pt-2"
                                >
                                    Source: {event.source}
                                </p>
                            {/if}
                        </div>
                    </div>
                {/each}
            {/if}
        </div>
    {/if}
</div>

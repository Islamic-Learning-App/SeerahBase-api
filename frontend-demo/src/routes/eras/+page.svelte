<script lang="ts">
    import { api } from "$lib/api";

    let eras = $state<any[]>([]);
    let selectedEraId = $state<number | null>(null);
    let events = $state<any[]>([]);
    let loading = $state(true);

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
</script>

<div class="space-y-8">
    <h1 class="text-4xl font-bold text-primary">Historical Timeline</h1>

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
            {#if events.length === 0}
                <p class="ml-8 text-gray-500 italic">
                    No events found for this era.
                </p>
            {:else}
                {#each events as event}
                    <div class="ml-8 relative group">
                        <!-- Dot -->
                        <div
                            class="absolute -left-[41px] top-1 w-5 h-5 rounded-full bg-secondary border-2 border-primary group-hover:bg-primary transition-colors"
                        ></div>

                        <div
                            class="bg-secondary p-6 rounded-xl border border-gray-800 hover:border-gray-700 transition-all"
                        >
                            <span
                                class="text-sm font-mono text-primary bg-primary/10 px-2 py-1 rounded mb-2 inline-block"
                            >
                                {event.event_date || "Unknown Date"}
                            </span>
                            <h3 class="text-2xl font-semibold text-light mb-2">
                                {event.title}
                            </h3>
                            <p class="text-gray-400 leading-relaxed">
                                {event.description}
                            </p>
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

<script lang="ts">
    import { page } from "$app/stores"; // CORRECT WAY to access params in older SvelteKit/some setups, but in Svelte 5 with runes we might use $page
    // Actually, improved way: use load function or page store. let's stick to simple client side fetch for now.
    import { api } from "$lib/api";
    import { onMount } from "svelte";

    // In Svelte 5 / SvelteKit 2:
    import { page as pageStore } from "$app/stores";

    let id = $state<number>(0);
    let event = $state<any>(null);
    let loading = $state(true);
    let error = $state("");

    // Markdown Parser helper (Same as eras page)
    function parseMarkdown(text: string) {
        if (!text) return "";
        // Replace [text](url) with <a href="url" ...>text</a>
        return text.replace(
            /\[([^\]]+)\]\(([^)]+)\)/g,
            '<a href="$2" target="_blank" class="text-primary hover:underline">$1</a>',
        );
    }

    $effect(() => {
        // Subscribe to page store to get ID
        const routeId = $pageStore.params.id;
        if (routeId) {
            id = Number(routeId);
            loadEvent(id);
        }
    });

    async function loadEvent(eventId: number) {
        loading = true;
        try {
            event = await api.getEventById(eventId);
            if (!event) throw new Error("Event not found");
        } catch (err: any) {
            error = err.message || "Failed to load event";
        } finally {
            loading = false;
        }
    }
</script>

<div class="max-w-3xl mx-auto py-10">
    <a href="/eras" class="text-gray-400 hover:text-white mb-6 inline-block"
        >&larr; Back to Timeline</a
    >

    {#if loading}
        <div class="flex justify-center py-20">
            <div
                class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"
            ></div>
        </div>
    {:else if error}
        <div
            class="bg-red-900/20 border border-red-500 text-red-400 p-6 rounded-xl text-center"
        >
            <p class="text-xl font-bold mb-2">Error</p>
            <p>{error}</p>
        </div>
    {:else if event}
        <div
            class="bg-secondary p-8 rounded-2xl border border-gray-800 shadow-2xl animate-in fade-in slide-in-from-bottom-4 duration-500"
        >
            <div
                class="flex justify-between items-start mb-6 border-b border-gray-700 pb-4"
            >
                <h1 class="text-4xl font-bold text-primary tracking-tight">
                    {event.title}
                </h1>
                <span
                    class="bg-dark text-accent px-4 py-1 rounded-full text-sm font-mono border border-gray-700"
                >
                    {event.event_date}
                </span>
            </div>

            <div
                class="prose prose-invert prose-lg prose-p:text-gray-300 prose-a:text-primary max-w-none leading-relaxed"
            >
                {@html parseMarkdown(event.description)}
            </div>

            {#if event.source}
                <div
                    class="mt-8 pt-6 border-t border-gray-700 flex items-center text-sm text-gray-500"
                >
                    <span class="mr-2">Source:</span>
                    <span class="italic text-gray-400">{event.source}</span>
                </div>
            {/if}
        </div>
    {:else}
        <div class="text-center text-gray-500 py-20">Event not found.</div>
    {/if}
</div>

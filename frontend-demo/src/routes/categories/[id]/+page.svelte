<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { api, type Event, type Category } from "$lib/api";
  import Skeleton from "$lib/components/Skeleton.svelte";
  import { fade, fly } from "svelte/transition";

  let events: Event[] = [];
  let category: Category | undefined;
  let loading = true;
  let error = "";

  // Get ID from route params
  $: categoryId = parseInt($page.params.id || "0");

  onMount(async () => {
    try {
      // Fetch both events and category info (by fetching all cats and finding one)
      // Ideally backend would have getCategoryById or events response include category info.
      const [allCats, eventList] = await Promise.all([
        api.getCategories(),
        api.getEventsByCategory(categoryId),
      ]);

      category = allCats.find((c) => c.id === categoryId);
      events = eventList;
    } catch (e) {
      error = "Failed to load data";
      console.error(e);
    } finally {
      loading = false;
    }
  });
</script>

<div class="max-w-4xl mx-auto space-y-8">
  <div class="mb-6 flex justify-between items-center">
    <a
      href="/categories"
      class="text-gray-400 hover:text-white transition-colors flex items-center gap-2"
    >
      &larr; Back to Categories
    </a>
  </div>

  {#if loading}
    <div class="space-y-4 animate-pulse">
      <div class="h-8 w-1/3 bg-white/5 rounded"></div>
      <div class="h-4 w-1/2 bg-white/5 rounded"></div>
      <div class="space-y-6 mt-8">
        {#each Array(3) as _}
          <div class="h-32 bg-white/5 rounded-xl"></div>
        {/each}
      </div>
    </div>
  {:else if error}
    <div class="p-4 bg-red-500/10 text-red-500 rounded-lg text-center">
      {error}
    </div>
  {:else if category}
    <header class="text-center space-y-4">
      <div class="text-6xl mb-4 animate-bounce-slow">
        {category.icon || "📚"}
      </div>
      <h1 class="text-4xl font-bold text-white">
        {category.name}
      </h1>
      <h2 class="text-2xl text-primary font-ben">
        {category.nameBn}
      </h2>
      {#if category.description}
        <p class="text-gray-400 max-w-2xl mx-auto">
          {category.description}
        </p>
      {/if}
    </header>

    <div
      class="relative border-l-2 border-white/10 ml-4 md:ml-6 my-12 space-y-12"
    >
      {#each events as event, i}
        <div
          class="relative pl-8 md:pl-12"
          in:fly={{ y: 20, duration: 500, delay: i * 100 }}
        >
          <!-- Timeline Dot -->
          <div
            class="absolute -left-[9px] top-0 w-4 h-4 rounded-full bg-primary border-4 border-gray-900 shadow-[0_0_10px_theme('colors.primary')]"
          ></div>

          <div
            class="bg-white/5 hover:bg-white/10 p-6 rounded-xl border border-white/5 transition-all group"
          >
            <div class="flex justify-between items-start mb-2">
              <span
                class="text-primary text-sm font-mono bg-primary/10 px-2 py-1 rounded"
              >
                {event.eventDate || "Unknown Date"}
              </span>
              {#if event.source}
                <span class="text-xs text-gray-500 italic" title={event.source}
                  >Source: {event.source}</span
                >
              {/if}
            </div>

            <h3
              class="text-xl font-bold text-white mb-1 group-hover:text-primary transition-colors"
            >
              <a href="/events/{event.id}">{event.title}</a>
            </h3>
            <h4 class="text-lg text-gray-400 font-ben mb-4">
              {event.titleBn || ""}
            </h4>

            <p class="text-gray-300 leading-relaxed text-sm line-clamp-3">
              {event.description}
            </p>

            <div class="mt-4 pt-4 border-t border-white/5 flex gap-4">
              <a
                href="/events/{event.id}"
                class="text-sm text-primary hover:underline">Read More &rarr;</a
              >
              <a
                href="/events/{event.id}/quiz"
                class="text-sm text-accent hover:underline">Take Quiz 📝</a
              >
            </div>
          </div>
        </div>
      {/each}

      {#if events.length === 0}
        <div class="text-center text-gray-500 py-12">
          No events found for this category yet.
        </div>
      {/if}
    </div>
  {:else}
    <div class="text-center text-red-500">Category not found.</div>
  {/if}
</div>

<style>
  /* Optional: Custom scrollbar or subtle animations */
  .animate-bounce-slow {
    animation: bounce 3s infinite;
  }
  @keyframes bounce {
    0%,
    100% {
      transform: translateY(-5%);
    }
    50% {
      transform: translateY(5%);
    }
  }
</style>

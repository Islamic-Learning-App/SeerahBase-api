<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { api, type Event } from "$lib/api";
  import Skeleton from "$lib/components/Skeleton.svelte";
  import { marked } from "marked";

  let event: Event | undefined;
  let loading = true;
  let error = "";

  $: eventId = parseInt($page.params.id);

  onMount(async () => {
    try {
      event = await api.getEventById(eventId);
    } catch (e) {
      error = "Failed to load event details";
      console.error(e);
    } finally {
      loading = false;
    }
  });
</script>

<div class="max-w-3xl mx-auto py-8 space-y-8">
  <div class="mb-6">
    <button
      class="text-gray-400 hover:text-white transition-colors flex items-center gap-2"
      on:click={() => history.back()}
    >
      &larr; Back
    </button>
  </div>

  {#if loading}
    <div class="space-y-4 animate-pulse">
      <div class="h-8 w-3/4 bg-white/5 rounded"></div>
      <div class="h-64 w-full bg-white/5 rounded-xl"></div>
      <div class="space-y-2">
        <div class="h-4 w-full bg-white/5 rounded"></div>
        <div class="h-4 w-5/6 bg-white/5 rounded"></div>
      </div>
    </div>
  {:else if error}
    <div class="bg-red-500/10 text-red-500 p-4 rounded-lg text-center">
      {error}
    </div>
  {:else if event}
    <article class="prose prose-invert max-w-none">
      <header class="text-center mb-8 not-prose">
        <div class="text-sm text-primary font-mono mb-2">
          {event.eventDate || "Unknown Date"}
        </div>
        <h1
          class="text-4xl md:text-5xl font-bold bg-gradient-to-br from-white to-gray-400 bg-clip-text text-transparent mb-2"
        >
          {event.title}
        </h1>
        {#if event.titleBn}
          <h2 class="text-2xl text-primary/90 font-ben">{event.titleBn}</h2>
        {/if}
      </header>

      {#if event.imageUrl}
        <div
          class="my-8 rounded-xl overflow-hidden border border-white/10 shadow-2xl"
        >
          <img
            src={event.imageUrl}
            alt={event.title}
            class="w-full h-auto object-cover max-h-[500px]"
          />
        </div>
      {/if}

      <div
        class="bg-white/5 p-8 rounded-2xl border border-white/5 backdrop-blur-sm"
      >
        <div class="font-serif text-lg leading-relaxed text-gray-200">
          {@html marked(event.description)}
        </div>

        {#if event.descriptionBn}
          <div
            class="mt-8 pt-8 border-t border-white/10 font-ben text-lg leading-relaxed text-gray-300"
          >
            {@html marked(event.descriptionBn)}
          </div>
        {/if}
      </div>

      {#if event.source}
        <div class="mt-8 text-sm text-gray-500 text-center italic">
          Source: {event.source}
        </div>
      {/if}

      <div class="mt-12 flex justify-center">
        <a
          href="/events/{event.id}/quiz"
          class="bg-primary text-black font-bold py-3 px-8 rounded-full hover:bg-yellow-400 transition-colors shadow-lg hover:shadow-primary/50 flex items-center gap-2"
        >
          <span>✨</span> Take Quiz on this Event
        </a>
      </div>
    </article>
  {:else}
    <div class="text-center text-red-500">Event not found</div>
  {/if}
</div>

<script>
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  let visible = false;

  onMount(() => {
    visible = true;
  });
</script>

<div
  class="flex flex-col items-center justify-center min-h-[80vh] text-center space-y-12"
>
  {#if visible}
    <div in:fade={{ duration: 1000, delay: 200 }} class="space-y-6">
      <h1 class="text-6xl md:text-8xl font-bold tracking-tighter">
        <span
          class="bg-gradient-to-r from-primary via-yellow-200 to-primary bg-clip-text text-transparent bg-300-percent animate-gradient"
        >
          Explore the Seerah
        </span>
      </h1>

      <p class="text-xl md:text-2xl text-gray-400 max-w-2xl mx-auto font-light">
        A journey through the life of Prophet Muhammad (ﷺ) and the history of
        Islam.
      </p>
    </div>

    <div
      in:fade={{ duration: 1000, delay: 600 }}
      class="flex flex-wrap gap-6 justify-center"
    >
      <a
        href="/categories"
        class="group relative px-8 py-4 bg-primary text-black font-bold rounded-full overflow-hidden transition-transform hover:scale-105"
      >
        <span class="relative z-10 flex items-center gap-2">
          <span>🕌</span> Explore Categories
        </span>
        <div
          class="absolute inset-0 bg-white/20 translate-y-full group-hover:translate-y-0 transition-transform"
        ></div>
      </a>

      <a
        href="/quiz"
        class="px-8 py-4 bg-white/10 border border-white/10 rounded-full hover:bg-white/20 transition-all hover:scale-105 backdrop-blur-sm"
      >
        Test Your Knowledge &rarr;
      </a>
    </div>

    <div
      in:fade={{ duration: 1000, delay: 1000 }}
      class="grid grid-cols-1 md:grid-cols-3 gap-8 text-left max-w-4xl w-full mt-12 bg-white/5 p-8 rounded-2xl border border-white/5"
    >
      <div class="space-y-2">
        <div class="text-3xl">📅</div>
        <h3 class="font-bold text-white">Interactive Timeline</h3>
        <p class="text-sm text-gray-400">
          Navigate through history chronologically or by major events.
        </p>
      </div>
      <div class="space-y-2">
        <div class="text-3xl">📚</div>
        <h3 class="font-bold text-white">20+ Categories</h3>
        <p class="text-sm text-gray-400">
          Detailed sections on Prophets, Battles, Revelations, and more.
        </p>
      </div>
      <div class="space-y-2">
        <div class="text-3xl">🧠</div>
        <h3 class="font-bold text-white">Knowledge Quiz</h3>
        <p class="text-sm text-gray-400">
          Test what you've learned with our interactive MCQ system.
        </p>
      </div>
    </div>
  {/if}
</div>

<style>
  .bg-300-percent {
    background-size: 300%;
  }
  .animate-gradient {
    animation: gradient 8s ease infinite;
  }
  @keyframes gradient {
    0% {
      background-position: 0% 50%;
    }
    50% {
      background-position: 100% 50%;
    }
    100% {
      background-position: 0% 50%;
    }
  }
</style>

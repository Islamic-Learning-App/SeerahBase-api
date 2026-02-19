<script lang="ts">
  import { onMount } from "svelte";
  import { api, type Category } from "$lib/api";
  import Skeleton from "$lib/components/Skeleton.svelte"; // Assuming Skeleton component exists or I need to create it

  let categories: Category[] = [];
  let loading = true;
  let groupedCategories: Record<string, Category[]> = {};

  const typeLabels: Record<string, string> = {
    era: "Historical Eras / যুগসমূহ",
    prophet: "Prophets / নবী-রাসূলগণ",
    surah_group: "Quranic Revelations / ওহী পর্যায়",
    topic: "Topics / বিষয়ভিত্তিক",
  };

  onMount(async () => {
    try {
      categories = await api.getCategories();
      groupedCategories = categories.reduce(
        (acc, cat) => {
          const type = cat.categoryType || "other";
          if (!acc[type]) acc[type] = [];
          acc[type].push(cat);
          return acc;
        },
        {} as Record<string, Category[]>,
      );
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });
</script>

<div class="space-y-12">
  <header class="text-center space-y-4">
    <h1
      class="text-4xl md:text-5xl font-bold bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent"
    >
      Browse Seerah Categories
    </h1>
    <p class="text-gray-400">
      Explore Islamic history through curated collections.
    </p>
  </header>

  {#if loading}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <!-- Simple Skeleton Loop -->
      {#each Array(6) as _}
        <div class="h-32 bg-white/5 rounded-xl animate-pulse"></div>
      {/each}
    </div>
  {:else}
    {#each Object.entries(groupedCategories) as [type, cats]}
      <section class="space-y-6">
        <h2
          class="text-2xl font-semibold text-primary/90 border-b border-white/10 pb-2"
        >
          {typeLabels[type] || type.toUpperCase()}
        </h2>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {#each cats as cat}
            <a
              href="/categories/{cat.id}"
              class="group relative bg-white/5 hover:bg-white/10 p-6 rounded-xl transition-all duration-300 border border-white/5 hover:border-primary/30 flex flex-col items-center text-center gap-3"
            >
              <div
                class="text-4xl filter drop-shadow-lg group-hover:scale-110 transition-transform"
              >
                {cat.icon || "📄"}
              </div>

              <div>
                <h3
                  class="font-bold text-lg group-hover:text-primary transition-colors"
                >
                  {cat.name}
                </h3>
                <p class="text-sm text-gray-400 font-ben">{cat.nameBn}</p>
              </div>

              {#if cat.description}
                <p class="text-xs text-gray-500 line-clamp-2">
                  {cat.description}
                </p>
              {/if}
            </a>
          {/each}
        </div>
      </section>
    {/each}
  {/if}
</div>

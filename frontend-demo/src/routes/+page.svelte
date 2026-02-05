<script lang="ts">
    import { onMount } from "svelte";

    // Placeholder for simple animation logic if needed
</script>

<div
    class="flex flex-col items-center justify-center py-20 text-center space-y-8"
>
    <h1
        class="text-6xl font-extrabold bg-clip-text text-transparent bg-gradient-to-r from-primary to-accent animate-pulse"
    >
        Explore the Seerah
    </h1>
    <p class="text-xl text-gray-400 max-w-2xl">
        Immerse yourself in the life of the Prophet (PBUH). Navigate through
        historical eras, test your knowledge with interactive quizzes, and
        contribute to the growing database.
    </p>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 w-full max-w-4xl mt-10">
        <a
            href="/eras"
            class="group p-8 bg-secondary rounded-2xl border border-gray-800 hover:border-primary transition-all hover:shadow-[0_0_20px_rgba(212,175,55,0.2)]"
        >
            <div class="text-4xl mb-4">📜</div>
            <h2 class="group-hover:text-primary transition-colors">Timeline</h2>
            <p class="text-sm text-gray-500">
                Explore events chronologically through different eras.
            </p>
        </a>

        <a
            href="/quiz"
            class="group p-8 bg-secondary rounded-2xl border border-gray-800 hover:border-primary transition-all hover:shadow-[0_0_20px_rgba(212,175,55,0.2)]"
        >
            <div class="text-4xl mb-4">🧠</div>
            <h2 class="group-hover:text-primary transition-colors">
                Take a Quiz
            </h2>
            <p class="text-sm text-gray-500">
                Test your knowledge with random questions.
            </p>
        </a>

        <a
            href="/admin"
            class="group p-8 bg-secondary rounded-2xl border border-gray-800 hover:border-primary transition-all hover:shadow-[0_0_20px_rgba(212,175,55,0.2)]"
        >
            <div class="text-4xl mb-4">⚙️</div>
            <h2 class="group-hover:text-primary transition-colors">
                Manage Data
            </h2>
            <p class="text-sm text-gray-500">
                Add or edit events and questions (Admin).
            </p>
        </a>
    </div>
</div>

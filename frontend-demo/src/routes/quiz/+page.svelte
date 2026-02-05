<script lang="ts">
    import { api } from "$lib/api";

    let questions = $state<any[]>([]);
    let currentIndex = $state(0);
    let selectedOptionId = $state<number | null>(null);
    let revealed = $state(false);
    let loading = $state(true);

    // Derived state for current question
    let currentQuestion = $derived(questions[currentIndex]);

    $effect(() => {
        loadQuiz();
    });

    async function loadQuiz() {
        loading = true;
        try {
            questions = await api.getRandomQuiz();
            currentIndex = 0;
            resetState();
        } catch (e) {
            console.error(e);
        } finally {
            loading = false;
        }
    }

    function resetState() {
        selectedOptionId = null;
        revealed = false;
    }

    function handleOptionClick(option: any) {
        if (revealed) return;
        selectedOptionId = option.id;
        revealed = true;
    }

    function nextQuestion() {
        if (currentIndex < questions.length - 1) {
            currentIndex++;
            resetState();
        } else {
            loadQuiz();
        }
    }
</script>

<div class="flex flex-col items-center max-w-2xl mx-auto space-y-8">
    <h1 class="text-3xl font-bold text-primary">Test Your Knowledge</h1>

    {#if loading}
        <div class="p-10 text-center animate-pulse text-gray-400">
            Loading quiz...
        </div>
    {:else if currentQuestion}
        <div
            class="w-full bg-secondary p-8 rounded-2xl border border-gray-800 shadow-xl"
        >
            <!-- Question Header -->
            <div class="flex justify-between items-center mb-6">
                <span
                    class="text-xs font-mono text-gray-500 uppercase tracking-widest"
                    >Question {currentIndex + 1}</span
                >
                <span
                    class={`px-2 py-1 rounded text-xs font-bold ${
                        currentQuestion.question.difficulty_level === "Hard"
                            ? "bg-red-900 text-red-200"
                            : currentQuestion.question.difficulty_level ===
                                "Medium"
                              ? "bg-yellow-900 text-yellow-200"
                              : "bg-green-900 text-green-200"
                    }`}
                >
                    {currentQuestion.question.difficulty_level || "Medium"}
                </span>
            </div>

            <h2 class="text-2xl font-semibold mb-8 text-light leading-snug">
                {currentQuestion.question.question_text}
            </h2>

            <!-- Options -->
            <div class="space-y-4">
                {#each currentQuestion.options as option}
                    <button
                        onclick={() => handleOptionClick(option)}
                        disabled={revealed}
                        class="w-full text-left p-4 rounded-xl border transition-all duration-200 flex justify-between items-center
              {revealed
                            ? option.is_correct
                                ? 'bg-green-900/20 border-green-600 text-green-100'
                                : selectedOptionId === option.id
                                  ? 'bg-red-900/20 border-red-600'
                                  : 'border-gray-800 opacity-50'
                            : 'bg-dark border-gray-700 hover:border-primary hover:bg-gray-800'}"
                    >
                        <span>{option.option_text}</span>
                        {#if revealed && option.is_correct}
                            <span class="text-green-500">✓</span>
                        {:else if revealed && selectedOptionId === option.id && !option.is_correct}
                            <span class="text-red-500">✗</span>
                        {/if}
                    </button>
                {/each}
            </div>

            <!-- Explanation & Next Button -->
            {#if revealed}
                <div class="mt-8 pt-6 border-t border-gray-800 animate-fade-in">
                    {#if currentQuestion.question.explanation}
                        <div class="mb-6 text-gray-400 text-sm">
                            <strong class="text-primary block mb-1"
                                >Explanation:</strong
                            >
                            {currentQuestion.question.explanation}
                        </div>
                    {/if}

                    <button
                        onclick={nextQuestion}
                        class="w-full py-3 bg-primary text-secondary font-bold rounded-xl hover:bg-yellow-400 transition-colors shadow-lg"
                    >
                        {currentIndex < questions.length - 1
                            ? "Next Question"
                            : "Load New Questions"}
                    </button>
                </div>
            {/if}
        </div>
    {:else}
        <p>No questions available.</p>
    {/if}
</div>

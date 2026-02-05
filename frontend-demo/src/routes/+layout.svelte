<script lang="ts">
	import favicon from "$lib/assets/favicon.svg";
	import "../app.css";

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="min-h-screen flex flex-col">
	<nav class="bg-secondary text-primary p-4 shadow-md">
		<div class="container mx-auto flex justify-between items-center">
			<a href="/" class="text-2xl font-bold tracking-tighter"
				>SeerahBase</a
			>
			<div class="flex space-x-6">
				<a href="/" class="hover:text-accent transition-colors">Home</a>
				<a href="/eras" class="hover:text-accent transition-colors"
					>Eras</a
				>
				<a href="/quiz" class="hover:text-accent transition-colors"
					>Quiz</a
				>
				<a href="/admin" class="hover:text-accent transition-colors"
					>Admin</a
				>
			</div>
		</div>
	</nav>

	<main class="container mx-auto p-6 flex-grow">
		{@render children()}
	</main>

	<footer class="bg-secondary text-gray-500 text-center p-4 mt-8">
		&copy; 2026 SeerahBase
	</footer>
</div>

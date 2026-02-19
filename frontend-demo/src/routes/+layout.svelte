<script>
  import "../app.css";
</script>

<div class="min-h-screen bg-secondary text-white font-sans flex flex-col">
  <nav
    class="bg-secondary/90 backdrop-blur-md sticky top-0 z-50 border-b border-white/10"
  >
    <div class="container mx-auto px-4 py-4 flex justify-between items-center">
      <a
        href="/"
        class="text-2xl font-bold text-primary flex items-center gap-2"
      >
        <span>🕌</span> SeerahBase
      </a>
      <div class="flex gap-6 text-sm font-medium">
        <a href="/" class="hover:text-primary transition-colors">Home</a>
        <a href="/categories" class="hover:text-primary transition-colors"
          >Explore</a
        >
        <a href="/quiz" class="hover:text-primary transition-colors">Quiz</a>
        <a href="/admin" class="hover:text-primary transition-colors">Admin</a>
      </div>
    </div>
  </nav>

  <main class="flex-grow container mx-auto px-4 py-8">
    <slot />
  </main>

  <footer class="bg-black/30 py-6 mt-12 border-t border-white/10">
    <div class="container mx-auto px-4 text-center text-gray-400 text-sm">
      &copy; {new Date().getFullYear()} SeerahBase. All rights reserved.
    </div>
  </footer>
</div>

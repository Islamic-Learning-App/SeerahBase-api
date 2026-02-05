/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            colors: {
                // Premium Dark/Gold Palette
                primary: '#D4AF37', // Gold
                secondary: '#1A1A1A', // Dark Gray/Black
                accent: '#F5F5DC', // Beige/Cream
                dark: '#0f0f0f',
                light: '#f5f5f5',
            },
            fontFamily: {
                sans: ['Inter', 'sans-serif'],
            },
        },
    },
    plugins: [],
}

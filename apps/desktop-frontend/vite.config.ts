import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

const sveltekitFix = async () => {
    const cwd = process.cwd();
    process.chdir(__dirname); // Temporarily change the working directory
    const plugin = await sveltekit(); // Load the SvelteKit plugin
    process.chdir(cwd); // Restore the original working directory
    return plugin;
};


export default defineConfig({
	plugins: [tailwindcss(), sveltekitFix()],
});

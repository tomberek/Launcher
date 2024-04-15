/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
	theme: {
	   extend: {
		 colors: {
			'teaclient': 'rgba(0,0,0,0.5)',
			'servers': '#180A24',
		   'teaclient': '#281734',
		   'servers-border': '#110C16'
		 },
	   },
	   fontFamily: {
		jbm: ["JetBrains Mono", "monospace"]
	   }
	},
	plugins: [],
}

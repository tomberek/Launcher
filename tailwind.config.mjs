/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
	theme: {
	   extend: {
		 backgroundColor: {
			'teaclient': 'rgba(0,0,0,0.5)'
		 },
		 borderColor: {
		   'teaclient': '#281734',
		 },
	   },
	},
	plugins: [],
}

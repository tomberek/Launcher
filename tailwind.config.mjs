/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
	theme: {
	   extend: {
		 backgroundColor: {
			'teaclient': 'rgba(0,0,0,0.37)'
		 },
		 borderColor: {
		   'teaclient': '#20102C',
		 },
	   },
	},
	plugins: [],
}

// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import consoleGrammar from './src/shiki/console-grammar.mjs';

const base = process.env.GITHUB_PAGES === 'true' ? '/aoc-ready-rust' : '/';

// https://astro.build/config
export default defineConfig({
	site: 'https://pratirupm.github.io',
	base,
	markdown: {
		shikiConfig: {
			langs: [consoleGrammar],
			langAlias: { console: 'book-console' },
		},
	},
	integrations: [
		starlight({
			title: 'Rust Before December',
			description: 'From your first program to your first Advent of Code, one small step at a time.',
			customCss: ['./src/styles/fonts.css', './src/styles/custom.css'],
			head: [
				{ tag: 'script', attrs: { src: `${base}/console-colors.js`, defer: true } },
				{ tag: 'script', attrs: { src: `${base}/a11y.js`, defer: true } },
				{
					tag: 'link',
					attrs: {
						rel: 'preload',
						href: `${base}/fonts/0xProto-Regular.woff2`,
						as: 'font',
						type: 'font/woff2',
						crossorigin: 'anonymous',
					},
				},
				{
					tag: 'link',
					attrs: {
						rel: 'preload',
						href: `${base}/fonts/0xProto-Bold.woff2`,
						as: 'font',
						type: 'font/woff2',
						crossorigin: 'anonymous',
					},
				},
			],
			lastUpdated: true,
			sidebar: [
				{
					label: 'Start here',
					collapsed: true,
					items: [
						{ label: 'Who this is for', slug: 'start/who-this-is-for' },
						{ label: 'What you will build', slug: 'start/what-you-will-build' },
						{ label: 'How this book works', slug: 'start/how-this-book-works' },
						{ label: 'Set up your workbench', slug: 'start/setup' },
						{ label: 'Meet Zed', slug: 'start/meet-zed' },
					],
				},
				{
					label: 'Ground floor',
					collapsed: true,
					items: [
						{ label: 'Open a terminal', slug: 'ground-floor/open-a-terminal' },
						{ label: 'Your first Rust program', slug: 'ground-floor/your-first-rust-program' },
						{ label: 'Values and decisions', slug: 'ground-floor/values-and-decisions' },
						{
							label: 'Lists, loops, and functions',
							slug: 'ground-floor/lists-loops-and-functions',
						},
						{
							label: 'Structs, enums, and errors',
							slug: 'ground-floor/structs-enums-and-errors',
						},
						{
							label: 'Borrowing without fear',
							slug: 'ground-floor/borrowing-without-fear',
						},
						{
							label: 'Tests and your first commit',
							slug: 'ground-floor/tests-and-your-first-commit',
						},
					],
				},
				{
					label: 'The seven days',
					items: [
						{ label: 'Day 1 · Read the input, draw the grid', slug: 'part-1/read-the-input' },
						{ label: 'Day 2 · Walk the grid, search the graph', slug: 'part-2/walk-the-grid' },
						{ label: 'Day 3 · Weigh the cost, order the queue', slug: 'part-3/weigh-the-cost' },
						{ label: 'Day 4 · Remember what you have seen', slug: 'part-4/remember-what-you-saw' },
						{ label: 'Day 5 · Simulate, then skip ahead', slug: 'part-5/simulate-then-skip' },
						{ label: 'Day 6 · Do the math, keep the intervals', slug: 'part-6/do-the-math' },
						{ label: 'Day 7 · Put it together', slug: 'part-7/put-it-together' },
					],
				},
				{
					label: 'Field guide',
					items: [
						{ label: 'The eleven years at a glance', slug: 'appendix/eleven-years' },
						{ label: 'The pattern index', slug: 'appendix/pattern-index' },
						{ label: 'The twelve-day playbook', slug: 'appendix/twelve-day-playbook' },
						{ label: 'Rust error decoder', slug: 'appendix/rust-error-decoder' },
						{ label: 'Cheat sheet', slug: 'appendix/cheat-sheet' },
						{ label: 'Glossary', slug: 'appendix/glossary' },
						{ label: 'About this book', slug: 'appendix/colophon' },
					],
				},
			],
		}),
	],
});

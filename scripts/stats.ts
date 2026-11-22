#!/usr/bin/env bun
import { readdir, readFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const docsDir = join(root, 'src/content/docs');
const args = process.argv.slice(2);
let filter = '';
let strict = false;
for (let i = 0; i < args.length; i++) {
	const arg = args[i] ?? '';
	if (arg === '--filter') filter = args[++i] ?? '';
	else if (arg === '--strict') strict = true;
	else if (!arg.startsWith('-')) filter = arg;
}

async function walk(dir: string): Promise<string[]> {
	const entries = await readdir(dir, { withFileTypes: true });
	const files: string[] = [];
	for (const entry of entries) {
		const full = join(dir, entry.name);
		if (entry.isDirectory()) files.push(...(await walk(full)));
		else if (/\.(?:md|mdx)$/.test(entry.name)) files.push(full);
	}
	return files.sort();
}

function proseWords(source: string): number {
	const lines = source.split(/\r?\n/);
	let inFrontmatter = false;
	let frontmatterDone = false;
	let inFence = false;
	let inComment = false;
	const prose: string[] = [];

	for (let i = 0; i < lines.length; i++) {
		const raw = lines[i] ?? '';
		if (!frontmatterDone && i === 0 && raw.trim() === '---') {
			inFrontmatter = true;
			continue;
		}
		if (inFrontmatter) {
			if (raw.trim() === '---') {
				inFrontmatter = false;
				frontmatterDone = true;
			}
			continue;
		}
		if (!frontmatterDone && raw.trim() !== '') frontmatterDone = true;

		if (/^\s*(?:```|~~~)/.test(raw)) {
			inFence = !inFence;
			continue;
		}
		if (inFence) continue;

		if (inComment) {
			if (raw.includes('-->')) inComment = false;
			continue;
		}
		if (raw.includes('<!--') && !raw.includes('-->')) {
			inComment = true;
			continue;
		}
		if (raw.trim().startsWith('<!--')) continue;
		if (/^\s*(?:import|export)\s/.test(raw)) continue;

		prose.push(
			raw
				.replace(/\]\([^)]*\)/g, ']')
				.replace(/https?:\/\/\S+/g, ' ')
				.replace(/<[^>]+>/g, ' ')
				.replace(/[`*_~#>|]/g, ' '),
		);
	}

	const text = prose.join(' ');
	return (text.match(/[A-Za-z0-9][A-Za-z0-9'’_-]*/g) ?? []).length;
}

const allFiles = await walk(docsDir);
const files = filter
	? allFiles.filter((file) => relative(root, file).includes(filter))
	: allFiles;
if (filter && files.length === 0) {
	console.error(`No pages matched filter "${filter}".`);
	process.exit(1);
}

let total = 0;
let flagged = 0;

for (const fullPath of files) {
	const file = relative(root, fullPath);
	const words = proseWords(await readFile(fullPath, 'utf8'));
	total += words;
	const isChapter = file.includes('/part-');
	const short = isChapter && words < 1200;
	const long = isChapter && words > 4000;
	if (short || long) flagged++;
	const mark = short ? 'short' : long ? 'long ' : 'ok   ';
	console.log(`${mark}  ${String(words).padStart(5)}  ${file}`);
}

console.log(`\n${files.length} pages, ${total} prose words, ${flagged} outside 1,200–4,000.`);

if (strict && flagged > 0) process.exit(1);

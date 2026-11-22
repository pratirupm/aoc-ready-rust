#!/usr/bin/env bun
/**
 * Guard against drift between the book's code blocks and the lab.
 *
 * 1. Every `rust` block whose title names a file inside `rust-aoc-lab/` must
 *    be an exact excerpt: each non-`...` line appears in the file, in order.
 * 2. Every file in `rust-aoc-lab/data/examples/` must appear verbatim as a
 *    `text` block somewhere in the book.
 */
import { readdir, readFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const docsDir = join(root, 'src/content/docs');
const labDir = join(root, 'rust-aoc-lab');

type Fence = {
	file: string;
	line: number;
	lang: string;
	title: string;
	code: string;
};

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

async function collectFences(): Promise<Fence[]> {
	const fences: Fence[] = [];

	for (const full of await walk(docsDir)) {
		const source = await readFile(full, 'utf8');
		const lines = source.split(/\r?\n/);
		let inFrontmatter = false;
		let frontmatterDone = false;
		let fence: Fence | null = null;

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

			const match = /^(`{3,}|~{3,})(.*)$/.exec(raw);
			if (match) {
				if (!fence) {
					const info = (match[2] ?? '').trim();
					const lang = info.split(/\s+/)[0] ?? '';
					const title =
						/title=(?:"([^"]*)"|'([^']*)')/.exec(info)?.[1] ??
						/title=(?:"([^"]*)"|'([^']*)')/.exec(info)?.[2] ??
						'';
					fence = {
						file: relative(root, full),
						line: i + 1,
						lang,
						title,
						code: '',
					};
				} else if (raw.trimStart().startsWith(match[1] ?? '```')) {
					fence.code = fence.code.replace(/\n$/, '');
					fences.push(fence);
					fence = null;
				} else {
					fence.code += raw + '\n';
				}
				continue;
			}

			if (fence) fence.code += raw + '\n';
		}
	}

	return fences;
}

function labFileForTitle(title: string): string | null {
	if (title.startsWith('rust-aoc-lab/')) return join(root, title.slice('rust-aoc-lab/'.length));

	if (!title.startsWith('src/')) return null;

	const candidate = join(labDir, title);
	return candidate;
}

const problems: string[] = [];
const fences = await collectFences();
let excerptsChecked = 0;

for (const fence of fences) {
	if (fence.lang !== 'rust' || !fence.title) continue;

	const target = labFileForTitle(fence.title);
	if (!target) continue;

	let source: string;
	try {
		source = await readFile(target, 'utf8');
	} catch {
		// A teaching file such as `src/main.rs` or a planned `src/bin/day01.rs`
		// does not exist in the lab. Only check files that do.
		continue;
	}

	const targetLines = source.split('\n').map((line) => line.trim());
	const wanted = fence.code
		.split('\n')
		.map((line) => line.trim())
		.filter((line) => line && line !== '...');

	let cursor = 0;
	for (const line of wanted) {
		const found = targetLines.indexOf(line, cursor);
		if (found === -1) {
			problems.push(
				`${fence.file}:${fence.line}  title "${fence.title}" no longer matches ` +
					`${relative(root, target)}: missing "${line}"`,
			);
			break;
		}
		cursor = found + 1;
	}

	excerptsChecked++;
}

const examplesDir = join(labDir, 'data/examples');
const exampleFiles = (await readdir(examplesDir)).filter((name) => name.endsWith('.txt')).sort();
const textBlocks = fences
	.filter((fence) => fence.lang === 'text')
	.map((fence) => fence.code.trim());

let examplesChecked = 0;
for (const name of exampleFiles) {
	const content = (await readFile(join(examplesDir, name), 'utf8')).trim();
	if (!textBlocks.includes(content)) {
		problems.push(
			`${relative(root, join(examplesDir, name))} has no byte-identical text block in the book`,
		);
	}
	examplesChecked++;
}

for (const problem of problems) console.log(problem);
console.log(
	`\n${excerptsChecked} lab excerpts and ${examplesChecked} example inputs checked: ` +
		`${problems.length} problem(s).`,
);

if (problems.length > 0) process.exit(1);

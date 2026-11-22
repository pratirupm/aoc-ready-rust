#!/usr/bin/env bun
import { readdir, readFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const docsDir = join(root, 'src/content/docs');
const args = process.argv.slice(2);
let filter = '';
for (let i = 0; i < args.length; i++) {
	const arg = args[i] ?? '';
	if (arg === '--filter') filter = args[++i] ?? '';
	else if (!arg.startsWith('-')) filter = arg;
}

type Severity = 'error' | 'warning';

type Finding = {
	file: string;
	line: number;
	severity: Severity;
	rule: string;
	message: string;
};

const findings: Finding[] = [];

const banned: Array<{
	pattern: RegExp;
	rule: string;
	message: string;
	severity: Severity;
}> = [
	{
		pattern: /\b(?:instantiate|invoke|utilize|leverage)\b/gi,
		rule: 'vocabulary',
		message: 'use a plainer verb: create, call, or use',
		severity: 'error',
	},
	{
		pattern: /\bprior to\b/gi,
		rule: 'vocabulary',
		message: 'prefer "before"',
		severity: 'error',
	},
	{
		pattern: /\bin order to\b/gi,
		rule: 'vocabulary',
		message: 'prefer "to"',
		severity: 'error',
	},
	{
		pattern: /\brefactor(?:ing|ed)?\b/gi,
		rule: 'vocabulary',
		message: 'prefer "clean up the code"',
		severity: 'error',
	},
	{
		pattern: /\bexecute(?:s|d|ing)?\b/gi,
		rule: 'vocabulary',
		message: 'prefer "run"',
		severity: 'error',
	},
	{
		pattern: /\brepo\b/gi,
		rule: 'vocabulary',
		message: 'prefer "repository" or "project"',
		severity: 'error',
	},
	{
		pattern: /\b(?:red|green) tests?\b/gi,
		rule: 'test-state',
		message: 'say "a failing test" or "a passing test"',
		severity: 'error',
	},
	{
		pattern: /\bcheckpoints?\b/gi,
		rule: 'test-state',
		message: 'say "commit" or "save the state"',
		severity: 'error',
	},
	{
		pattern: /\b(?:CI|continuous integration|workflows?|pipelines?)\b/g,
		rule: 'ci',
		message: 'the book does not teach CI',
		severity: 'error',
	},
	{
		pattern: /\bchapter\s+\d+\b/gi,
		rule: 'cross-reference',
		message: 'no chapter numbers; say "later" or "next"',
		severity: 'error',
	},
	{
		pattern: /\bin this chapter\b/gi,
		rule: 'meta',
		message: 'drop the meta opening',
		severity: 'error',
	},
	{
		pattern: /\b(?:answer key|you build)\b/gi,
		rule: 'meta',
		message: 'remove the exercise-block language',
		severity: 'error',
	},
	{
		pattern:
			/\b(?:takes?|needs?|in|under|about|around)\s+\d+\s+(?:minutes?|hours?|days?|weeks?)\b/gi,
		rule: 'time-estimate',
		message: 'remove the time estimate',
		severity: 'error',
	},
	{
		pattern: /\b\d+[- ](?:minute|hour|day|week)s?\b/gi,
		rule: 'time-estimate',
		message: 'remove the time estimate',
		severity: 'error',
	},
	{
		pattern: /\bjust\b/gi,
		rule: 'tone',
		message: '"just" can make a stuck reader feel behind',
		severity: 'warning',
	},
	{
		pattern: /\bsimply\b/gi,
		rule: 'tone',
		message: 'drop "simply"',
		severity: 'warning',
	},
	{
		pattern: /\bobviously\b/gi,
		rule: 'tone',
		message: 'drop "obviously"',
		severity: 'warning',
	},
	{
		pattern: /\bof course\b/gi,
		rule: 'tone',
		message: 'drop "of course"',
		severity: 'warning',
	},
	{
		pattern: /\beasy\b/gi,
		rule: 'tone',
		message: '"easy" is discouraging when it is not',
		severity: 'warning',
	},
];

const fencePattern = /^(`{3,}|~{3,})(.*)$/;

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

function add(file: string, line: number, severity: Severity, rule: string, message: string) {
	findings.push({ file, line, severity, rule, message });
}

function stripInline(text: string): string {
	return text
		.replace(/`[^`]*`/g, '')
		.replace(/https?:\/\/\S+/g, '')
		.replace(/\]\([^)]*\)/g, ']');
}

function checkProse(file: string, line: number, text: string) {
	const clean = stripInline(text);
	if (!clean.trim()) return;
	for (const rule of banned) {
		rule.pattern.lastIndex = 0;
		const match = rule.pattern.exec(clean);
		if (match) {
			add(file, line, rule.severity, rule.rule, `${rule.message} (found "${match[0]}")`);
		}
	}
}

async function lintFile(fullPath: string) {
	const file = relative(root, fullPath);
	const source = await readFile(fullPath, 'utf8');
	const lines = source.split(/\r?\n/);

	let inFrontmatter = false;
	let frontmatterDone = false;
	let inComment = false;
	let fence: {
		marker: string;
		lang: string;
		title: string;
		start: number;
		lines: string[];
		hadTranscript: boolean;
	} | null = null;

	for (let i = 0; i < lines.length; i++) {
		const raw = lines[i] ?? '';
		const lineNo = i + 1;

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
		if (!frontmatterDone && raw.trim() !== '') {
			frontmatterDone = true;
		}

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

		const fenceMatch = raw.match(fencePattern);
		if (fenceMatch) {
			if (!fence) {
				const info = (fenceMatch[2] ?? '').trim();
				const lang = info.split(/\s+/)[0] ?? '';
				const title = /title=(?:"([^"]*)"|'([^']*)')/.exec(info);
				let hadTranscript = false;
				for (let j = i - 1; j >= 0; j--) {
					const previous = (lines[j] ?? '').trim();
					if (!previous) continue;
					hadTranscript = /<!--\s*transcript:/.test(previous);
					break;
				}
				fence = {
					marker: fenceMatch[1] ?? '```',
					lang,
					title: title?.[1] ?? title?.[2] ?? '',
					start: lineNo,
					lines: [],
					hadTranscript,
				};
			} else if (raw.trimStart().startsWith(fence.marker)) {
				if (!fence.lang) {
					add(file, fence.start, 'error', 'code-language', 'code block needs a language');
				}
				// Complete config files and full functions need a little more
				// room than a short snippet; transcripts are also allowed to run.
				const limit =
					fence.lang === 'json' || fence.lang === 'console' || fence.title ? 40 : 25;
				if (fence.lines.length > limit && !(fence.lang === 'json' && fence.title)) {
					add(
						file,
						fence.start,
						'warning',
						'code-length',
						`code block has ${fence.lines.length} lines; aim for ${limit} or fewer`,
					);
				}
				if (fence.lang === 'console' && !fence.lines.some((l) => l.trimStart().startsWith('$'))) {
					add(file, fence.start, 'warning', 'console-prompt', 'console block has no $ command');
				}
				let nextLine = '';
				for (let j = i + 1; j < lines.length; j++) {
					const candidate = (lines[j] ?? '').trim();
					if (candidate) {
						nextLine = candidate;
						break;
					}
				}
				if (!nextLine || nextLine.startsWith('```') || nextLine.startsWith('#')) {
					add(
						file,
						lineNo,
						'warning',
						'code-followup',
						'code block should be followed by a sentence',
					);
				}
				fence = null;
			} else {
				fence.lines.push(raw);
			}
			continue;
		}

		if (fence) {
			fence.lines.push(raw);
			continue;
		}

		checkProse(file, lineNo, raw);
	}

	if (fence) {
		add(file, fence.start, 'error', 'code-fence', 'code block is never closed');
	}

	if (!/^---/m.test(source.split('\n')[0] ?? '')) {
		add(file, 1, 'error', 'frontmatter', 'page needs frontmatter with a title');
	} else {
		const front = source.match(/^---\r?\n([\s\S]*?)\r?\n---/);
		if (!front || !/^title:\s*\S/m.test(front[1] ?? '')) {
			add(file, 1, 'error', 'frontmatter', 'frontmatter needs a title');
		} else if (!/^description:\s*\S/m.test(front[1] ?? '')) {
			add(file, 1, 'warning', 'frontmatter', 'frontmatter has no description');
		}
	}
}

const allFiles = await walk(docsDir);
const files = filter
	? allFiles.filter((file) => relative(root, file).includes(filter))
	: allFiles;
if (filter && files.length === 0) {
	console.error(`No pages matched filter "${filter}".`);
	process.exit(1);
}
for (const file of files) await lintFile(file);

const errors = findings.filter((f) => f.severity === 'error');
const warnings = findings.filter((f) => f.severity === 'warning');

for (const finding of [...errors, ...warnings]) {
	const label = finding.severity === 'error' ? 'error' : 'warning';
	console.log(`${finding.file}:${finding.line}  ${label}  ${finding.rule}  ${finding.message}`);
}

console.log(
	`\n${files.length} pages checked: ${errors.length} error(s), ${warnings.length} warning(s).`,
);

if (errors.length > 0) process.exit(1);

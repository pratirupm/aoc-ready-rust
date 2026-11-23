// Small a11y repairs for Starlight widgets and Expressive Code.
(() => {
	const setupMenu = () => {
		const button = document.querySelector('.sl-menu-button');
		const sidebar = document.querySelector('#starlight__sidebar');
		if (!button || !sidebar) return;
		if (button.dataset.a11yReady === 'true') return;

		button.dataset.a11yReady = 'true';
		button.setAttribute('aria-controls', sidebar.id);

		const sync = () => {
			const open = sidebar.matches(':popover-open') || sidebar.hasAttribute('data-open');
			button.setAttribute('aria-expanded', String(open));
		};

		button.addEventListener('click', () => setTimeout(sync, 0));
		sidebar.addEventListener('toggle', sync);
		sync();
	};

	const labelSearchDialog = () => {
		const dialog = document.querySelector('dialog');
		if (dialog && !dialog.hasAttribute('aria-label')) {
			dialog.setAttribute('aria-label', 'Search');
		}
	};

	const setupTabs = () => {
		const headings = [...document.querySelectorAll('h1, h2, h3, h4, h5, h6')];
		document.querySelectorAll('[role="tablist"]').forEach((list) => {
			if (list.dataset.a11yReady !== 'true') {
				list.dataset.a11yReady = 'true';

				const preceding = headings.findLast(
					(heading) => heading.compareDocumentPosition(list) & Node.DOCUMENT_POSITION_FOLLOWING,
				);
				if (preceding && !list.hasAttribute('aria-label')) {
					list.setAttribute('aria-label', preceding.textContent.trim());
				}

				list.querySelectorAll('[role="tab"]').forEach((tab) => {
					const href = tab.getAttribute('href');
					if (href?.startsWith('#') && !tab.hasAttribute('aria-controls')) {
						tab.setAttribute('aria-controls', href.slice(1));
					}
				});
			}
		});
	};

	const labelCode = () => {
		document.querySelectorAll('.expressive-code figure.frame').forEach((frame) => {
			const pre = frame.querySelector('pre');
			if (!pre) return;

			if (!pre.hasAttribute('aria-label')) {
				const title = frame.querySelector('.header .title')?.textContent.trim();
				const terminal = frame.querySelector('.header .sr-only')?.textContent.trim();
				pre.setAttribute('aria-label', title ? `Code: ${title}` : (terminal ?? 'Code block'));
			}

			frame
				.querySelectorAll('button[title="Copy to clipboard"]')
				.forEach((button) => button.setAttribute('aria-label', 'Copy code to clipboard'));
		});
	};

	const markScrollableCode = () => {
		document.querySelectorAll('.expressive-code pre').forEach((pre) => {
			pre.closest('.frame')?.classList.toggle('is-scrollable', pre.scrollWidth > pre.clientWidth + 1);
		});
	};

	const setup = () => {
		setupMenu();
		labelSearchDialog();
		setupTabs();
		labelCode();
		markScrollableCode();
	};

	setup();
	window.addEventListener('load', setup);
	window.addEventListener('resize', markScrollableCode);

	const observer = new MutationObserver(setup);
	observer.observe(document.documentElement, { childList: true, subtree: true });
	document.addEventListener('click', () => setTimeout(setup, 0));
})();

// Small a11y repairs for two Starlight widgets.
// - The mobile menu button does not expose whether the drawer is open.
// - The Pagefind dialog has no accessible name.
(() => {
	const button = document.querySelector('.sl-menu-button');
	const sidebar = document.querySelector('#starlight__sidebar');

	if (button && sidebar) {
		button.setAttribute('aria-controls', sidebar.id);
		const sync = () => {
			const open = sidebar.matches(':popover-open') || sidebar.hasAttribute('data-open');
			button.setAttribute('aria-expanded', String(open));
		};
		button.addEventListener('click', () => setTimeout(sync, 0));
		sidebar.addEventListener('toggle', sync);
		sync();
	}

	const labelSearchDialog = () => {
		const dialog = document.querySelector('dialog');
		if (dialog && !dialog.hasAttribute('aria-label')) {
			dialog.setAttribute('aria-label', 'Search');
		}
	};

	labelSearchDialog();
	document.addEventListener('click', () => setTimeout(labelSearchDialog, 0));
})();

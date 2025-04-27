window.MathJax = {
	tex: {
		inlineMath: [["$", "$"]],
		displayMath: [["$$", "$$"]],
	},
	startup: {
		pageReady: () => {
			return MathJax.startup.defaultPageReady().then(() => {
				// After MathJax is ready, process the spans
				for (const span of document.querySelectorAll("span[data-math-style]")) {
					const mathText = span.textContent;
					const isDisplay = span.getAttribute("data-math-style") === "display";
					span.innerHTML = isDisplay ? `$$${mathText}$$` : `$${mathText}$`;
				}
				// Typeset the new content
				MathJax.typeset();
			});
		},
	},
	options: {
		renderActions: {
			addMenu: [], // disable the MathJax menu
		},
	},
};

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

					// If the math span is followed by punctuation, insert an NBSP to
					// discourage line wrapping engines from wrapping between a math
					// expression and punctuation, e.g. `$a+b$,` should not have a line
					// break between the inline math and comma.
					const nextNode = span.nextSibling;
					if (nextNode && nextNode.nodeType === Node.TEXT_NODE) {
						const nextText = nextNode.textContent;
						if (nextText.match(/^[.,!?;:]/) !== null) {
							// \u202f is NNBSP, narrow non-breaking space
							// \u00a0, the NBSP, is a full-width space in theory
							// \u2060 the zero-width nobreak should be used
							// here. but chrome doesn't appear to handle it
							// correctly because it will still insert a line
							// break so :shrug:
							nextNode.textContent = `\u202f${nextText}`;
						}
					}
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

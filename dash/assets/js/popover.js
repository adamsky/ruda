var popoverTriggerList = [].slice.call(document.querySelectorAll('[data-bs-toggle="popover"]'));
popoverTriggerList.map(function (popoverTriggerEl) {
	let options = {
		delay: {show: 50, hide: 50},
		html: popoverTriggerEl.getAttribute('data-bs-html') === "true" ?? false,
		placement: popoverTriggerEl.getAttribute('data-bs-placement') ?? 'auto'
	};
	return new bootstrap.Popover(popoverTriggerEl, options);
});

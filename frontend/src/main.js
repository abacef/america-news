import App from './App.svelte';

const app = new App({
	target: document.body,
	props: {
		name_from_prop: 'Aaron'
	}
});

export default app;
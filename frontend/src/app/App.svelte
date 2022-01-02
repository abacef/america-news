<script>
	// export let name_from_prop;

	import {getGreeting} from "./api"

	import NavBar from "./NavBar.svelte"

	import Router from 'svelte-spa-router'

    import Home from "./routes/Home.svelte";
    import About from "./routes/About.svelte";
    import Blog from "./routes/Blog.svelte";
	import NotFound from "./routes/NotFound.svelte";

	const routes = {
		'/': Home,
		'/about': About,
		'/blog': Blog,
		'*': NotFound,
	}

</script>


<NavBar/>

<!-- Where the content of what is rendered shows up -->
<Router {routes}/>

{#await getGreeting()}
	<p>...waiting for response from rocket</p>
{:then rocket_resp}
	<p>The response is: {rocket_resp}</p>
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}



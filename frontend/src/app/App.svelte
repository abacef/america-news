<script>
	// export let name_from_prop;

	import {getGreeting} from "./api"

	import NavBar from "./NavBar.svelte"

	import { Router, Route } from "svelte-routing";
    import Home from "./routes/Home.svelte";
    import About from "./routes/About.svelte";
    import Blog from "./routes/Blog.svelte";

</script>

<Router url="">
	<NavBar/>

	<div>
		<Route path="blog" component="{Blog}" />
		<Route path="about" component="{About}" />
		<Route path="/"><Home /></Route>
	</div>

	{#await getGreeting()}
		<p>...waiting for response from rocket</p>
	{:then rocket_resp}
		<p>The response is: {rocket_resp}</p>
	{:catch error}
		<p style="color: red">{error.message}</p>
	{/await}
</Router>


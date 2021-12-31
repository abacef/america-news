<script>
	export let name_from_prop;

	import NavBar from "./NavBar.svelte"

	async function getRocketResp() {
		const res = await fetch('http://localhost:8000/api/hello');
		const content = await res.json();

		if (res.ok) {
			return content.greeting;
		} else {
			throw new Error(body);
		}
	}

</script>

<NavBar></NavBar>

{#await getRocketResp()}
	<p>...waiting for response from rocket</p>
{:then rocket_resp}
	<p>The response is: {rocket_resp}</p>
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}
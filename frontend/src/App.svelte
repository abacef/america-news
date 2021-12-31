<script>
	export let name_from_prop;

	let name_from_rocket_promise = getRocketResp()

	async function getRocketResp() {
		const res = await fetch('http://localhost:8000/api/hello');
		console.log(res)
		const content = await res.text();
		
		if (res.ok) {
			return content;
		} else {
			throw new Error(body);
		}
	}

</script>

{#await name_from_rocket_promise}
	<p>...waiting for response from rocket</p>
{:then rocket_resp}
	<p>The response is: {rocket_resp}</p>
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}
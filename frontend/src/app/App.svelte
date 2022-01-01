<script>
	export let name_from_prop;

	import NavBar from "./NavBar.svelte"

	import {getGreeting} from "./api"

</script>



<button>Click me</button>

{#await getGreeting()}
	<p>...waiting for response from rocket</p>
{:then rocket_resp}
	<p>The response is: {rocket_resp}</p>
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}



<!-- <NavBar/> -->


<!-- <div class="window" style="margin: 32px; width: 250px">
	<div class="title-bar">
	<div class="title-bar-text">
		My First VB4 Program
	</div>

	<div class="title-bar-controls">
		<button aria-label="Minimize"></button>
		<button aria-label="Maximize"></button>
		<button aria-label="Close"></button>
	</div>
	</div>
	<div class="window-body">
	<p>Hello, world!</p>
	<section class="field-row" style="justify-content: flex-end">
		<button>OK</button>
		<button>Cancel</button>
	</section>
	</div>
</div> -->




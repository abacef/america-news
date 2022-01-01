<script>
	export let name_from_prop;

	import {getGreeting} from "./api"

</script>

<table width="100%">
	<tr>
		<td>
			<button>click for nav</button>
		</td>
		<td align="right">
			<button>sign in</button>
		</td>
	</tr>
</table>



{#await getGreeting()}
	<p>...waiting for response from rocket</p>
{:then rocket_resp}
	<p>The response is: {rocket_resp}</p>
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}



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




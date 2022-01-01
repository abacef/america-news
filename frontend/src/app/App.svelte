<script>
	// export let name_from_prop;

	import {getGreeting} from "./api"

    import Modal from './Modal.svelte';

    let isOpenModal = false;

</script>


<table width="100%" border="0" cellspacing="0" cellpadding="0">
	<tr>
		<td>
			<select>
				<option>5 - Incredible!</option>
				<option>4 - Great!</option>
				<option>3 - Pretty good</option>
				<option>2 - Not so great</option>
				<option>1 - Unfortunate</option>
			  </select>
		</td>
		<td align="right">
			<button on:click={() => isOpenModal = true}>Sign In</button>
		</td>
	</tr>
</table>

<Modal isOpenModal={isOpenModal} on:closeModal={() => isOpenModal = false} />


{#await getGreeting()}
	<p>...waiting for response from rocket</p>
{:then rocket_resp}
	<p>The response is: {rocket_resp}</p>
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}

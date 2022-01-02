<script>

	import {push} from 'svelte-spa-router'

</script>

<table width="100%" border="0" cellspacing="0" cellpadding="0">
	<tr>
		<td>
			<select>
				<option on:click={() => push("/")}>Home</option>
				<option on:click={() => push("/about")}>About</option>
				<option on:click={() => push("/blog")}>Blog</option>
			</select>
		</td>
		<td align="right" style="vertical-align:top">
			<button on:click={() => push("/sign-in")}>Sign In</button>
		</td>
	</tr>
</table>
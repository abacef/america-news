<script>

	let hamburgerMenuOpen = false;

</script>

<table width="100%" border="0" cellspacing="0" cellpadding="0">
	<tr>
		<td>
			<button on:click={() => hamburgerMenuOpen = !hamburgerMenuOpen}>☰</button>
			{#if hamburgerMenuOpen}
				<ul class="tree-view" style="width:63px">
					<li><a href="#/">Home</a></li>
					<li><a href="#/about">About</a></li>
					<li><a href="#/blog">Blog</a></li>
				</ul>
			{/if}
		</td>
		<td align="right" style="vertical-align:top">
			<button>Sign In</button>
		</td>
	</tr>
</table>
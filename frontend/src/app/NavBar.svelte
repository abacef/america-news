<script>
	import { Link } from "svelte-routing";

</script>

<table width="100%" border="0" cellspacing="0" cellpadding="0">
	<tr>
		<td>
			<ul class="tree-view" style="width:100px">
				<li>
					<details>
						<summary>☰</summary>
						<ul>
							<li><Link to="/">Home</Link></li>
							<li><Link to="about">About</Link></li>
							<li><Link to="blog">Blog</Link></li>
						</ul>
					</details>
				</li>
			</ul>
		</td>
		<td align="right" style="vertical-align:top">
			<button>Sign In</button>
		</td>
	</tr>
</table>
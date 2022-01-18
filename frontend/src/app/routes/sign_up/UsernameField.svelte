<script>

    import users from "../../logins";

    let username = "";

    let usernameIsAvailable = null;

    let statusClass = "";

    $: if (username === "") {
        usernameIsAvailable = null
    } else {
        usernameIsAvailable = users.isUsernameAvailable(username)
    }

    $: if (usernameIsAvailable === false) {
        statusClass = "is-danger"
    } else if (usernameIsAvailable === true) {
        statusClass = "is-success"
    } else {
        statusClass = "";
    }

</script>

<div class="field">
    <label class="label">Username</label>
    <div class="control has-icons-right">
        <input class="input {statusClass}" type="text" bind:value={username} placeholder="Choose a username">
        {#if usernameIsAvailable === true}
            <span class="icon is-small is-right">
                <i class="fas fa-check"></i>
            </span>
        {:else if usernameIsAvailable === false}
            <span class="icon is-small is-right">
                <i class="fas fa-exclamation-triangle"></i>
            </span>
        {/if}
    </div>
    {#if usernameIsAvailable === true}
        <p class="help is-success">This username is available</p>
    {:else if usernameIsAvailable === false}
        <p class="help is-danger">This email is invalid</p>
    {:else}
        <p/>
    {/if}
</div>
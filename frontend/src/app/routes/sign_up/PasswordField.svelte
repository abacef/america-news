<script>

    import users from "../../logins";

    let password = "";

    let passwordIsValid = null;

    let statusClass = "";

    let passwordIncorrectText = null;

    $: if (password === "") {
        passwordIsValid = null;
        passwordIncorrectText = null;
    } else {
        let isPasswordValid = users.passwordIsValid(password)
        if (isPasswordValid === true) {
            passwordIsValid = true;
            passwordIncorrectText = null;
        } else {
            passwordIsValid = false;
            passwordIncorrectText = isPasswordValid;
        }
    }

    $: if (passwordIsValid === false) {
        statusClass = "is-danger"
    } else if (passwordIsValid === true) {
        statusClass = "is-success"
    } else {
        statusClass = "";
    }

</script>

<div class="field">
    <label class="label">Password</label>
    <div class="control has-icons-right">
        <input class="input {statusClass}" type="text" bind:value={password} placeholder="Choose a password">
        {#if passwordIsValid === true}
            <span class="icon is-small is-right">
                <i class="fas fa-check"></i>
            </span>
        {:else if passwordIsValid === false}
            <span class="icon is-small is-right">
                <i class="fas fa-exclamation-triangle"></i>
            </span>
        {/if}
    </div>
    {#if passwordIsValid === true}
        <p class="help is-success">This password is valid</p>
    {:else if passwordIsValid === false}
        <p class="help is-danger">{passwordIncorrectText}</p>
    {:else}
        <p/>
    {/if}
</div>
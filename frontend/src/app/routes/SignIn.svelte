<script>

    import users from "../logins";

    $: errorMessageUsername = username === "bad" ? "❌ username is bad" : "";
    $: errorMessagePassword = password === "bad" ? "❌ password is bad" : "";


    let username = "";
    let password = "";

    let serverErrorMessage = "";

    function signIn() {
        if (!users.addUser(username, password)) {
            serverErrorMessage = "Username already in use. Choose another one";
        } else {
            serverErrorMessage = "yay";
        }
    }


</script>

<div class="window" style="width: 300px; margin: 0px auto;">
    <div class="title-bar">
        <div class="title-bar-text">Sign In</div>
    </div>
    <div class="window-body">
        <div class="field-row-stacked" style="width: 200px">
            <input id="username-field" type="text" placeholder="Username" bind:value={username}/>
            <label for="username-field">{errorMessageUsername}</label>
        </div>
        <br>
        <div class="field-row-stacked" style="width: 200px">
            <input id="password-field" type="text" placeholder="Password" bind:value={password}/>
            <label for="password-field">{errorMessagePassword}</label>
        </div>
        <br>
        {serverErrorMessage}
        <table width="100%" border="0" cellspacing="0" cellpadding="0">
            <tr>
                <td align="right">
                    <button disabled={errorMessageUsername != "" || errorMessagePassword != ""} on:click={signIn}>Sign Up</button>
                </td>
            </tr>
        </table>
    </div>
</div>
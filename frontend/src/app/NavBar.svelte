<script>

    import {push} from 'svelte-spa-router'

    var isBurgerOpen = false;

    function toggleBurgerMenu() {
        isBurgerOpen = !isBurgerOpen
    }

</script>

<!-- Basically coppied this code https://bulma.io/documentation/components/navbar/#basic-navbar -->
<nav class="navbar" role="navigation" aria-label="main navigation">
    <div class="navbar-brand">
        <a class="navbar-item" href="/">
            <img src="/favicon.png">
        </a>
  
        <a role="button" class="navbar-burger {isBurgerOpen ? "is-active" : ""}" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample" on:click={toggleBurgerMenu}>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
        </a>
    </div>
  
    <div id="navbarBasicExample" class="navbar-menu {isBurgerOpen ? "is-active" : ""}">
        <div class="navbar-start">
            <a class="navbar-item" href="/#/about">
                About
            </a>

            <a class="navbar-item" href="/#/blog">
                Blog
            </a>
        </div>
  
        <div class="navbar-end">
            <div class="navbar-item">
                <div class="buttons">
                    <a class="button is-primary">
                    <strong>Sign up</strong>
                    </a>
                    <a class="button is-light">
                    Log in
                    </a>
                </div>
            </div>
        </div>
    </div>
</nav>

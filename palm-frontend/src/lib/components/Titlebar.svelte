<style>
	.title-wrapper {
        --header-outer-height: 95px;
        --header-inner-height: 70px;
        --header-height-difference: calc(var(--header-outer-height) - var(--header-inner-height));
        --header-bg: #fff;
    }

    .responsive-wrapper {
        width: 90%;
        max-width: 1280px;
        margin-left: auto;
        margin-right: auto;
    }

    /* Sticky header */
    .header-outer {
        /* Make it stick */
        height: var(--header-outer-height);
        position: sticky;
        top: calc(var(--header-height-difference) * -1); /* Multiply by -1 to get a negative value */
        display: flex;
        align-items: center;

        /* Other */
        background-color: var(--header-bg);
        box-shadow: 0 2px 10px 0 rgba(0,0,0, 0.1);
    }

    .header-inner {
        /* Make it stick */
        height: var(--header-inner-height);
        position: sticky;
        top: 0;

        /* Other */
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    a, a:link, a:visited, a:hover, a:active {
        text-decoration: none;
    }

    .header-logo {
        /* height: calc(var(--header-inner-height) - 30px); */
        font-size: 2em;
        cursor: pointer;

        user-select: none;

        color: #000;
    }

    a .header-logo {
        text-decoration: none;
    }

    .header-navigation {
        display: flex;
        flex-wrap: wrap;
    }

    .header-navigation a {
        font-size: 1.125rem;
        color: inherit;
        margin-left: 1.75rem;
        position: relative;
        font-weight: 500;
        text-decoration: none;
    }

    .header-navigation a:hover:after {
        transform: scalex(1);
    }

    .header-navigation a:after {
        transition: 0.25s ease;
        content: "";
        display: block;
        width: 100%;
        height: 2px;
        background-color: currentcolor;
        transform: scalex(0);
        position: absolute;
        bottom: -2px;
        left: 0;
    }

</style>

<header class="header-outer title-wrapper">
    <div class="header-inner responsive-wrapper">
        <a href="/"><div class="header-logo">
            <!-- <i class="fa-solid fa-leaf"></i> -->
            <i class="fa-brands fa-envira"></i>
            <!-- <i class="fa-regular fa-hand"></i> -->
            <!-- <i style="font-size: 3em;" class="fa-solid fa-hands"></i> -->
            Palm
        </div></a>
        <!-- TODO: Only when logged-in. Also, check if admin is active. -->
        <nav class="header-navigation">
			<a href="#">Admin</a>
			<a href="#">Profile</a>
		</nav>
    </div>
</header>
<script lang="ts">
    import Widget from "./Widget.svelte";

    let email = $state('');
    let password = $state('');
    let error = $state('');
    let loading = $state(false);

    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault();
        loading = true;
        error = '';

        try {
            const res = await fetch('10.0.0.0/login', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ email, password })
            });

            if (!res.ok) {
                error = (await res.json().catch(() => ({}))).message ?? 'Login failed';
                return;
            }

            // const { token } = await res.json();
            // auth.set(token);
            // goto('/dashboard');
        } catch {
            error = 'Network error';
        } finally {
            loading = false;
        }
    }

</script>

<!-- Style starter code comes from https://codefronts.com/components/css-login-forms/floating-label-login/ -->
<style>
    form {
        --accent: oklch(35.135% 0.00138 198.686);
        --accent-button: oklch(15.434% 0.00002 271.152);
        --field-h: 56px;
        display: flex;
        flex-direction: column;
        gap: 22px;
    }

    form *, form *::before, form *::after {
        box-sizing: border-box;
        margin: 0;
        padding: 0;
    }

    .form-input {
        position: relative;
    }

    input {
        width: 100%;
        height: var(--field-h);
        border: 0;
        border-bottom: 2px solid #cdd6dd;
        background: transparent;
        padding: 18px 2px 4px;
        font-size: 16px;
        color: #12222b;
        transition: border-color .2s;
    }
    
    input:focus-visible {
        outline:none
    }
    
    .form-input:focus-within input {
        border-bottom-color:var(--accent)
    }

    .label {
        position: absolute;
        left: 2px;
        top: 18px;
        font-size: 16px;
        color: #8695a0;
        pointer-events: none;
        transform-origin: left top;
        transition: transform .18s ease, color .18s ease;
    }

    input:focus + .label, input:not(:placeholder-shown) + .label {
        transform: translateY(-16px) scale(.76);
        color: var(--accent);
    }

    .options {
        display: flex;
        align-items: center;
        justify-content: space-between;
        font-size: 13px;
    }

    .remember {
        display: flex;
        align-items: center;
        gap: 7px;
        color: #54636d;
        cursor: pointer;
    }

    .remember input {
        width: 15px;
        height: 15px;
        accent-color: var(--accent);
    }
    
    a {
        color: var(--accent);
        font-weight: 600;
        text-decoration: none;
    }

    a:hover{
        text-decoration: underline;
    }

    button {
        height: 52px;
        border: 0;
        border-radius: 12px;
        cursor: pointer;
        background: var(--accent);
        color: #fff;
        font-size: 15px;
        font-weight: 600;
        transition: filter .18s,transform .12s;
    }

    button:hover {
        filter: brightness(1.06);
    }

    button:active {
        transform: scale(.985);
    }

    button:focus-visible {
        outline: 3px solid var(--accent-button);
        outline-offset: 3px;
    }

    .error {
        color: rgb(192, 25, 42);
        font-size: small;
    }

</style>


<Widget>
    <h1>Welcome back!</h1>
    <form onsubmit={handleSubmit}>
        <div class="form-input">
            <input id="email" type="email" bind:value={email} required autocomplete="username" placeholder=" " />
            <label class="label" for="email">Email address</label>
        </div>
        <div class="form-input">
            <input id="password" type="password" bind:value={password} required autocomplete="current-password" placeholder=" " />
            <label class="label" for="password">Password</label>
        </div>
        <div class="options">
            <label class="remember"><input type="checkbox"> Remember me</label>
            <a href="#">Forgot?</a>
        </div>
        <div class="error">
            {#if error}<p>{error}</p>{/if}
        </div>
        <button disabled={loading}>{loading ? 'Signing in…' : 'Sign in'}</button>
    </form>
</Widget>